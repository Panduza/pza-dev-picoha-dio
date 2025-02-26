//! Use your Raspberry Pico board as a GPIO extender
#![no_std]
#![no_main]

// uart debug
mod uart_debug;

use crate::api_dio::PicohaDioRequest;

// application logic
mod api_dio_utils;
mod dio_request_processor;

use dio_request_processor::DioRequestProcessor;

use femtopb::Message;
mod api_dio;

// Used to demonstrate writing formatted strings
use core::fmt::Write;

use serial_line_ip;

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Flex;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use {defmt_rtt as _, panic_probe as _};

use static_cell::StaticCell;

pub const MAX_PINS: usize = 29;

#[embassy_executor::main]
async fn _main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    //print_debug_message!("Firmware Start!");

    let use_boot2 = true;
    let mut id_unique: [u8; 8] = [0; 8];
    static mut SERIAL_ID_STR: heapless::String<32> = heapless::String::<32>::new();

    unsafe {
        // let jedec_id = rp2040_flash::flash::flash_jedec_id(use_boot2);

        //
        rp2040_flash::flash::flash_unique_id(&mut id_unique, use_boot2);
        //print_debug_message!("jedec_id {:?}", jedec_id);

        core::write!(
            SERIAL_ID_STR,
            "PICOHADIO{:X}{:X}{:X}{:X}{:X}{:X}{:X}{:X}",
            id_unique[0],
            id_unique[1],
            id_unique[2],
            id_unique[3],
            id_unique[4],
            id_unique[5],
            id_unique[6],
            id_unique[7]
        )
        .unwrap_or_else(|_| {
            //print_debug_message!("Error fetch serial id");
        });

        //print_debug_message!("Serial ID {:?}", &raw const SERIAL_ID_STR);
    }

    // --------------------------------------------------------------
    // USB CDC
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let config = unsafe {
        let mut config = embassy_usb::Config::new(0x16c0, 0x5e1);
        config.manufacturer = Some("Panduza");
        config.product = Some("PICOHA-DIO");
        config.serial_number = Some(&SERIAL_ID_STR);
        config.max_power = 100;
        config.max_packet_size_0 = 64;
        config
    };

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut usb_builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let usb_builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        usb_builder
    };

    // Create classes on the builder.
    let mut serial = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut usb_builder, state, 64)
    };

    #[cfg(debug_assertions)]
    let serial_debug = {
        static STATE_DEBUG: StaticCell<State> = StaticCell::new();
        let state_debug = STATE_DEBUG.init(State::new());
        let serial_debug = CdcAcmClass::new(&mut usb_builder, state_debug, 64);
        serial_debug
    };
    #[cfg(debug_assertions)]
    let log_fut = embassy_usb_logger::with_class!(1024, log::LevelFilter::Debug, serial_debug);

    // Build the builder.
    let mut usb = usb_builder.build();
    let usb_fut = usb.run();

    // --------------------------------------------------------------

    let mut pins: [Option<Flex>; MAX_PINS] = [
        Some(Flex::new(p.PIN_0)),
        Some(Flex::new(p.PIN_1)),
        Some(Flex::new(p.PIN_2)),
        Some(Flex::new(p.PIN_3)),
        Some(Flex::new(p.PIN_4)),
        Some(Flex::new(p.PIN_5)),
        Some(Flex::new(p.PIN_6)),
        Some(Flex::new(p.PIN_7)),
        Some(Flex::new(p.PIN_8)),
        Some(Flex::new(p.PIN_9)),
        Some(Flex::new(p.PIN_10)),
        Some(Flex::new(p.PIN_11)),
        Some(Flex::new(p.PIN_12)),
        Some(Flex::new(p.PIN_13)),
        Some(Flex::new(p.PIN_14)),
        Some(Flex::new(p.PIN_15)),
        Some(Flex::new(p.PIN_16)),
        Some(Flex::new(p.PIN_17)),
        Some(Flex::new(p.PIN_18)),
        Some(Flex::new(p.PIN_19)),
        Some(Flex::new(p.PIN_20)),
        Some(Flex::new(p.PIN_21)),
        Some(Flex::new(p.PIN_22)),
        None, // 23 Controls the on-board SMPS Power Save pin
        None, // 24 VBUS sense - high if VBUS is present, else low
        None, // 25 Connected to user LED
        Some(Flex::new(p.PIN_26)),
        Some(Flex::new(p.PIN_27)),
        Some(Flex::new(p.PIN_28)),
        // 29 Used in ADC mode (ADC3) to measure VSYS/3
    ];

    // Create the request processor and init all pin to input
    let mut request_processor = DioRequestProcessor::new(&mut pins);

    let mut decode_buffer: serial_line_ip::DecoderBuffer<512> =
        serial_line_ip::DecoderBuffer::new();

    let decode_fut = async {
        let mut buf = [0u8; 512];
        loop {
            serial.wait_connection().await;
            // Check for new data
            if let Ok(count) = serial.read_packet(&mut buf).await {
                let mut data = &buf[..count];
                debug!("========================");
                debug!("+ received: {:?}", data);

                loop {
                    // Check if we have enough data to decode
                    match decode_buffer.feed(data) {
                        Ok((nb_bytes_processed, /*found_trame_complete*/ true)) => {
                            let trame = decode_buffer.slice();

                            if let Ok(request) = PicohaDioRequest::decode(trame) {
                                debug!("+ process request");
                                let _ = request_processor
                                    .process_request(&mut serial, &request)
                                    .await;
                            } else {
                                debug!("      * error decoding request");
                            }

                            decode_buffer.reset();
                            data = &data[nb_bytes_processed..];
                        }
                        other => {
                            debug!("      * error feed data: {:?}", other);
                            break;
                        }
                    }
                }
            }
        }
    };

    #[cfg(debug_assertions)]
    join(usb_fut, join(decode_fut, log_fut)).await;
    #[cfg(not(debug_assertions))]
    join(usb_fut, decode_fut).await;
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

// End of file

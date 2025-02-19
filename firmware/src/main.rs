//! Use your Raspberry Pico board as a GPIO extender
#![no_std]
#![no_main]

// uart debug
mod uart_debug;

#[cfg(any(feature = "uart0_debug"))]
use uart_debug::{uart_debug_init, uart_debug_print, DebugUart};

use crate::api_dio::PicohaDioRequest;

// application logic
mod api_dio_utils;
mod dio_request_processor;

use dio_request_processor::DioRequestProcessor;

use femtopb::{error::DecodeError, Message};
mod api_dio;

// Used to demonstrate writing formatted strings
use core::fmt::Write;

// A shorter alias for the Hardware Abstraction Layer, which provides

use serial_line_ip;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
// use embassy_rp::gpio;
use embassy_rp::gpio::Flex;
use embassy_rp::peripherals::USB;
use embassy_rp::uart;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::UsbDevice;
use {defmt_rtt as _, panic_probe as _};

use static_cell::StaticCell;

pub const MAX_PINS: usize = 29;

#[embassy_executor::main]
async fn _main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Default::default());

    // Init uart debug
    // Set up UART on GP0 and GP1 (Pico pins 1 and 2)
    #[cfg(any(feature = "uart0_debug"))]
    unsafe {
        static mut UART: Option<DebugUart> = None;

        UART = {
            let config = uart::Config::default();
            Some(uart::Uart::new_blocking(p.UART0, p.PIN_0, p.PIN_1, config))
        };
        uart_debug_init(&mut UART);
    }

    print_debug_message!(b"Firmware Start!\r\n");

    let use_boot2 = true;
    let mut id_unique: [u8; 8] = [0; 8];
    static mut SERIAL_ID_STR: heapless::String<32> = heapless::String::<32>::new();

    unsafe {
        let jedec_id = rp2040_flash::flash::flash_jedec_id(use_boot2);

        //
        rp2040_flash::flash::flash_unique_id(&mut id_unique, use_boot2);
        print_debug_message!("jedec_id {:?}\r\n", jedec_id);

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
            print_debug_message!(b"Error fetch serial id");
        });

        print_debug_message!("Serial ID {:?}\r\n", &raw const SERIAL_ID_STR);
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
    let mut builder = {
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    // Create classes on the builder.
    let mut serial = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    // Build the builder.
    let usb = builder.build();

    // Run the USB device.
    unwrap!(spawner.spawn(usb_task(usb)));

    // --------------------------------------------------------------

    #[cfg(not(any(feature = "uart0_debug")))]
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

    #[cfg(any(feature = "uart0_debug"))]
    let mut pins: [Option<Flex>; MAX_PINS] = [
        None,
        None,
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

    loop {
        let mut buf = [0u8; 512];
        serial.wait_connection().await;
        // Check for new data
        if let Ok(count) = serial.read_packet(&mut buf).await {
            let mut data = &buf[..count];
            print_debug_message!(b"========================\r\n");
            print_debug_message!("+ recieved: {:?}", data);

            loop {
                // print_debug_message!(b"1");
                // Check if we have enough data to decode
                match decode_buffer.feed(data) {
                    Ok((nb_bytes_processed, found_trame_complete)) => {
                        // print_debug_message!(b"2");
                        if found_trame_complete {
                            let trame = decode_buffer.slice();
                            if let Ok(request) = decode_api_request(trame) {
                                print_debug_message!("+ process request: {:?}", request);
                                let _ = request_processor
                                    .process_request(&mut serial, &request)
                                    .await;
                                decode_buffer.reset();
                                data = &buf[..count - nb_bytes_processed];
                            }
                        } else {
                            // print_debug_message!(b"3");
                            break;
                        }
                    }
                    other => {
                        print_debug_message!("{:?}", other);
                        break;
                    }
                }
            }
        }
    }
}

/// Decode an API request
///
fn decode_api_request<'a>(frame: &'a [u8]) -> Result<PicohaDioRequest<'a>, DecodeError> {
    PicohaDioRequest::decode(frame)
        .and_then(|ppp| {
            let mut new_request = PicohaDioRequest::default();
            new_request.r#type = ppp.r#type;
            new_request.pin_num = ppp.pin_num;
            new_request.value = ppp.value;
            Ok(new_request)
        })
        .or_else(|e| {
            print_debug_message!("      * error decoding request: {:?}", &e);
            Err(e)
        })
}

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}

// End of file

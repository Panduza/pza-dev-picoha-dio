//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

// uart debug
mod uart_debug;
use uart_debug::uart_debug_init;
use uart_debug::uart_debug_print;

// application logic
mod app_dio;
use app_dio::AppDio;

use bsp::entry;
// use defmt::*;
// use defmt_rtt as _;
use embedded_hal::digital::{InputPin, OutputPin};
use femtopb::Message;
mod api_dio;
// use panic_probe as _;

use fugit::RateExtU32;
use rp2040_hal::{
    pio::PIOExt,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
};
// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;
// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// Used to demonstrate writing formatted strings
use core::fmt::Write;
use heapless::String;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use rp_pico::hal::gpio::{FunctionPio0, Pin};

use serial_line_ip;

#[entry]
unsafe fn main() -> ! {
    // info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let mes = api_dio::PicohaDioRequest {
        r#type: femtopb::EnumValue::Known(api_dio::RequestType::Ping),
        pin_num: 0,
        value: femtopb::EnumValue::Known(api_dio::PinValue::Low),
        unknown_fields: Default::default(),
    };

    // let mut cursor = femtopb::Cursor::new();
    // let rr = mes.encoded_len()

    // let user_factory = NP_Factory::new(
    //     r#"
    //     struct({ fields: {
    //         command: u8({ default: 0 }),
    //         pin: u8({ default: 0 }),
    //         value: u8({ default: 0 }),
    //     }})
    // "#,
    // )?;
    // // close buffer and get internal bytes
    // let user_bytes: Vec<u8> = user_buffer.finish().bytes();

    // let mut slip = serial_line_ip::Encoder::new();
    // let mut totals = slip.encode(INPUT_1, &mut output).unwrap();

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // --------------------------------------------------------------
    // Get pins of the systems
    let pins: rp_pico::Pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // --------------------------------------------------------------
    // Init uart debug
    // Set up UART on GP0 and GP1 (Pico pins 1 and 2)
    let uart_debug_pins = (pins.gpio0.into_function(), pins.gpio1.into_function());
    let uart_debug = UartPeripheral::new(pac.UART0, uart_debug_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    uart_debug_init(uart_debug);
    print_debug_message!(b"Hello World!\r\n");

    // --------------------------------------------------------------

    // // configure LED pin for Pio0.
    // // let led: Pin<_, FunctionPio0, _> = pins.led.into_function();
    // let p0: Pin<_, FunctionPio0, _> = pins.gpio0.into_function();

    // let p1: Pin<_, FunctionPio0, _> = pins.gpio1.into_function(); // data
    //                                                               // PIN id for use inside of PIO

    // let mut piiii = pins.led.into_inout();

    // Use GPIO 28 as an InOutPin
    // let mut pin = pins.led.into_push_pull_output().into_dyn_pin();

    // pin.
    // let _ = pin.set_low();

    // pin.is_high().unwrap();

    // .into_push_pull_output()
    // // .into_floating_input()
    // .into_dyn_pin();

    // piiii.set_output_disable(disable)
    // piiii.set_high().unwrap();

    // let led_pin_id = led.id().num;
    // let led_pin_id = 25;
    // let led_pin_id = 0; // mck
    // let data_pin = 1; // data

    // --------------------------------------------------------------
    // USB CDC
    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    // Set up the USB Communications Class Device driver
    let mut serial: SerialPort<rp2040_hal::usb::UsbBus> = SerialPort::new(&usb_bus);
    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x05E1))
        .strings(&[StringDescriptors::default()
            .manufacturer("panduza")
            .product("picoha-dio")
            .serial_number("TEST")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // --------------------------------------------------------------
    let mut cmd_buf = [0; 20];
    let mut cmd_buf_size = 0;

    let mut said_hello = false;

    let mut app = AppDio::new();
    loop {
        // A welcome message at the beginning
        if !said_hello && timer.get_counter().ticks() >= 2_000_000 {
            said_hello = true;
            let _ = serial.write(b"Hello, World!\r\n");

            let time = timer.get_counter().ticks();
            let mut text: String<64> = String::new();
            writeln!(&mut text, "Current timer ticks: {}", time).unwrap();

            // This only works reliably because the number of bytes written to
            // the serial port is smaller than the buffers available to the USB
            // peripheral. In general, the return value should be handled, so that
            // bytes not transferred yet don't get lost.
            let _ = serial.write(text.as_bytes());
        }

        // Check for new data
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 512];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    app.process_incoming_data(&mut serial, &buf[..count]);
                    // command_buffer[..count].iter().for_each(|b| {

                    // });

                    // cmd_buf[cmd_buf_size..count].clone_from_slice(&buf);

                    // Usage:
                    print_debug_message!("Received {} bytes, {}", count, cmd_buf.len());

                    let oooo = &buf[..count];
                    // writeln!(&mut message, "Received {:?}", oooo).unwrap();
                    // DEBUG_UART
                    //     .as_ref()
                    //     .unwrap()
                    //     .write_full_blocking(message.as_bytes());

                    {
                        let (left, right) = cmd_buf.split_at_mut(cmd_buf_size);

                        // writeln!(&mut message, "left {}, right {}", left.len(), right.len())
                        //     .unwrap();
                        // DEBUG_UART
                        //     .as_ref()
                        //     .unwrap()
                        //     .write_full_blocking(message.as_bytes());

                        right[..count].clone_from_slice(&buf[..count]);

                        cmd_buf_size += count;
                    }

                    // writeln!(&mut message, "total {:?}", cmd_buf).unwrap();
                    // DEBUG_UART
                    //     .as_ref()
                    //     .unwrap()
                    //     .write_full_blocking(message.as_bytes());

                    // cmd_buf_size += count;

                    //
                    let mut slip_decoder = serial_line_ip::Decoder::new();
                    let mut decoded_buffer = [0u8; 30];
                    match slip_decoder.decode(&cmd_buf[..cmd_buf_size], &mut decoded_buffer) {
                        Ok((input_bytes_processed, output_slice, is_end_of_packet)) => {
                            // writeln!(
                            //     &mut message,
                            //     "!!! {:?}, {:?}, {:?}",
                            //     input_bytes_processed, output_slice, is_end_of_packet
                            // )
                            // .unwrap();
                            // DEBUG_UART
                            //     .as_ref()
                            //     .unwrap()
                            //     .write_full_blocking(message.as_bytes());

                            match api_dio::PicohaDioRequest::decode(output_slice) {
                                Ok(ppp) => {
                                    print_debug_message!("deco {:?}", ppp.pin_num);
                                }
                                Err(e) => {
                                    // writeln!(&mut message, "error deco {:?}", e).unwrap();
                                    // DEBUG_UART
                                    //     .as_ref()
                                    //     .unwrap()
                                    //     .write_full_blocking(message.as_bytes());
                                }
                            };
                        }
                        Err(_) => todo!(),
                    };
                }
            }
        }
    }
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_debug_message!(b"Panic!\r\n");
    let line = _info.location().unwrap().line();
    let file = _info.location().unwrap().file();
    print_debug_message!("panic {}:{}", file, line);

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

// End of file

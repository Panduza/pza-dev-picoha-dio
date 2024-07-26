//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

// uart debug
mod uart_debug;
// use rp2040_hal::gpio::new_pin;
use uart_debug::uart_debug_init;
use uart_debug::uart_debug_print;

use crate::{api_dio::PicohaDioAnswer, api_dio::PicohaDioRequest};
// application logic
mod api_dio_utils;
mod dio_request_processor;

use dio_request_processor::DioRequestProcessor;

use bsp::entry;
// use defmt::*;
// use defmt_rtt as _;
// use embedded_hal::digital::{InputPin, OutputPin};
use femtopb::Message;
mod api_dio;
// use panic_probe as _;

use fugit::RateExtU32;
use rp2040_hal::{
    // pio::PIOExt,
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
// use heapless::String;

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
            UartConfig::new(115200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    uart_debug_init(uart_debug);
    print_debug_message!(b"Firmware Start!\r\n");

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
    //
    let pins_id = [
        // Some(pins.gpio0.into_dyn_pin().id()),
        // Some(pins.gpio1.into_dyn_pin().id()),
        None, // 0 debug uart
        None, // 1 debug uart
        Some(pins.gpio2.into_dyn_pin().id()),
        Some(pins.gpio3.into_dyn_pin().id()),
        Some(pins.gpio4.into_dyn_pin().id()),
        Some(pins.gpio5.into_dyn_pin().id()),
        Some(pins.gpio6.into_dyn_pin().id()),
        Some(pins.gpio7.into_dyn_pin().id()),
        Some(pins.gpio8.into_dyn_pin().id()),
        Some(pins.gpio9.into_dyn_pin().id()),
        Some(pins.gpio10.into_dyn_pin().id()),
        Some(pins.gpio11.into_dyn_pin().id()),
        Some(pins.gpio12.into_dyn_pin().id()),
        Some(pins.gpio13.into_dyn_pin().id()),
        Some(pins.gpio14.into_dyn_pin().id()),
        Some(pins.gpio15.into_dyn_pin().id()),
        Some(pins.gpio16.into_dyn_pin().id()),
        Some(pins.gpio17.into_dyn_pin().id()),
        Some(pins.gpio18.into_dyn_pin().id()),
        Some(pins.gpio19.into_dyn_pin().id()),
        Some(pins.gpio20.into_dyn_pin().id()),
        Some(pins.gpio21.into_dyn_pin().id()),
        Some(pins.gpio22.into_dyn_pin().id()),
        None,                               // 23
        None,                               // 24
        Some(pins.led.into_dyn_pin().id()), // 25 led
        None,                               // 26
        None,                               // 27
        Some(pins.gpio28.into_dyn_pin().id()),
        None,
    ];

    // let mut request_buffer = DioRequestBuffer::new();
    let mut decode_buffer: serial_line_ip::DecoderBuffer<512> =
        serial_line_ip::DecoderBuffer::new();
    let mut request_processor = DioRequestProcessor::new(pins_id);
    loop {
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
                    let mut data = &buf[..count];
                    print_debug_message!("+ recieved: {:?}", data);

                    loop {
                        // Check if we have enough data to decode
                        match decode_buffer.feed(data) {
                            core::prelude::v1::Ok((nb_bytes_processed, found_trame_complete)) => {
                                if found_trame_complete {
                                    let trame = decode_buffer.slice();
                                    let request = try_to_decode_api_request(trame).unwrap();
                                    print_debug_message!("+ process request: {:?}", request);
                                    request_processor.process_request(&mut serial, request);
                                    decode_buffer.reset();
                                    data = &buf[..count - nb_bytes_processed];
                                } else {
                                    break;
                                }
                            }
                            _ => {
                                break;
                            }
                        }

                        // Try to parse a request from the buffer
                    }
                }
            }
        }
    }
}

/// Try to decode an API request
///
fn try_to_decode_api_request(frame: &[u8]) -> Option<PicohaDioRequest> {
    match PicohaDioRequest::decode(frame) {
        Ok(ppp) => {
            let mut new_request = PicohaDioRequest::default();
            new_request.r#type = ppp.r#type;
            new_request.pin_num = ppp.pin_num;
            new_request.value = ppp.value;
            Some(new_request)
        }
        Err(e) => {
            print_debug_message!("      * error decoding request: {:?}", e);
            None
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

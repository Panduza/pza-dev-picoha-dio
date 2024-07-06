//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
use femtopb::Message;
use panic_probe as _;

use rp2040_hal::pio::PIOExt;

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

mod api;

use serial_line_ip;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let mes = api::PicohaDioRequest {
        r#type: femtopb::EnumValue::Known(api::RequestType::Ping),
        pin_num: 0,
        value: femtopb::EnumValue::Known(api::PinValue::Low),
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

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // configure LED pin for Pio0.
    let led: Pin<_, FunctionPio0, _> = pins.led.into_function();
    let p0: Pin<_, FunctionPio0, _> = pins.gpio0.into_function();

    let p1: Pin<_, FunctionPio0, _> = pins.gpio1.into_function(); // data
                                                                  // PIN id for use inside of PIO

    // let led_pin_id = led.id().num;
    // let led_pin_id = 25;
    let led_pin_id = 0; // mck
    let data_pin = 1; // data

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x05E1))
        .strings(&[StringDescriptors::default()
            .manufacturer("panduza")
            .product("picoha-dio")
            .serial_number("TEST")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // next step
    // container => https://github.com/mvertescher/serial-line-ip-rs
    // payload => https://crates.io/crates/no_proto

    let mut said_hello = false;
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
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });
                    // Send back to the host
                    let mut wr_ptr = &buf[..count];
                    while !wr_ptr.is_empty() {
                        match serial.write(wr_ptr) {
                            Ok(len) => wr_ptr = &wr_ptr[len..],
                            // On error, just drop unwritten data.
                            // One possible error is Err(WouldBlock), meaning the USB
                            // write buffer is full.
                            Err(_) => break,
                        };
                    }
                }
            }
        }
    }
}

// End of file

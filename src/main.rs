//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

// uart debug
mod uart_debug;
use rp2040_hal::gpio::new_pin;
use uart_debug::uart_debug_init;
use uart_debug::uart_debug_print;

// application logic
mod api_dio_utils;
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
    // let pppp: Pin<
    //     rp2040_hal::gpio::DynPinId,
    //     rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>,
    //     rp2040_hal::gpio::DynPullType,
    // > = pins
    //     .led
    //     .into_push_pull_output()
    //     .into_pull_type()
    //     .into_dyn_pin();
    let p2 = pins
        .gpio3
        .into_floating_input()
        .into_pull_type()
        .into_dyn_pin();

    let did = pins.led.into_push_pull_output().into_dyn_pin().id();
    let pppppp = new_pin(did);

    let mut neerr = pppppp
        .try_into_function::<hal::gpio::FunctionSioOutput>()
        .ok()
        .unwrap();

    // let pppppppppppp = neerr.into_push_pull_output();

    neerr.set_high().unwrap();

    delay.delay_ms(2000u32);

    let pppppp2 = new_pin(did);

    let mut neerr2 = pppppp2
        .try_into_function::<hal::gpio::FunctionSioInput>()
        .ok()
        .unwrap();

    print_debug_message!("Hello World! {}\r\n", neerr2.is_high().unwrap());

    // let dd = p2.reconfigure();

    // let mut pins_array_oooo: [Option<
    //     Pin<
    //         rp2040_hal::gpio::DynPinId,
    //         rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioOutput>,
    //         rp2040_hal::gpio::DynPullType,
    //     >,
    // >; 1] = [Some(pppp)];

    // pins_array_oooo[0].as_mut().unwrap().set_high().unwrap();

    let pins_array: [Pin<
        rp2040_hal::gpio::DynPinId,
        rp2040_hal::gpio::FunctionSio<rp2040_hal::gpio::SioInput>,
        rp2040_hal::gpio::DynPullType,
    >; 1] = [p2];

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
    //
    let mut app = AppDio::new();
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
                    app.process_incoming_data(&mut serial, &buf[..count]);
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

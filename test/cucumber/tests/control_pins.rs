mod libs;
use futures::FutureExt;
use tokio_serial::SerialPort;
use std::{time::{self, Duration}};
use tokio::time::sleep;

use libs::connectors::UsbSettings;

use cucumber::{writer::out::WriteStrExt, World};

use libs::world::PiochaWorld;

#[tokio::main]
async fn main() {
    PiochaWorld::cucumber()
    .init_tracing()
    // .after(|_feature, _rule, _scenario, _ev, _world| {
    //     if let Some(w) = _world {
    //         if w.serial_stream.is_some() {
    //             tracing::info!("Closing serial connection");
    //             w.serial_stream.as_mut().unwrap().clear_line().unwrap();
    //             w.serial_stream = None;
    //         }
    //     }
    //     sleep(Duration::from_millis(1000)).boxed_local()
    // })
    .run("features/control_pins.feature")
    .await;
}

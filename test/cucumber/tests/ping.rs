mod libs;
use libs::connectors::UsbSettings;
use libs::connectors::SerialSettings;

use libs::api_dio::PicohaDioRequest;
use libs::api_dio::RequestType;

use cucumber::{given, when, then, World};
use prost::Message;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
use tokio_serial::SerialStream;


use libs::world::PiochaWorld;

// // These `Cat` definitions would normally be inside your project's code, 
// // not test code, but we create them here for the show case.
// #[derive(Debug, Default)]
// struct  {
//     pub hungry: bool,
// }

// impl Cat {
//     fn feed(&mut self) {
//         self.hungry = false;
//     }
// }


// Steps are defined with `given`, `when` and `then` attributes.
#[given("a serial connection to the device opened")]
async fn open_connection(world: &mut PiochaWorld) {
    // Get the port name
    let port_name = world
        .serial_settings
        .port_name
        .as_ref()
        .expect("Port name is not set in settings");

    // Setup builder
    let serial_builder = tokio_serial::new(port_name, world.serial_settings.baudrate)
        .data_bits(world.serial_settings.data_bits)
        .stop_bits(world.serial_settings.stop_bits)
        .parity(world.serial_settings.parity)
        .flow_control(world.serial_settings.flow_control);

    // Build the stream
    world.serial_stream = Some(
        SerialStream::open(&serial_builder).expect("Failed to open serial port"),
    );
}

#[when("I send a ping command to the device")]
async fn send_ping(world: &mut PiochaWorld) {
    let mut request = PicohaDioRequest::default();
    request.set_type(RequestType::Ping);


    world.write_then_read(&request.encode_to_vec(), &mut [0u8; 1024]).await.unwrap();

    // let serial_stream = world
    //     .serial_stream
    //     .as_mut()
    //     .expect("Serial stream is not set");

    // request.send(serial_stream).await;
}

#[tokio::main]
async fn main() {
    PiochaWorld::run(
        "features/ping.feature",
    ).await;
}
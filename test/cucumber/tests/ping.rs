mod libs;
use libs::api_dio::AnswerType;
use libs::api_dio::PicohaDioAnswer;
use libs::connectors::UsbSettings;

use libs::api_dio::PicohaDioRequest;
use libs::api_dio::RequestType;

use cucumber::{given, then, when, World};
use prost::Message;
use tokio_serial::SerialStream;

use libs::world::PiochaWorld;

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
    world.serial_stream =
        Some(SerialStream::open(&serial_builder).expect("Failed to open serial port"));
}

#[when("I send a ping command to the device")]
async fn send_ping(world: &mut PiochaWorld) {
    let mut request = PicohaDioRequest::default();
    request.set_type(RequestType::Ping);

    let answer_buffer = &mut [0u8; 1024];
    let size = world
        .write_then_read(&request.encode_to_vec(), answer_buffer)
        .await
        .unwrap();

    // Decode the answer
    let answer_slice = answer_buffer[..size].as_ref();
    println!("Received {} bytes -> {:?}", size, answer_slice);
    let answer = PicohaDioAnswer::decode(answer_slice).unwrap();
    world.last_answer = Some(answer);
}

#[then("I must receive a SUCCESS response from the device")]
async fn receive_success(world: &mut PiochaWorld) {
    let answer = world.last_answer.as_ref().unwrap();
    assert_eq!(answer.r#type, AnswerType::Success as i32);
}

#[tokio::main]
async fn main() {
    PiochaWorld::cucumber()
        .init_tracing()
        .run("features/ping.feature")
        .await;
}

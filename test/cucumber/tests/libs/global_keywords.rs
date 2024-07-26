use crate::libs::api_dio::AnswerType;
use crate::libs::api_dio::PicohaDioAnswer;
use crate::libs::api_dio::PinValue;

use crate::libs::api_dio::PicohaDioRequest;
use crate::libs::api_dio::RequestType;

use cucumber::{given, then, when};
use prost::Message;
use tokio_serial::SerialStream;

use crate::libs::world::PiochaWorld;
use rand::Rng;

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a serial connection to the device opened")]
async fn open_connection(world: &mut PiochaWorld) {
    // Debug
    if world.serial_stream.is_some() {
        tracing::info!("Serial connection already opened");
        return;
    }

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

    tracing::info!("Serial open connection on port {}", port_name);

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

#[when(expr = "I send a set_direction {string} in pin {string} command to the device")]
async fn i_send_a_set_direction_in_pin_command_to_the_device(
    world: &mut PiochaWorld,
    direction: String,
    pin: String,
) {
    let mut request = PicohaDioRequest::default();
    request.set_type(RequestType::SetPinDirection);
    request.pin_num = pin.parse().unwrap();

    match direction.as_str() {
        "output" => request.set_value(PinValue::Output),
        "input" => request.set_value(PinValue::Input),
        _ => panic!("Invalid direction value"),
    }

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

#[when(expr = "I send a set_value {string} in pin {string} command to the device")]
async fn i_send_a_set_value_in_pin_command_to_the_device(
    world: &mut PiochaWorld,
    value: String,
    pin: String,
) {
    let mut request = PicohaDioRequest::default();
    request.set_type(RequestType::SetPinValue);
    request.pin_num = pin.parse().unwrap();

    match value.as_str() {
        "high" => request.set_value(PinValue::High),
        "low" => request.set_value(PinValue::Low),
        _ => panic!("Invalid direction value"),
    }

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

#[given(expr = "I send a corrupted data to the device")]
async fn i_send_a_corrupted_data_to_the_device(world: &mut PiochaWorld) {
    let mut data = [0u8; 20];
    let mut rng = rand::thread_rng();

    for i in 0..data.len() {
        data[i] = rng.gen();
    }

    let _size = world.just_write(&data).await.unwrap();
}

#[when(expr = "I wait for 2 seconds")]
async fn wait_2_sec(world: &mut PiochaWorld) {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}


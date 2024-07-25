mod connectors;
use connectors::UsbSettings;
use connectors::SerialSettings;

use cucumber::{given, World};

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

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct PiochaWorld {
    // cat: Cat,
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a serial connection to the device opened")]
async fn open_connection(world: &mut PiochaWorld) {
    // world.cat.hungry = true;
}

#[tokio::main]
async fn main() {

    PiochaWorld::run(
        "features/ping.feature",
    ).await;
}
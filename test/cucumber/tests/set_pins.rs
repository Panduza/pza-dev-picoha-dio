use super::libs::SerialSettings;

use cucumber::{given, World};

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    cat: Cat,
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a serial connection to the device opened")]
fn open_connection(world: &mut AnimalWorld) {
    world.cat.hungry = true;
}


fn main() {
    futures::executor::block_on(AnimalWorld::run(
        "features/ping.feature",
    ));
}
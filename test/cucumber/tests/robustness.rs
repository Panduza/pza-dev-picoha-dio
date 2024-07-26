mod libs;
use libs::connectors::UsbSettings;

use cucumber::World;

use libs::world::PiochaWorld;

#[tokio::main]
async fn main() {
    PiochaWorld::cucumber()
        .init_tracing()
        .run("features/robustness.feature")
        .await;
}

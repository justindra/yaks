mod features;

use cucumber::World as _;
use features::world::World;

#[tokio::test]
async fn run_all_features() {
    World::run("features/").await;
}

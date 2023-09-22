pub mod fixtures;

use cucumber::World;
use fixtures::{before_scenario, DatadogWorld};

#[tokio::main]
async fn main() {
    env_logger::init();

    for api_version in 1..3 {
        println!("[Running v{} BDD Tests]", api_version);
        DatadogWorld::cucumber()
            .before(|_feature, _rule, _scenario, _world| {
                Box::pin(before_scenario(_feature, _rule, _scenario, _world))
            })
            .filter_run(
                format!("tests/scenarios/features/v{}/", api_version),
                |_, _, sc| {
                    sc.tags
                        .iter()
                        .all(|tag| tag != "skip" && tag != "skip-rust")
                },
            )
            .await;
    }
}

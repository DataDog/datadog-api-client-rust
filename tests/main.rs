mod scenarios;

use cucumber::World;
use scenarios::fixtures::{after_scenario, before_scenario, DatadogWorld};

#[tokio::main]
async fn main() {
    env_logger::init();
    DatadogWorld::cucumber()
        .before(|feature, rule, scenario, world| {
            Box::pin(before_scenario(feature, rule, scenario, world))
        })
        .after(|feature, rule, scenario, ev, world| {
            Box::pin(after_scenario(feature, rule, scenario, ev, world))
        })
        .filter_run(format!("tests/scenarios/features/"), |_, _, sc| {
            sc.tags
                .iter()
                .all(|tag| tag != "skip" && tag != "skip-rust")
        })
        .await;
}

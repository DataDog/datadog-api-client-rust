#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod scenarios;

use cucumber::World;
use scenarios::fixtures::{after_scenario, before_scenario, DatadogWorld};

#[tokio::main]
async fn main() {
    env_logger::init();

    for api_version in 1..3 {
        println!("[Testing v{}]", api_version);
        DatadogWorld::cucumber()
            .before(|_feature, _rule, _scenario, world| {
                Box::pin(before_scenario(_feature, _rule, _scenario, world))
            })
            .after(|_feature, _rule, _scenario, _ev, _world| {
                Box::pin(after_scenario(_feature, _rule, _scenario, _ev, _world))
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

mod scenarios;

use cucumber::{cli, parser, runner, writer, World};
use scenarios::fixtures::{after_scenario, before_scenario, DatadogWorld};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    let record_mode = env::var("RECORD")
        .unwrap_or("false".to_string())
        .as_str()
        .to_lowercase();
    let is_replay = !record_mode.eq("true") && !record_mode.eq("none");
    let parsed_cli: cli::Opts<parser::basic::Cli, runner::basic::Cli, writer::basic::Cli> =
        cli::Opts::parsed();
    DatadogWorld::cucumber()
        .with_default_cli()
        .repeat_failed()
        .fail_on_skipped()
        .before(|feature, rule, scenario, world| {
            Box::pin(before_scenario(feature, rule, scenario, world))
        })
        .after(|feature, rule, scenario, ev, world| {
            Box::pin(after_scenario(feature, rule, scenario, ev, world))
        })
        .filter_run("tests/scenarios/features/".to_string(), move |_, _, sc| {
            let name_re = parsed_cli.re_filter.clone();
            let name_match = name_re
                .and_then(|filter| {
                    if filter.is_match(sc.name.as_str()) {
                        Some(true)
                    } else {
                        Some(false)
                    }
                })
                .unwrap_or(true);
            if !name_match {
                return false;
            }
            if sc.tags.contains(&"skip".to_string()) || sc.tags.contains(&"skip-rust".to_string()) {
                return false;
            } else if !is_replay && sc.tags.contains(&"replay-only".to_string()) {
                return false;
            } else if is_replay && sc.tags.contains(&"integration-only".to_string()) {
                return false;
            } else {
                return true;
            }
        })
        .await;
}
// right now it repeats failed steps, eventually write custom writer logic for repeating failed scenarios

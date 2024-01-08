mod scenarios;

use cucumber::{cli, parser, runner, writer, World};
use lazy_static::lazy_static;
use regex::Regex;
use scenarios::fixtures::{after_scenario, before_scenario, DatadogWorld};
use serde_json::Value;
use std::{env, fs::File, io::BufReader};

lazy_static! {
    static ref GIVEN_V1: Value = {
        log::debug!("loading given.json");
        let given_v1_file = File::open("tests/scenarios/features/v1/given.json").unwrap();
        serde_json::from_reader(BufReader::new(given_v1_file))
            .expect("failed to deserialize given.json")
    };
    static ref GIVEN_V2: Value = {
        let given_v2_file = File::open("tests/scenarios/features/v2/given.json").unwrap();
        serde_json::from_reader(BufReader::new(given_v2_file))
            .expect("failed to deserialize given.json")
    };
    static ref UNDO_V1: Value = {
        let undo_v1_file = File::open("tests/scenarios/features/v1/undo.json").unwrap();
        serde_json::from_reader(BufReader::new(undo_v1_file))
            .expect("failed to deserialize undo.json")
    };
    static ref UNDO_V2: Value = {
        let undo_v2_file = File::open("tests/scenarios/features/v2/undo.json").unwrap();
        serde_json::from_reader(BufReader::new(undo_v2_file))
            .expect("failed to deserialize undo.json")
    };
    static ref API_VERSION_RE: Regex = Regex::new(r"tests/scenarios/features/v(\d+)/").unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let record_mode = env::var("RECORD")
        .unwrap_or("false".to_string())
        .to_lowercase();
    let is_replay = !record_mode.eq("true") && !record_mode.eq("none");
    let parsed_cli: cli::Opts<parser::basic::Cli, runner::basic::Cli, writer::basic::Cli> =
        cli::Opts::parsed();

    DatadogWorld::cucumber()
        .with_default_cli()
        .repeat_failed()
        .fail_on_skipped()
        .before(move |feature, rule, scenario, world| {
            world.api_version = API_VERSION_RE
                .captures(feature.path.as_ref().unwrap().to_str().unwrap())
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            if world.api_version == 1 {
                world.given_map = Some(&GIVEN_V1);
                world.undo_map = Some(&UNDO_V1);
            } else if world.api_version == 2 {
                world.given_map = Some(&GIVEN_V2);
                world.undo_map = Some(&UNDO_V2);
            }
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

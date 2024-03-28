mod scenarios;

use cucumber::{cli, parser, runner, writer, StatsWriter, World};
use lazy_static::lazy_static;
use regex::Regex;
use scenarios::fixtures::{
    after_scenario, before_scenario, given_resource_in_system, DatadogWorld,
};
use serde_json::Value;
use std::{collections::HashMap, env, fs::File, io::BufReader};

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Array(ref mut a), Value::Array(ref b)) => {
            a.extend(b.clone());
        }
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

lazy_static! {
    pub static ref GIVEN_MAP: HashMap<String, Value> = {
        let given_v1_file = File::open("tests/scenarios/features/v1/given.json").unwrap();
        let givens_v1: Value = serde_json::from_reader(BufReader::new(given_v1_file))
            .expect("failed to deserialize given.json");
        let given_v2_file = File::open("tests/scenarios/features/v2/given.json").unwrap();
        let given_v2: Value = serde_json::from_reader(BufReader::new(given_v2_file))
            .expect("failed to deserialize given.json");

        HashMap::from([("v1".to_string(), givens_v1), ("v2".to_string(), given_v2)])
    };
    pub static ref UNDO_MAP: Value = {
        let undo_v1_file = File::open("tests/scenarios/features/v1/undo.json").unwrap();
        let mut undos: Value = serde_json::from_reader(BufReader::new(undo_v1_file))
            .expect("failed to deserialize undo.json");
        let undo_v2_file = File::open("tests/scenarios/features/v2/undo.json").unwrap();
        let undo_v2: Value = serde_json::from_reader(BufReader::new(undo_v2_file))
            .expect("failed to deserialize undo.json");
        merge(&mut undos, &undo_v2);
        undos
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
    let concurrent_scenarios = match is_replay {
        true => 64,
        false => 1,
    };
    let parsed_cli: cli::Opts<parser::basic::Cli, runner::basic::Cli, writer::basic::Cli> =
        cli::Opts::parsed();
    let mut cucumber = DatadogWorld::cucumber()
        .with_default_cli()
        .max_concurrent_scenarios(Some(concurrent_scenarios))
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
            Box::pin(before_scenario(feature, rule, scenario, world))
        })
        .after(|feature, rule, scenario, ev, world| {
            Box::pin(after_scenario(feature, rule, scenario, ev, world))
        });

    for (_, values) in GIVEN_MAP.iter() {
        for value in values.as_array().unwrap() {
            cucumber = cucumber.given(
                Regex::new(value.get("step").unwrap().as_str().unwrap()).unwrap(),
                given_resource_in_system,
            );
        }
    }

    if cucumber
        .filter_run("tests/scenarios/features/".to_string(), move |_, _, sc| {
            let name_re = parsed_cli.re_filter.clone();
            let name_match = name_re
                .and_then(|filter| Some(filter.is_match(sc.name.as_str())))
                .unwrap_or(true);
            if !name_match {
                false
            } else if sc.tags.contains(&"skip".into()) || sc.tags.contains(&"skip-rust".into()) {
                false
            } else if !is_replay && sc.tags.contains(&"replay-only".into()) {
                false
            } else if is_replay && sc.tags.contains(&"integration-only".into()) {
                false
            } else {
                true
            }
        })
        .await
        .execution_has_failed()
    {
        std::process::exit(1);
    }
}
// right now it repeats failed steps, eventually write custom writer logic for repeating failed scenarios

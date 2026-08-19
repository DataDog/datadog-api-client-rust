mod scenarios;

use cucumber::{cli, parser, runner, writer, StatsWriter, World};
use lazy_static::lazy_static;
use regex::Regex;
use scenarios::fixtures::{
    after_scenario, before_scenario, given_resource_in_system, DatadogWorld,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    io::BufReader,
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};

const GENERATED_TEST_ROOT: &str = "tests/scenarios/generated-test";
const GENERATED_TEST_PORT: &str = "18086";

struct GeneratedTestServer {
    child: Option<Child>,
}

fn generated_tests_enabled() -> bool {
    env::var("DD_USE_GENERATED_TESTS")
        .unwrap_or_else(|_| "false".to_string())
        .eq_ignore_ascii_case("true")
}

impl GeneratedTestServer {
    async fn start(record_mode: &str) -> Self {
        let server = PathBuf::from(GENERATED_TEST_ROOT).join("test-server");
        if !generated_tests_enabled() || record_mode != "false" || !server.is_file() {
            return Self { child: None };
        }

        if env::var("DD_TEST_RUNNER_DATA").is_err() {
            env::set_var(
                "DD_TEST_RUNNER_DATA",
                PathBuf::from(GENERATED_TEST_ROOT).join("test-runner-data"),
            );
        }
        if env::var("DD_TEST_SERVER_URL").is_ok() {
            return Self { child: None };
        }

        let port =
            env::var("DD_TEST_SERVER_PORT").unwrap_or_else(|_| GENERATED_TEST_PORT.to_string());
        let url = format!("http://127.0.0.1:{port}");
        env::set_var("DD_TEST_SERVER_URL", &url);
        let log_path = env::var("DD_TEST_SERVER_LOG")
            .unwrap_or_else(|_| "/tmp/datadog-rust-test-server.log".to_string());
        let log = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&log_path)
            .expect("failed to open generated test server log");
        let child = Command::new(server)
            .arg("--port")
            .arg(&port)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .spawn()
            .expect("failed to start generated test server");
        let mut guard = Self { child: Some(child) };
        let health = format!("{url}/__openapi_transformer__/health");
        for _ in 0..50 {
            if let Ok(response) = reqwest::get(&health).await {
                if response.status().is_success() {
                    return guard;
                }
            }
            if guard.child.as_mut().unwrap().try_wait().unwrap().is_some() {
                panic!("generated test server exited early; see {log_path}");
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        drop(guard);
        panic!("generated test server failed to start; see {log_path}");
    }
}

impl Drop for GeneratedTestServer {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn feature_data_path(version: &str, filename: &str) -> PathBuf {
    PathBuf::from("tests/scenarios/features")
        .join(version)
        .join(filename)
}

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
        let given_v1_file = File::open(feature_data_path("v1", "given.json")).unwrap();
        let givens_v1: Value = serde_json::from_reader(BufReader::new(given_v1_file))
            .expect("failed to deserialize given.json");
        let given_v2_file = File::open(feature_data_path("v2", "given.json")).unwrap();
        let given_v2: Value = serde_json::from_reader(BufReader::new(given_v2_file))
            .expect("failed to deserialize given.json");

        HashMap::from([("v1".to_string(), givens_v1), ("v2".to_string(), given_v2)])
    };
    pub static ref UNDO_MAP: Value = {
        let undo_v1_file = File::open(feature_data_path("v1", "undo.json")).unwrap();
        let mut undos: Value = serde_json::from_reader(BufReader::new(undo_v1_file))
            .expect("failed to deserialize undo.json");
        let undo_v2_file = File::open(feature_data_path("v2", "undo.json")).unwrap();
        let undo_v2: Value = serde_json::from_reader(BufReader::new(undo_v2_file))
            .expect("failed to deserialize undo.json");
        merge(&mut undos, &undo_v2);
        undos
    };
    static ref API_VERSION_RE: Regex = Regex::new(r"/v(\d+)/").unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let record_mode = env::var("RECORD")
        .unwrap_or("false".to_string())
        .to_lowercase();
    let generated_test_server = GeneratedTestServer::start(&record_mode).await;
    if generated_tests_enabled() && env::var("DD_TEST_SERVER_URL").is_ok() {
        println!("=== Using Generated Test Runner ===");
    }
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

    let failed = cucumber
        .filter_run("tests/scenarios/features/", move |_, _, sc| {
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
        .execution_has_failed();
    drop(generated_test_server);
    if failed {
        std::process::exit(1);
    }
}
// right now it repeats failed steps, eventually write custom writer logic for repeating failed scenarios

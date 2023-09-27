use crate::scenarios::function_mappings::*;
use cucumber::{
    event::ScenarioFinished, gherkin::Feature, gherkin::Rule, gherkin::Scenario, given, then, when,
    World,
};
use datadog_api_client::datadog::configuration::Configuration;
use handlebars::Handlebars;
use regex::Regex;
use serde_json::json;
use sha256::digest;
use std::{collections::HashMap, fs::{File, read_to_string}, io::BufReader, time::SystemTime};


#[derive(Debug, Default)]
pub struct Response {
    pub object: serde_json::Value,
    pub code: u16,
    pub err: Option<serde_json::Value>,
}

#[derive(Debug, Default, World)]
pub struct DatadogWorld {
    pub config: Configuration,
    pub api_version: i32,
    pub fixtures: serde_json::Value,
    pub operation_id: String,
    pub body: serde_json::Value,
    pub response: Response,
    pub parameters: HashMap<String, String>,
    pub function_mappings: HashMap<String, TestCall>,
    pub given_map: serde_json::Value,
    pub undo_map: serde_json::Value,
}

pub async fn before_scenario(
    feature: &Feature,
    _rule: Option<&Rule>,
    scenario: &Scenario,
    world: &mut DatadogWorld,
) {
    GetTestCalls(world);
    let api_version_re = Regex::new(r"tests/scenarios/features/v(\d+)/").unwrap();
    // TODO: refactor this lol
    world.api_version = api_version_re
        .captures(feature.path.as_ref().unwrap().to_str().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let given_file = File::open(format!(
        "tests/scenarios/features/v{}/given.json",
        world.api_version
    ))
    .unwrap();
    world.given_map = serde_json::from_reader(BufReader::new(given_file)).unwrap();
    let undo_file = File::open(format!(
        "tests/scenarios/features/v{}/undo.json",
        world.api_version
    ))
    .unwrap();
    world.undo_map = serde_json::from_reader(BufReader::new(undo_file)).unwrap();
    let mut config = Configuration::new();
    config.api_key_auth = Some("00000000000000000000000000000000".to_string());
    config.app_key_auth = Some("0000000000000000000000000000000000000000".to_string());
    world.config = config;

    let non_alnum_re = Regex::new(r"[^A-Za-z0-9]+").unwrap();
    let prefix = match true {
        true => "Test-Rust",
        false => "Test",
    };
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let escaped_name = non_alnum_re
        .replace_all(scenario.name.as_str(), "_")
        .to_string();
    let name = match escaped_name.len() > 100 {
        true => escaped_name[..100].to_string(),
        false => escaped_name,
    };
    let unique = format!("{}-{}-{}", prefix, name, now);
    let unique_alnum = non_alnum_re.replace_all(unique.as_str(), "").to_string();
    world.fixtures = json!({
        "unique": unique,
        "unique_lower": unique.to_ascii_lowercase(),
        "unique_upper": unique.to_ascii_uppercase(),
        "unique_alnum": unique_alnum,
        "unique_lower_alnum": unique_alnum.to_ascii_lowercase(),
        "unique_upper_alnum": unique_alnum.to_ascii_uppercase(),
        "unique_hash": digest(unique)[..16],
        "now": now,
    });
}

pub async fn after_scenario(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _ev: &ScenarioFinished,
    _world: Option<&mut DatadogWorld>,
) {
}

#[given(expr = "a valid \"apiKeyAuth\" key in the system")]
fn a_valid_apikey_auth(world: &mut DatadogWorld) {
    world.config.api_key_auth = std::env::var("DD_TEST_CLIENT_API_KEY").ok();
}

#[given(expr = "a valid \"appKeyAuth\" key in the system")]
fn a_valid_appkey_auth(world: &mut DatadogWorld) {
    world.config.app_key_auth = std::env::var("DD_TEST_CLIENT_APP_KEY").ok();
}

#[given(expr = "an instance of {string} API")]
fn an_instance_of_api(_world: &mut DatadogWorld, _api: String) {}

#[given(expr = "new {string} request")]
fn a_new_request(world: &mut DatadogWorld, operation_id: String) {
    world.operation_id = operation_id
}

#[given(regex = r"^body with value (.*)$")]
fn body_with_value(world: &mut DatadogWorld, body: String) {
    let rendered = template(body, world.fixtures.clone());
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.body = body_struct;
}

#[given(regex = r"^body from file (.*)$")]
fn body_from_file(world: &mut DatadogWorld, path: String) {
    let body = read_to_string(format!(
        "tests/scenarios/features/v{}/{}",
        world.api_version,
        path,
    )).unwrap();
    let rendered = template(body, world.fixtures.clone());
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.body = body_struct;
}

#[given(expr = "request contains {string} parameter from {string}")]
fn request_parameter_from_path(world: &mut DatadogWorld, param: String, path: String) {
    let lookup = lookup(path, world.response.object.clone()).unwrap();
    world.parameters.insert(param, lookup.to_string());
}

#[given(expr = "request contains {string} parameter with value {}")]
fn request_parameter_with_value(world: &mut DatadogWorld, param: String, value: String) {
    let rendered = template(value, world.fixtures.clone());
    world.parameters.insert(param, rendered);
}

#[when(regex = r"^the request is sent$")]
fn when_request_sent(world: &mut DatadogWorld) {
    world.function_mappings.get(&world.operation_id).unwrap()(world);
}

#[then(expr = "the response status is {int} {}")]
fn responseStatusIs(world: &mut DatadogWorld, status_code: u16, _status_message: String) {
    assert!(world.response.code == status_code)
}

#[then(expr = "the response {string} is equal to {}")]
fn responseEqualTo(world: &mut DatadogWorld, path: String, value: String) {
    let lookup = lookup(path, world.response.object.clone()).unwrap();
    let rendered_value = template(value, world.fixtures.clone());
    let expected: serde_json::Value = serde_json::from_str(rendered_value.as_str()).unwrap();
    assert_eq!(lookup, expected);
}

#[then(expr = "the response {string} has length {int}")]
fn responseHasLength(world: &mut DatadogWorld, path: String, expected_len: usize) {
    let len = lookup(path, world.response.object.clone())
        .unwrap()
        .as_array()
        .unwrap()
        .len();
    assert_eq!(len, expected_len);
}

fn lookup(path: String, object: serde_json::Value) -> Option<serde_json::Value> {
    let index_re = Regex::new(r"\[(\d+)\]+").unwrap();
    let mut json_pointer = format!("/{}", path).replace(".", "/");
    for (_, [idx]) in index_re
        .captures_iter(&json_pointer.clone())
        .map(|c| c.extract())
    {
        json_pointer = index_re
            .replace(&json_pointer, format!("/{idx}"))
            .to_string();
    }

    return object.pointer(&json_pointer).cloned();
}

fn template(string: String, fixtures: serde_json::Value) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(string.as_str(), &fixtures)
        .unwrap()
}

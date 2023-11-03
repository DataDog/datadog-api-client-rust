use crate::scenarios::function_mappings::{
    collect_function_calls, initialize_api_instance, ApiInstances,
};
use chrono::DateTime;
use cucumber::{
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
    given, then, when, World,
};
use datadog_api_client::datadog::configuration::Configuration;
use handlebars::Handlebars;
use log::debug;
use regex::Regex;
use reqwest_middleware::ClientBuilder;
use rvcr::{VCRMiddleware, VCRMode};
use serde_json::{json, Value};
use sha256::digest;
use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, read_to_string, File},
    io::BufReader,
    path::PathBuf,
    time::SystemTime,
};

pub type TestCall = fn(&mut DatadogWorld, &HashMap<String, Value>);

#[derive(Debug, Default)]
pub struct Response {
    pub object: Value,
    pub code: u16,
    pub err: Option<Value>,
}

#[derive(Debug, Clone)]
struct UndoOperation {
    operation_id: String,
    parameters: HashMap<String, Value>,
}

#[derive(Debug, Default, World)]
pub struct DatadogWorld {
    pub api_version: i32,
    pub config: Configuration,
    pub fixtures: Value,
    pub function_mappings: HashMap<String, TestCall>,
    pub operation_id: String,
    pub parameters: HashMap<String, Value>,
    pub response: Response,
    pub api_instances: ApiInstances,
    given_map: Value,
    undo_map: Value,
    undo_operations: Vec<UndoOperation>,
}

pub async fn before_scenario(
    feature: &Feature,
    _rule: Option<&Rule>,
    scenario: &Scenario,
    world: &mut DatadogWorld,
) {
    let api_version_re =
        Regex::new(r"tests/scenarios/features/v(\d+)/").expect("api version regex failed");
    // TODO: refactor this lol
    world.api_version = api_version_re
        .captures(feature.path.as_ref().unwrap().to_str().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    collect_function_calls(world);
    let given_file = File::open(format!(
        "tests/scenarios/features/v{}/given.json",
        world.api_version
    ))
    .expect("failed to open given.json file");
    world.given_map = serde_json::from_reader(BufReader::new(given_file))
        .expect("failed to deserialize given.json");
    let undo_file = File::open(format!(
        "tests/scenarios/features/v{}/undo.json",
        world.api_version
    ))
    .expect("failed to open undo.json file");
    world.undo_map = serde_json::from_reader(BufReader::new(undo_file))
        .expect("failed to deserialize undo.json");

    let non_alnum_re = Regex::new(r"[^A-Za-z0-9]+").expect("non alnum regex failed");
    let escaped_filename = non_alnum_re
        .replace_all(scenario.name.as_str(), "-")
        .to_string();
    let filename = match escaped_filename.len() > 100 {
        true => escaped_filename[..100].to_string(),
        false => escaped_filename,
    };
    let mut prefix = "Test".to_string();
    let mut cassette_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cassette_dir.push(format!(
        "tests/scenarios/cassettes/v{}/{}",
        world.api_version,
        feature.name.replace(' ', "_").to_lowercase()
    ));
    create_dir_all(&cassette_dir).expect("failed to create cassette directory");
    let mut cassette = cassette_dir.clone();
    cassette.push(format!("{}.json", filename));
    let mut freeze = cassette_dir.clone();
    freeze.push(format!("{}.frozen", filename));
    let mut config = Configuration::new();
    let mut frozen_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let vcr_client_builder = ClientBuilder::new(reqwest::Client::new());
    config.client = match env::var("RECORD").unwrap_or("false".to_string()).as_str() {
        "none" => {
            prefix.push_str("-Rust");
            vcr_client_builder.build()
        }
        "true" => {
            // let _ = remove_file(cassette.clone());
            // let _ = remove_file(freeze.clone());
            // let mut freeze_file = File::create(freeze).expect("failed to write freeze file");
            // freeze_file
            //     .write_all(
            //         DateTime::to_rfc3339(
            //             &DateTime::from_timestamp(frozen_time as i64, 0)
            //                 .expect("failed to convert timestamp to datetime"),
            //         )
            //         .as_bytes(),
            //     )
            //     .expect("failed to write freeze file");
            // let middleware: VCRMiddleware = VCRMiddleware::try_from(cassette)
            //     .expect("Failed to initialize rVCR middleware")
            //     .with_mode(VCRMode::Record)
            //     .with_modify_request(|req| {
            //         req.headers.remove_entry("dd-api-key");
            //         req.headers.remove_entry("dd-application-key");
            //     })
            //     .with_modify_response(|res| {
            //         res.headers.remove_entry("content-security-policy");
            //     });
            // vcr_client_builder.with(middleware).build()
            panic!("sdk's shouldn't be recording, that's the spec repo's job.");
        }
        _ => {
            frozen_time = DateTime::parse_from_rfc3339(
                read_to_string(freeze)
                    .expect("Failed to read freeze file")
                    .as_str(),
            )
            .expect("Failed to parse freeze file time")
            .signed_duration_since(DateTime::UNIX_EPOCH)
            .num_seconds() as u64;
            debug!("{}", frozen_time);
            let middleware: VCRMiddleware = VCRMiddleware::try_from(cassette)
                .expect("Failed to initialize rVCR middleware")
                .with_mode(VCRMode::Replay)
                .with_modify_request(|req| {
                    req.headers.remove_entry("dd-api-key");
                    req.headers.remove_entry("dd-application-key");
                })
                .with_modify_response(|res| {
                    res.headers.remove_entry("content-security-policy");
                })
                .with_request_matcher(|vcr_req, req| {
                    vcr_req.uri.to_string() == req.uri.to_string()
                        && vcr_req.body.string == req.body.string
                        && vcr_req.method == req.method
                });
            vcr_client_builder.with(middleware).build()
        }
    };
    config.api_key_auth = Some("00000000000000000000000000000000".to_string());
    config.app_key_auth = Some("0000000000000000000000000000000000000000".to_string());
    world.config = config;

    let escaped_name = non_alnum_re
        .replace_all(scenario.name.as_str(), "_")
        .to_string();
    let name = match escaped_name.len() > 100 {
        true => escaped_name[..100].to_string(),
        false => escaped_name,
    };
    let unique = format!("{}-{}-{}", prefix, name, frozen_time);
    let unique_alnum = non_alnum_re.replace_all(unique.as_str(), "").to_string();
    world.fixtures = json!({
        "unique": unique,
        "unique_lowe: Stringr": unique.to_ascii_lowercase(),
        "unique_upper": unique.to_ascii_uppercase(),
        "unique_alnum": unique_alnum,
        "unique_lower_alnum": unique_alnum.to_ascii_lowercase(),
        "unique_upper_alnum": unique_alnum.to_ascii_uppercase(),
        "unique_hash": digest(unique)[..16],
        "now": frozen_time,
    });
}

pub async fn after_scenario(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _ev: &ScenarioFinished,
    world: Option<&mut DatadogWorld>,
) {
    if let Some(world) = world {
        for undo in world.undo_operations.clone().iter().rev() {
            world
                .function_mappings
                .get(&undo.operation_id)
                .expect("undo operation not found")(world, &undo.parameters);
        }
    }
}

#[given(expr = "a valid \"apiKeyAuth\" key in the system")]
fn valid_apikey_auth(world: &mut DatadogWorld) {
    world.config.api_key_auth = env::var("DD_TEST_CLIENT_API_KEY").ok();
}

#[given(expr = "a valid \"appKeyAuth\" key in the system")]
fn valid_appkey_auth(world: &mut DatadogWorld) {
    world.config.app_key_auth = env::var("DD_TEST_CLIENT_APP_KEY").ok();
}

#[given(expr = "an instance of {string} API")]
fn instance_of_api(world: &mut DatadogWorld, api: String) {
    initialize_api_instance(world, api);
}

#[given(expr = "there is a valid {string} in the system")]
fn given_resource_in_system(world: &mut DatadogWorld, given_key: String) {
    let given = world
        .given_map
        .as_array()
        .unwrap()
        .iter()
        .find(|value| value.get("key").unwrap().as_str().unwrap() == given_key)
        .unwrap()
        .clone();
    let mut given_parameters: HashMap<String, Value> = HashMap::new();
    if let Some(params) = given.get("parameters") {
        for param in params.as_array().unwrap() {
            let param_name = param.get("name").unwrap().as_str().unwrap().to_string();
            if let Some(source) = param.get("source") {
                if let Some(value) = lookup(&source.as_str().unwrap().to_string(), &world.fixtures)
                {
                    given_parameters.insert(param_name.clone(), value);
                }
            };
            if let Some(template_value) = param.get("value") {
                let mut rendered = template(template_value.to_string(), &world.fixtures);
                rendered = serde_json::from_str(rendered.as_str()).unwrap();
                given_parameters.insert(
                    param_name.clone(),
                    serde_json::from_str(rendered.as_str()).unwrap(),
                );
            };
        }
    }

    if let Some(tag) = given.get("tag") {
        let mut api_name = tag
            .as_str()
            .expect("failed to parse given tag as str")
            .to_string();
        api_name.retain(|c| !c.is_whitespace());
        initialize_api_instance(world, api_name);
    }

    let operation_id = given
        .get("operationId")
        .expect("operationId missing from given")
        .as_str()
        .expect("failed to parse given operation id as str")
        .to_string();

    world
        .function_mappings
        .get(&operation_id)
        .expect("given operation not found")(world, &given_parameters);
    match build_undo(world, &operation_id) {
        Ok(Some(undo)) => world.undo_operations.push(undo),
        Ok(None) => {}
        Err(err) => panic!("{err}"),
    }
    if let Some(source) = given.get("source") {
        let source_path = source.as_str().unwrap().to_string();
        if let Some(fixture) = lookup(&source_path, &world.response.object) {
            if let Value::Object(ref mut map) = world.fixtures {
                map.insert(given_key, fixture);
            }
        }
    } else if let Value::Object(ref mut map) = world.fixtures {
        map.insert(given_key, world.response.object.clone());
    }
}

#[given(expr = "new {string} request")]
fn new_request(world: &mut DatadogWorld, operation_id: String) {
    world.operation_id = operation_id
}

#[given(regex = r"^body with value (.*)$")]
fn body_with_value(world: &mut DatadogWorld, body: String) {
    let rendered = template(body, &world.fixtures);
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.parameters.insert("body".to_string(), body_struct);
}

#[given(regex = r"^body from file (.*)$")]
fn body_from_file(world: &mut DatadogWorld, path: String) {
    let body = read_to_string(format!(
        "tests/scenarios/features/v{}/{}",
        world.api_version, path
    ))
    .unwrap();
    let rendered = template(body, &world.fixtures);
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.parameters.insert("body".to_string(), body_struct);
}

#[given(expr = "request contains {string} parameter from {string}")]
fn request_parameter_from_path(world: &mut DatadogWorld, param: String, path: String) {
    let lookup = lookup(&path, &world.fixtures).unwrap();
    world.parameters.insert(param, lookup);
}

#[given(expr = "request contains {string} parameter with value {}")]
fn request_parameter_with_value(world: &mut DatadogWorld, param: String, value: String) {
    let rendered = template(value, &world.fixtures);
    world.parameters.insert(param, Value::from(rendered));
}

#[when(regex = r"^the request is sent$")]
fn request_sent(world: &mut DatadogWorld) {
    world
        .function_mappings
        .get(&world.operation_id)
        .expect("request operation not found")(world, &world.parameters.clone());
    match build_undo(world, &world.operation_id.clone()) {
        Ok(Some(undo)) => {
            world.undo_operations.push(undo);
        }
        Ok(None) => {}
        Err(err) => panic!("{err}"),
    }
}

#[then(expr = "the response status is {int} {}")]
fn response_status_is(world: &mut DatadogWorld, status_code: u16, _status_message: String) {
    assert!(world.response.code == status_code)
}

#[then(expr = "the response {string} is equal to {}")]
fn response_equal_to(world: &mut DatadogWorld, path: String, value: String) {
    let lookup = lookup(&path, &world.response.object).expect("value not found in response");
    let rendered_value = template(value, &world.fixtures);
    let expected: Value = serde_json::from_str(rendered_value.as_str()).unwrap();
    assert_eq!(lookup, expected);
}

#[then(expr = "the response {string} has length {int}")]
fn response_has_length(world: &mut DatadogWorld, path: String, expected_len: usize) {
    let len = lookup(&path, &world.response.object)
        .unwrap()
        .as_array()
        .unwrap()
        .len();
    assert_eq!(len, expected_len);
}

fn lookup(path: &String, object: &Value) -> Option<Value> {
    let index_re = Regex::new(r"\[(\d+)\]+").expect("index regex failed");
    let mut json_pointer = format!("/{}", path).replace('.', "/");
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

fn template(string: String, fixtures: &Value) -> String {
    Handlebars::new()
        .render_template(string.as_str(), &fixtures)
        .expect("failed to apply template")
}

fn build_undo(
    world: &mut DatadogWorld,
    operation_id: &String,
) -> Result<Option<UndoOperation>, Value> {
    let undo = world
        .undo_map
        .get(operation_id)
        .unwrap()
        .get("undo")
        .unwrap();
    match undo.get("type").unwrap().as_str() {
        Some("unsafe") => {
            let mut undo_operation = UndoOperation {
                operation_id: undo
                    .get("operationId")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                parameters: HashMap::new(),
            };
            let params = undo.get("parameters").unwrap().as_array().unwrap();
            for param in params {
                let param_name = param.get("name").unwrap().as_str().unwrap().to_string();
                if let Some(source) = param.get("source") {
                    if let Some(value) = lookup(
                        &source.as_str().unwrap().to_string(),
                        &world.response.object,
                    ) {
                        undo_operation.parameters.insert(param_name.clone(), value);
                    }
                };
                if let Some(template_value) = param.get("template") {
                    if let Some(rendered) = template_value.as_str() {
                        undo_operation.parameters.insert(
                            param_name.clone(),
                            template(rendered.to_string(), &world.fixtures).into(),
                        );
                    };
                };
            }
            Ok(Some(undo_operation))
        }
        _ => Ok(None),
    }
}

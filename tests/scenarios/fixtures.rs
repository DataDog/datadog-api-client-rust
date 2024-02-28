use crate::{
    scenarios::function_mappings::{collect_function_calls, initialize_api_instance, ApiInstances},
    GIVEN_MAP, UNDO_MAP,
};
use chrono::{DateTime, Duration, Months, SecondsFormat};
use convert_case::{Case, Casing};
use cucumber::{
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
    given, then, when, World,
};
use datadog_api_client::datadog::configuration::{APIKey, Configuration};
use lazy_static::lazy_static;
use minijinja::{Environment, State};
use regex::Regex;
use reqwest_middleware::ClientBuilder;
use rvcr::{VCRMiddleware, VCRMode};
use serde_json::{json, Value};
use sha256::digest;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{create_dir_all, read_to_string},
    ops::Add,
    path::PathBuf,
    str::FromStr,
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
    tag: Option<String>,
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
    pub api_name: Option<String>,
    pub api_instances: ApiInstances,
    undo_operations: Vec<UndoOperation>,
}

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"^\d+$").unwrap();
    static ref BOOL_RE: Regex = Regex::new(r"^(true|false)$").unwrap();
    static ref ARRAY_RE: Regex = Regex::new(r"^\[.*\]$").unwrap();
    static ref INDEX_RE: Regex = Regex::new(r"\[(\d+)\]+").unwrap();
    static ref NON_ALNUM_RE: Regex = Regex::new(r"[^A-Za-z0-9]+").unwrap();
    static ref TIME_FMT_HELPER_RE: Regex =
        Regex::new(r"now(?: *([+-]) *(\d+)([smhdMy]))?").unwrap();
    static ref TEMPLATE_ENV: Environment<'static> = {
        let mut env = Environment::new();
        env.add_function("timestamp", timestamp_helper);
        env.add_function("timeISO", time_iso_helper);
        env
    };
}

pub async fn before_scenario(
    feature: &Feature,
    _rule: Option<&Rule>,
    scenario: &Scenario,
    world: &mut DatadogWorld,
) {
    collect_function_calls(world);

    let filename = NON_ALNUM_RE
        .replace_all(scenario.name.as_str(), "-")
        .to_string();

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

    let mut frozen_time = chrono::Utc::now().signed_duration_since(DateTime::UNIX_EPOCH);

    let vcr_client_builder = ClientBuilder::new(reqwest::Client::new());
    let client = match env::var("RECORD").unwrap_or("false".to_string()).as_str() {
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
            //             &DateTime::from_timestamp(frozen_time.num_milliseconds() as i64, 0)
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
            panic!("Use the cassette transform in datadog-api-spec instead. This recording mode is only here for completeness.");
        }
        _ => {
            frozen_time = DateTime::parse_from_rfc3339(
                read_to_string(freeze)
                    .expect("Failed to read freeze file")
                    .as_str(),
            )
            .expect("Failed to parse freeze file time")
            .signed_duration_since(DateTime::UNIX_EPOCH);

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
                .with_request_matcher(|vcr_req, req| req_eq(vcr_req, req));
            vcr_client_builder.with(middleware).build()
        }
    };
    world.config.client(client);
    world.config.set_auth_key(
        "apiKeyAuth",
        APIKey {
            key: "00000000000000000000000000000000".to_string(),
            prefix: "".to_owned(),
        },
    );
    world.config.set_auth_key(
        "appKeyAuth",
        APIKey {
            key: "0000000000000000000000000000000000000000".to_string(),
            prefix: "".to_owned(),
        },
    );

    let escaped_name = NON_ALNUM_RE
        .replace_all(scenario.name.as_str(), "_")
        .to_string();
    let name = match escaped_name.len() > 100 {
        true => escaped_name[..100].to_string(),
        false => escaped_name,
    };
    let unique = format!("{}-{}-{}", prefix, name, frozen_time.num_seconds());
    let unique_alnum = NON_ALNUM_RE.replace_all(unique.as_str(), "").to_string();
    world.fixtures = json!({
        "unique": unique,
        "unique_lower": unique.to_ascii_lowercase(),
        "unique_upper": unique.to_ascii_uppercase(),
        "unique_alnum": unique_alnum,
        "unique_lower_alnum": unique_alnum.to_ascii_lowercase(),
        "unique_upper_alnum": unique_alnum.to_ascii_uppercase(),
        "unique_hash": digest(unique)[..16],
        "now": frozen_time.num_seconds(),
        "now_millis": frozen_time.num_milliseconds(),
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
            if undo.tag.is_some() {
                initialize_api_instance(world, undo.tag.clone().unwrap());
            }
            let test_call = world
                .function_mappings
                .get(&format!("v{}.{}", world.api_version, &undo.operation_id))
                .unwrap_or_else(|| {
                    let alt_version = match world.api_version {
                        1 => 2,
                        2 => 1,
                        _ => panic!("invalid api version"),
                    };
                    world
                        .function_mappings
                        .get(&format!("v{}.{}", alt_version, &undo.operation_id))
                        .expect("undo operation not found")
                });
            test_call(world, &undo.parameters);
        }
    }
}

#[given(expr = "a valid \"apiKeyAuth\" key in the system")]
fn valid_apikey(world: &mut DatadogWorld) {
    world.config.set_auth_key(
        "apiKeyAuth",
        APIKey {
            key: env::var("DD_TEST_CLIENT_API_KEY").unwrap_or_default(),
            prefix: "".to_owned(),
        },
    );
    if let Some(api) = world.api_name.as_ref() {
        initialize_api_instance(world, api.to_string());
    }
}

#[given(expr = "a valid \"appKeyAuth\" key in the system")]
fn valid_appkey(world: &mut DatadogWorld) {
    world.config.set_auth_key(
        "appKeyAuth",
        APIKey {
            key: env::var("DD_TEST_CLIENT_APP_KEY").unwrap_or_default(),
            prefix: "".to_owned(),
        },
    );
    if let Some(api) = world.api_name.as_ref() {
        initialize_api_instance(world, api.to_string());
    }
}

#[given(expr = "an instance of {string} API")]
fn instance_of_api(world: &mut DatadogWorld, api: String) {
    initialize_api_instance(world, api.clone());
    world.api_name = Some(api);
}

pub fn given_resource_in_system(
    world: &mut DatadogWorld,
    context: cucumber::step::Context,
) -> std::pin::Pin<Box<dyn futures::Future<Output = ()> + '_>> {

    let mut given: Value = Value::Null;
    let mut given_api_version: String = "".to_string();
    let mut found = false;
    for (version, values) in GIVEN_MAP.iter() {
        for value in values.as_array().unwrap() {
            if value.get("step").unwrap().as_str().unwrap() == context.step.value {
                given = value.clone();
                given_api_version = version.clone();
                found = true;
                break;
            };
        }
        if found {
            break;
        }
    }

    if !found {
        panic!("given step not found");
    }
    
    let given_key = given.get("key").unwrap().as_str().unwrap().to_string();
    Box::pin(async move {
        let mut given_parameters: HashMap<String, Value> = HashMap::new();
        if let Some(params) = given.get("parameters") {
            for param in params.as_array().unwrap() {
                let param_name = param.get("name").unwrap().as_str().unwrap().to_string();
                if let Some(source) = param.get("source") {
                    if let Some(value) =
                        lookup(&source.as_str().unwrap().to_string(), &world.fixtures)
                    {
                        given_parameters.insert(param_name.clone(), value);
                    }
                };
                if let Some(template_value) = param.get("value") {
                    let rendered = template(
                        template_value.as_str().unwrap().to_string(),
                        &world.fixtures,
                    );
                    given_parameters.insert(
                        param_name.clone(),
                        serde_json::from_str(rendered.as_str()).unwrap(),
                    );
                };
            }
        }

        let api_name = if let Some(tag) = given.get("tag") {
            let mut api_name = tag
                .as_str()
                .expect("failed to parse given tag as str")
                .to_string();
            api_name.retain(|c| !c.is_whitespace());

            api_name
        } else {
            world.api_name.clone().unwrap()
        };

        let operation_id = given
            .get("operationId")
            .expect("operationId missing from given")
            .as_str()
            .expect("failed to parse given operation id as str")
            .to_string();

        let unstable_operation_id = format!(
            "{}.{}",
            given_api_version,
            operation_id.to_case(Case::Snake)
        );

        if world.config.is_unstable_operation(&unstable_operation_id) {
            world
                .config
                .set_unstable_operation_enabled(&unstable_operation_id, true);
        }

        initialize_api_instance(world, api_name);

        let test_call = world
            .function_mappings
            .get(&format!("{}.{}", given_api_version, &operation_id))
            .unwrap();

        test_call(world, &given_parameters);

        if let Some(source) = given.get("source") {
            let source_path = source.as_str().unwrap().to_string();
            if let Some(fixture) = lookup(&source_path, &world.response.object) {
                if let Value::Object(ref mut map) = world.fixtures {
                    map.insert(given_key.clone(), fixture);
                }
            }
        } else if let Value::Object(ref mut map) = world.fixtures {
            map.insert(given_key.clone(), world.response.object.clone());
        }
        match build_undo(
            world,
            &operation_id,
            Some(given_key),
            given_parameters.clone(),
        ) {
            Ok(Some(undo)) => world.undo_operations.push(undo),
            Ok(None) => {}
            Err(err) => panic!("{err}"),
        }
    })
}

#[given(expr = "new {string} request")]
fn new_request(world: &mut DatadogWorld, operation_id: String) {
    world.operation_id = operation_id
}

#[given(expr = "operation {string} enabled")]
fn enable_unstable(world: &mut DatadogWorld, operation_id: String) {
    let operation_id = format!(
        "v{}.{}",
        world.api_version,
        operation_id.to_case(Case::Snake)
    );
    world
        .config
        .set_unstable_operation_enabled(&operation_id, true);
    initialize_api_instance(world, world.api_name.clone().unwrap());
}

#[given(regex = r"^body with value (.*)$")]
fn body_with_value(world: &mut DatadogWorld, body: String) {
    let rendered = template(body, &world.fixtures);
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.parameters.insert("body".to_string(), body_struct);
}

#[given(expr = "body from file {string}")]
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
    let lookup = lookup(&path, &world.fixtures).expect("failed to lookup parameter");
    world.parameters.insert(param, lookup);
}

#[given(expr = "request contains {string} parameter with value {}")]
fn request_parameter_with_value(world: &mut DatadogWorld, param: String, value: String) {
    let trimmed_value = value.trim_matches('"').to_string();
    let rendered = template(trimmed_value.clone(), &world.fixtures);
    // check if the value was an explicit string
    if trimmed_value != value {
        world.parameters.insert(param, Value::String(rendered));
        return;
    }
    if NUMBER_RE.is_match(&rendered) {
        let number =
            serde_json::Number::from_str(&rendered).expect("couldn't parse param as number");
        world.parameters.insert(param, Value::Number(number));
    } else if BOOL_RE.is_match(&rendered) {
        let boolean = Value::Bool(rendered == "true");
        world.parameters.insert(param, boolean);
    } else if ARRAY_RE.is_match(&rendered) {
        let vec: Vec<Value> = serde_json::from_str(&rendered).expect("couldn't parse param as vec");
        world.parameters.insert(param, Value::Array(vec));
    } else {
        world.parameters.insert(param, Value::from(rendered));
    }
}

#[when(regex = r"^the request is sent$")]
fn request_sent(world: &mut DatadogWorld) {
    world
        .function_mappings
        .get(&format!("v{}.{}", world.api_version, &world.operation_id))
        .expect(&format!(
            "{:?} request operation id not found",
            world.operation_id
        ))(world, &world.parameters.clone());

    match build_undo(
        world,
        &world.operation_id.clone(),
        None,
        world.parameters.clone(),
    ) {
        Ok(Some(undo)) => {
            world.undo_operations.push(undo);
        }
        Ok(None) => {}
        Err(err) => panic!("{err}"),
    }
}

// #[when(regex = r"^the request with pagination is sent$")]
// fn request_with_pagination_sent(_world: &mut DatadogWorld) {

// }

#[then(expr = "the response status is {int} {}")]
fn response_status_is(world: &mut DatadogWorld, status_code: u16, _status_message: String) {
    assert!(world.response.code == status_code)
}

#[then(expr = "the response {string} is equal to {}")]
fn response_equal_to(world: &mut DatadogWorld, path: String, value: String) {
    let lookup = lookup(&path, &world.response.object).expect("value not found in response");
    let rendered_value = template(value, &world.fixtures);
    let expected: Value = serde_json::from_str(rendered_value.as_str()).unwrap();
    if lookup.is_number() && expected.is_number() {
        assert_eq!(lookup.as_f64().unwrap(), expected.as_f64().unwrap());
    } else {
        assert_eq!(lookup, expected);
    }
}

#[then(expr = "the response {string} has field {string}")]
fn response_has_field(world: &mut DatadogWorld, path: String, field_path: String) {
    let found = lookup(&path, &world.response.object).expect("value not found in response");
    let field = lookup(&field_path, &found);
    assert!(field.is_some());
}

#[then(expr = "the response {string} has item with field {string} with value {}")]
fn response_has_item_with_field(
    world: &mut DatadogWorld,
    path: String,
    field_path: String,
    value: String,
) {
    let found = lookup(&path, &world.response.object).expect("value not found in response");
    let rendered_value = template(value, &world.fixtures);
    let expected: Value = serde_json::from_str(rendered_value.as_str()).unwrap();
    for item in found.as_array().unwrap() {
        let field = lookup(&field_path, item);
        if field.is_some() {
            let field = field.unwrap();
            if field.is_number()
                && expected.is_number()
                && field.as_f64().unwrap() == expected.as_f64().unwrap()
            {
                return;
            } else if field == expected {
                return;
            }
        }
    }
    assert!(false);
}

#[then(expr = "the response {string} array contains value {}")]
fn response_contains(world: &mut DatadogWorld, path: String, value: String) {
    let lookup = lookup(&path, &world.response.object).expect("value not found in response");
    let rendered_value = template(value, &world.fixtures);
    let expected: Value = serde_json::from_str(rendered_value.as_str()).unwrap();
    for item in lookup.as_array().unwrap() {
        if item.is_number()
            && expected.is_number()
            && item.as_f64().unwrap() == expected.as_f64().unwrap()
        {
            return;
        } else if item == &expected {
            return;
        }
    }
    assert!(false);
}

#[then(expr = "the response {string} has the same value as {string}")]
fn response_same_value_as(world: &mut DatadogWorld, path: String, value: String) {
    let lookup_lhs = lookup(&path, &world.response.object).expect("value not found in response");
    let lookup_rhs = lookup(&value, &world.fixtures).expect("value not found in fixtures");
    assert_eq!(lookup_lhs, lookup_rhs);
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

#[then(expr = "the response {string} is {word}")]
fn response_is_bool(world: &mut DatadogWorld, path: String, expected: String) {
    let found = lookup(&path, &world.response.object)
        .unwrap()
        .as_bool()
        .unwrap();
    assert_eq!(found, expected == "true");
}

fn req_eq(lhs: &vcr_cassette::Request, rhs: &vcr_cassette::Request) -> bool {
    let lhs_queries: HashSet<_> = lhs
        .uri
        .query()
        .unwrap_or_default()
        .split("&")
        .into_iter()
        .collect();
    let rhs_queries: HashSet<_> = rhs
        .uri
        .query()
        .unwrap_or_default()
        .split("&")
        .into_iter()
        .collect();
    lhs.uri.scheme() == rhs.uri.scheme()
        && lhs.uri.host() == rhs.uri.host()
        && lhs.uri.port() == rhs.uri.port()
        && lhs.uri.path() == rhs.uri.path()
        && lhs_queries == rhs_queries
        && lhs.body.string == rhs.body.string
        && lhs.method == rhs.method
}

fn lookup(path: &String, object: &Value) -> Option<Value> {
    let mut json_pointer = format!("/{}", path).replace('.', "/");
    for (_, [idx]) in INDEX_RE
        .captures_iter(&json_pointer.clone())
        .map(|c| c.extract())
    {
        json_pointer = INDEX_RE
            .replace(&json_pointer, format!("/{idx}"))
            .to_string();
    }
    return object.pointer(&json_pointer).cloned();
}

fn relative_time_helper(v: &String, ts: i64) -> DateTime<chrono::Utc> {
    // get parameter from helper or throw an error
    let caps = TIME_FMT_HELPER_RE
        .captures(&v)
        .expect("failed to parse timeISO template function");
    let ts_s = ts / 1000;
    let ts_n =
        u32::try_from((ts % 1000) * 1_000_000).expect("timestamp could not be converted to ns");
    let time = chrono::DateTime::from_timestamp(ts_s, ts_n).unwrap();
    if caps.get(1).is_some() {
        let diff = str::parse::<i64>(
            &(caps.get(1).unwrap().as_str().to_string() + caps.get(2).unwrap().as_str()),
        )
        .unwrap();
        match caps.get(3).unwrap().as_str() {
            "s" => time.add(Duration::seconds(diff)),
            "m" => time.add(Duration::minutes(diff)),
            "h" => time.add(Duration::hours(diff)),
            "d" => time.add(Duration::days(diff)),
            "M" => {
                if diff > 0 {
                    time.checked_add_months(Months::new(diff as u32)).unwrap()
                } else {
                    time.checked_sub_months(Months::new(diff.abs() as u32))
                        .unwrap()
                }
            }
            "y" => {
                if diff > 0 {
                    time.checked_add_months(Months::new((diff * 12) as u32))
                        .unwrap()
                } else {
                    time.checked_sub_months(Months::new((diff.abs() * 12) as u32))
                        .unwrap()
                }
            }
            _ => panic!("invalid time unit"),
        }
    } else {
        time
    }
}

fn time_iso_helper(state: &State, time_str: String) -> String {
    let now: i64 = state.lookup("now_millis").unwrap().try_into().unwrap();
    relative_time_helper(&time_str, now).to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn timestamp_helper(state: &State, time_str: String) -> String {
    let now: i64 = state.lookup("now_millis").unwrap().try_into().unwrap();
    relative_time_helper(&time_str, now)
        .signed_duration_since(DateTime::UNIX_EPOCH)
        .num_seconds()
        .to_string()
}

fn template(string: String, fixtures: &Value) -> String {
    TEMPLATE_ENV
        .render_str(string.as_str(), fixtures)
        .expect("failed to apply template")
}

fn process_param_from_response(
    param: &Value,
    undo_operation: &mut UndoOperation,
    given_key: Option<String>,
    world: &DatadogWorld,
) {
    let param_name = param.get("name").unwrap().as_str().unwrap().to_string();

    if let Some(source) = param.get("source") {
        if let Some(value) = lookup(
            &source.as_str().unwrap().to_string(),
            &world.response.object,
        ) {
            undo_operation.parameters.insert(param_name.clone(), value);
        }
    }

    if let Some(template_value) = param.get("template") {
        if let Some(rendered) = template_value.as_str() {
            let json_value = match given_key.clone() {
                Some(key) => template(
                    rendered.to_string(),
                    &world
                        .fixtures
                        .get(&key)
                        .unwrap_or_else(|| &world.response.object),
                ),
                None => template(rendered.to_string(), &world.response.object),
            };
            undo_operation.parameters.insert(
                param_name.clone(),
                serde_json::from_str(json_value.as_str()).unwrap(),
            );
        }
    }
}

fn process_param_from_request(
    param: &Value,
    undo_operation: &mut UndoOperation,
    request_parameters: HashMap<String, Value>,
) {
    let param_name = param.get("name").unwrap().as_str().unwrap().to_string();

    if let Some(source) = param.get("source") {
        if let Some(value) = lookup(
            &source.as_str().unwrap().to_string(),
            &serde_json::to_value(&request_parameters).unwrap(),
        ) {
            undo_operation.parameters.insert(param_name.clone(), value);
        }
    }
    let request_params_value = &serde_json::to_value(
        &request_parameters
            .get(&param_name)
            .unwrap_or(&serde_json::Value::Null),
    )
    .unwrap();
    if let Some(template_value) = param.get("template") {
        if let Some(rendered) = template_value.as_str() {
            let json_value = template(rendered.to_string(), request_params_value);
            undo_operation.parameters.insert(
                param_name.clone(),
                serde_json::from_str(json_value.as_str()).unwrap(),
            );
        }
    }
}

fn build_undo(
    world: &mut DatadogWorld,
    operation_id: &String,
    given_key: Option<String>,
    request_parameters: HashMap<String, Value>,
) -> Result<Option<UndoOperation>, Value> {
    if world.response.code < 200 || world.response.code >= 300 {
        return Ok(None);
    }
    let undo = UNDO_MAP
        .get(operation_id)
        .unwrap()
        .get("undo")
        .unwrap();
    match undo.get("type").unwrap().as_str() {
        Some("unsafe") => {
            let api_name = if let Some(tag) = undo.get("tag") {
                let mut api_name = tag.as_str().unwrap().to_string();
                api_name.retain(|c| !c.is_whitespace());
                api_name
            } else {
                world.api_name.clone().unwrap()
            };

            let mut undo_operation = UndoOperation {
                operation_id: undo
                    .get("operationId")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                tag: Some(api_name.clone()),
                parameters: HashMap::new(),
            };

            let unstable_operation_id: String = format!(
                "v{}.{}",
                world.api_version,
                undo.get("operationId")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_case(Case::Snake)
            );

            if world.config.is_unstable_operation(&unstable_operation_id) {
                world
                    .config
                    .set_unstable_operation_enabled(&unstable_operation_id, true);
            }
            initialize_api_instance(world, undo_operation.tag.clone().unwrap());

            let params = undo.get("parameters").unwrap().as_array().unwrap();
            for param in params {
                match param.get("origin") {
                    Some(origin) => {
                        if origin == "response" {
                            process_param_from_response(
                                param,
                                &mut undo_operation,
                                given_key.clone(),
                                world,
                            );
                        } else if origin == "request" {
                            process_param_from_request(
                                param,
                                &mut undo_operation,
                                request_parameters.clone(),
                            );
                        }
                    }
                    None => {
                        process_param_from_response(
                            param,
                            &mut undo_operation,
                            given_key.clone(),
                            world,
                        );
                    }
                }
            }
            Ok(Some(undo_operation))
        }
        _ => Ok(None),
    }
}

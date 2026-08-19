use crate::{
    scenarios::function_mappings::{collect_function_calls, initialize_api_instance, ApiInstances},
    GIVEN_MAP, UNDO_MAP,
};
use chrono::{DateTime, Duration, Months, SecondsFormat, Utc};
use convert_case::{Case, Casing};
use cucumber::{
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
    given, then, when, World,
};
use datadog_api_client::datadog::{APIKey, Configuration};
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
    fs::{create_dir_all, read_to_string, remove_file, File},
    io::Write,
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
    pub http_client: Option<reqwest_middleware::ClientWithMiddleware>,
    pub fixtures: Value,
    pub function_mappings: HashMap<String, TestCall>,
    pub operation_id: String,
    pub parameters: HashMap<String, Value>,
    pub path_parameters: HashMap<String, Value>,
    pub response: Response,
    pub api_name: Option<String>,
    pub api_instances: Box<ApiInstances>,
    undo_operations: Vec<UndoOperation>,
    test_feature: String,
    test_scenario: String,
    test_server_session: Option<String>,
    test_runner_plan: Option<Value>,
}

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"^\d+$").unwrap();
    static ref BOOL_RE: Regex = Regex::new(r"^(true|false)$").unwrap();
    static ref ARRAY_RE: Regex = Regex::new(r"^\[.*\]$").unwrap();
    static ref INDEX_RE: Regex = Regex::new(r"\[(\d+)\]+").unwrap();
    static ref NON_ALNUM_RE: Regex = Regex::new(r"[^A-Za-z0-9]+").unwrap();
    static ref TIME_FMT_HELPER_RE: Regex =
        Regex::new(r"now(?: *([+-]) *(\d+)([smhdMy]))?").unwrap();
    static ref LEADING_ARR_RE: Regex = Regex::new(r"^\/[\/]+").unwrap();
    static ref TEMPLATE_ENV: Environment<'static> = {
        let mut env = Environment::new();
        env.add_function("timestamp", timestamp_helper);
        env.add_function("timeISO", time_iso_helper);
        env
    };
}

fn test_runner_enabled() -> bool {
    generated_tests_enabled() && env::var("DD_TEST_RUNNER_DATA").is_ok()
}

fn test_server_enabled() -> bool {
    generated_tests_enabled() && env::var("DD_TEST_SERVER_URL").is_ok()
}

fn generated_tests_enabled() -> bool {
    env::var("DD_USE_GENERATED_TESTS")
        .unwrap_or_else(|_| "false".to_string())
        .eq_ignore_ascii_case("true")
}

pub fn feature_data_path(version: i32, filename: &str) -> PathBuf {
    PathBuf::from("tests/scenarios/features")
        .join(format!("v{version}"))
        .join(filename)
}

fn set_test_fixtures(
    world: &mut DatadogWorld,
    scenario: &Scenario,
    prefix: &str,
    frozen_time: Duration,
) {
    let escaped_name = NON_ALNUM_RE
        .replace_all(scenario.name.as_str(), "_")
        .to_string();
    let name = match escaped_name.len() > 100 {
        true => escaped_name[..100].to_string(),
        false => escaped_name,
    };
    let unique = format!("{}-{}-{}", prefix, name, frozen_time.num_seconds());
    let unique_alnum = NON_ALNUM_RE.replace_all(unique.as_str(), "").to_string();
    let uuid_first = frozen_time.num_seconds().to_string();
    let uuid = format!(
        "{}-0000-0000-0000-{}00",
        uuid_first[..8].to_string(),
        uuid_first[..10].to_string()
    );
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
        "uuid": uuid,
    });
}

async fn test_server_request(endpoint: &str, payload: Option<Value>) -> Value {
    let client = reqwest::Client::new();
    let mut request = client
        .post(format!(
            "{}/__openapi_transformer__{}",
            env::var("DD_TEST_SERVER_URL").unwrap(),
            endpoint
        ))
        .header("content-type", "application/json");
    if let Some(value) = payload {
        request = request.body(serde_json::to_vec(&value).unwrap());
    }
    let response = request.send().await.expect("test server request failed");
    let status = response.status();
    let body = response
        .text()
        .await
        .expect("failed to read test server response");
    assert!(
        status.is_success(),
        "Test server POST {endpoint} failed ({status}): {body}"
    );
    serde_json::from_str(&body).expect("failed to decode test server response")
}

async fn next_test_server_request(world: &DatadogWorld) -> Option<Value> {
    let session = world.test_server_session.as_ref().unwrap();
    let response = reqwest::Client::new()
        .get(format!(
            "{}/__openapi_transformer__/sessions/{session}/next-request",
            env::var("DD_TEST_SERVER_URL").unwrap(),
        ))
        .send()
        .await
        .expect("failed to inspect the next test server request");
    assert!(
        response.status().is_success(),
        "Test server next-request failed ({})",
        response.status(),
    );
    serde_json::from_str::<Value>(
        &response
            .text()
            .await
            .expect("failed to read the next test server request"),
    )
    .expect("failed to decode the next test server request")["request"]
        .as_object()
        .map(|request| Value::Object(request.clone()))
}

async fn start_test_server_session(
    feature: &Feature,
    scenario: &Scenario,
    world: &mut DatadogWorld,
) -> Duration {
    let root = PathBuf::from(env::var("DD_TEST_RUNNER_DATA").unwrap());
    let manifest: Value =
        serde_json::from_str(&read_to_string(root.join("manifest.json")).unwrap()).unwrap();
    let version = format!("v{}", world.api_version);
    let item = manifest["scenarios"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| {
            item["version"] == version
                && item["feature"] == feature.name
                && item["scenario"] == scenario.name
        })
        .unwrap_or_else(|| {
            panic!(
                "Generated request plan not found for {}/{}/{}",
                version, feature.name, scenario.name
            )
        });
    world.test_feature = feature.name.clone();
    world.test_scenario = scenario.name.clone();
    world.test_runner_plan = Some(
        serde_json::from_str(&read_to_string(root.join(item["file"].as_str().unwrap())).unwrap())
            .unwrap(),
    );
    let session = test_server_request(
        "/sessions",
        Some(json!({
            "version": version,
            "feature": feature.name,
            "scenario": scenario.name,
        })),
    )
    .await;
    world.test_server_session = Some(session["session"].as_str().unwrap().to_string());
    DateTime::parse_from_rfc3339(session["frozen_at"].as_str().unwrap())
        .expect("failed to parse generated test-server time")
        .signed_duration_since(DateTime::UNIX_EPOCH)
}

async fn stop_test_server_session(world: &mut DatadogWorld) {
    let Some(session) = world.test_server_session.take() else {
        return;
    };
    test_server_request(&format!("/sessions/{session}/stop"), None).await;
}

fn materialize_test_value(value: &Value, fixtures: &Value) -> Value {
    match value {
        Value::Object(map)
            if map.len() == 1 && map.contains_key("$openapi_transformer_template") =>
        {
            let rendered = template(
                map["$openapi_transformer_template"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                fixtures,
            );
            serde_json::from_str(&rendered).expect("failed to decode rendered request value")
        }
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, item)| (key.clone(), materialize_test_value(item, fixtures)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| materialize_test_value(item, fixtures))
                .collect(),
        ),
        Value::String(value) => {
            let mut rendered = template(value.clone(), fixtures);
            let trailing_newlines = value.len() - value.trim_end_matches('\n').len();
            let rendered_newlines = rendered.len() - rendered.trim_end_matches('\n').len();
            rendered.push_str(&"\n".repeat(trailing_newlines.saturating_sub(rendered_newlines)));
            Value::String(rendered)
        }
        _ => value.clone(),
    }
}

fn apply_test_runner_plan(world: &mut DatadogWorld, pagination: bool) {
    if !test_runner_enabled() {
        return;
    }
    let plan = world.test_runner_plan.clone().unwrap();
    assert_eq!(plan["request"]["pagination"].as_bool().unwrap(), pagination);
    let api_name = plan["api"].as_str().unwrap().replace('-', "");
    initialize_api_instance(world, api_name.clone());
    world.api_name = Some(api_name);
    world.operation_id = plan["operation_id"].as_str().unwrap().to_string();
    world.parameters.clear();
    world.path_parameters.clear();

    for parameter in plan["request"]["parameters"].as_array().unwrap() {
        let name = parameter["name"].as_str().unwrap().to_string();
        let source = &parameter["source"];
        let value = if source["type"] == "fixture" {
            lookup(
                &source["path"].as_str().unwrap().to_string(),
                &world.fixtures,
            )
            .expect("failed to lookup generated request fixture")
        } else {
            materialize_test_value(&source["value"], &world.fixtures)
        };
        world.parameters.insert(name.clone(), value.clone());
        world.path_parameters.insert(name.clone(), value.clone());
        world
            .path_parameters
            .insert(name.to_case(Case::Snake), value);
    }
    if !plan["request"]["body"].is_null() {
        world.parameters.insert(
            "body".to_string(),
            materialize_test_value(&plan["request"]["body"]["value"], &world.fixtures),
        );
    }
}

fn test_value_as_string(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(test_value_as_string)
            .collect::<Vec<_>>()
            .join(","),
        Value::Null => String::new(),
        _ => value.to_string(),
    }
}

async fn send_test_runner_request(world: &mut DatadogWorld) {
    let plan = world.test_runner_plan.clone().unwrap();
    let request_plan = &plan["request"];
    let mut path = request_plan["path"].as_str().unwrap().to_string();
    let mut query = Vec::new();
    let mut headers = reqwest::header::HeaderMap::new();

    for parameter in request_plan["parameters"].as_array().unwrap() {
        let name = parameter["name"].as_str().unwrap();
        let value = world.parameters.get(name).unwrap();
        match parameter["in"].as_str().unwrap() {
            "path" => {
                path = path.replace(&format!("{{{name}}}"), &test_value_as_string(value));
            }
            "query" => {
                if let Some(values) = value.as_array() {
                    if parameter["explode"].as_bool().unwrap_or(true) {
                        query.extend(
                            values
                                .iter()
                                .map(|value| (name.to_string(), test_value_as_string(value))),
                        );
                    } else {
                        query.push((name.to_string(), test_value_as_string(value)));
                    }
                } else {
                    query.push((name.to_string(), test_value_as_string(value)));
                }
            }
            "header" => {
                headers.insert(
                    reqwest::header::HeaderName::from_str(name).unwrap(),
                    reqwest::header::HeaderValue::from_str(&test_value_as_string(value)).unwrap(),
                );
            }
            location => panic!("unsupported generated request parameter location: {location}"),
        }
    }
    headers.insert(
        "x-openapi-test-session",
        world.test_server_session.as_ref().unwrap().parse().unwrap(),
    );

    let client = reqwest::Client::new();
    let method =
        reqwest::Method::from_bytes(request_plan["method"].as_str().unwrap().as_bytes()).unwrap();
    let mut request = client
        .request(
            method,
            format!("{}{}", env::var("DD_TEST_SERVER_URL").unwrap(), path),
        )
        .headers(headers)
        .query(&query);
    if let Some(content_type) = request_plan["content_type"].as_str() {
        request = request.header("content-type", content_type);
    }
    if let Some(body) = world.parameters.get("body") {
        request = request.body(serde_json::to_vec(body).unwrap());
    }

    let response = request.send().await.expect("generated test request failed");
    world.response.code = response.status().as_u16();
    let request_mismatch = response
        .headers()
        .get("x-openapi-test-error")
        .is_some_and(|value| value == "request-mismatch");
    let body = response
        .bytes()
        .await
        .expect("failed to read generated test response");
    assert!(
        !request_mismatch,
        "generated request did not match the test server: {}",
        String::from_utf8_lossy(&body),
    );
    world.response.object = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&body).into_owned()))
    };
}

fn response_items(response: &Value) -> Vec<Value> {
    if let Some(items) = response.as_array() {
        return items.clone();
    }
    let object = response
        .as_object()
        .expect("paginated response must be an array or object");
    if let Some(items) = object.get("data").and_then(Value::as_array) {
        return items.clone();
    }
    object
        .values()
        .find_map(Value::as_array)
        .expect("paginated response object has no array field")
        .clone()
}

async fn send_test_runner_page(world: &mut DatadogWorld, request_plan: &Value) {
    let session = world.test_server_session.as_ref().unwrap();
    let method =
        reqwest::Method::from_bytes(request_plan["method"].as_str().unwrap().as_bytes()).unwrap();
    let mut request = reqwest::Client::new()
        .request(
            method,
            format!(
                "{}{}",
                env::var("DD_TEST_SERVER_URL").unwrap(),
                request_plan["path"].as_str().unwrap(),
            ),
        )
        .header("x-openapi-test-session", session);
    if let Some(content_type) = request_plan["content_type"].as_str() {
        if !content_type.is_empty() {
            request = request.header("content-type", content_type);
        }
    }
    let query: Vec<(String, String)> = request_plan["query"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            let pair = item.as_array().unwrap();
            (
                pair[0].as_str().unwrap().to_string(),
                pair[1].as_str().unwrap().to_string(),
            )
        })
        .collect();
    request = request.query(&query);
    match request_plan["body"]["type"].as_str().unwrap() {
        "empty" => {}
        "json" => {
            request = request.body(serde_json::to_vec(&request_plan["body"]["value"]).unwrap());
        }
        "text" => {
            request = request.body(request_plan["body"]["value"].as_str().unwrap().to_string());
        }
        body_type => panic!("unsupported generated request body type: {body_type}"),
    }
    let response = request
        .send()
        .await
        .expect("generated pagination request failed");
    world.response.code = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .expect("failed to read generated pagination response");
    world.response.object = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).unwrap()
    };
}

async fn send_test_runner_paginated_request(world: &mut DatadogWorld) {
    let first = next_test_server_request(world)
        .await
        .expect("generated pagination request not found");
    let method = first["method"].as_str().unwrap().to_string();
    let path = first["path"].as_str().unwrap().to_string();
    send_test_runner_page(world, &first).await;
    let mut items = response_items(&world.response.object);

    while let Some(next) = next_test_server_request(world).await {
        if next["method"] != method || next["path"] != path {
            break;
        }
        send_test_runner_page(world, &next).await;
        items.extend(response_items(&world.response.object));
    }
    world.response.object = Value::Array(items);
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

    world.config.set_auth_key(
        "apiKeyAuth",
        APIKey {
            key: "00000000000000000000000000000000".to_string(),
            prefix: "".to_owned(),
        },
    );

    if test_server_enabled() {
        let frozen_time = start_test_server_session(feature, scenario, world).await;
        world.config.server_index = 1;
        world.config.server_variables = HashMap::from([
            ("protocol".to_string(), "http".to_string()),
            (
                "name".to_string(),
                env::var("DD_TEST_SERVER_URL")
                    .unwrap()
                    .trim_start_matches("http://")
                    .to_string(),
            ),
        ]);
        world.config.set_retry(false, 0);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-openapi-test-session",
            world.test_server_session.as_ref().unwrap().parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        world.http_client = Some(ClientBuilder::new(client).build());
        set_test_fixtures(world, scenario, "Test", frozen_time);
        return;
    }
    world.config.set_auth_key(
        "appKeyAuth",
        APIKey {
            key: "0000000000000000000000000000000000000000".to_string(),
            prefix: "".to_owned(),
        },
    );

    let mut reqwest_client_builder = reqwest::Client::builder();
    if let Some(proxy_url) = &world.config.proxy_url {
        let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
        reqwest_client_builder = reqwest_client_builder.proxy(proxy);
    }

    let mut vcr_client_builder = ClientBuilder::new(reqwest_client_builder.build().unwrap());
    vcr_client_builder = match env::var("RECORD").unwrap_or("false".to_string()).as_str() {
        "none" => {
            prefix.push_str("-Rust");
            vcr_client_builder
        }
        "true" => {
            let _ = remove_file(cassette.clone());
            let _ = remove_file(freeze.clone());
            let mut freeze_file = File::create(freeze).expect("failed to write freeze file");
            freeze_file
                .write_all(
                    Utc::now()
                        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                        .to_string()
                        .as_bytes(),
                )
                .expect("failed to write freeze file");
            let middleware: VCRMiddleware = VCRMiddleware::try_from(cassette)
                .expect("Failed to initialize rVCR middleware")
                .with_mode(VCRMode::Record)
                .with_modify_request(|req| {
                    req.headers.remove_entry("dd-api-key");
                    req.headers.remove_entry("dd-application-key");
                })
                .with_modify_response(|res| {
                    res.headers.remove_entry("content-security-policy");
                });
            vcr_client_builder.with(middleware)
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
            vcr_client_builder.with(middleware)
        }
    };

    world.config.set_retry(true, 3);

    if world.config.enable_retry {
        struct RetryableStatus;
        impl reqwest_retry::RetryableStrategy for RetryableStatus {
            fn handle(
                &self,
                res: &Result<reqwest::Response, reqwest_middleware::Error>,
            ) -> Option<reqwest_retry::Retryable> {
                match res {
                    Ok(success) => reqwest_retry::default_on_request_success(success),
                    Err(_) => None,
                }
            }
        }
        let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
            .build_with_max_retries(world.config.max_retries);

        let retry_middleware =
            reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                backoff_policy,
                RetryableStatus,
            );

        vcr_client_builder = vcr_client_builder.with(retry_middleware);
    }

    world.http_client = Some(vcr_client_builder.build());

    let escaped_name = NON_ALNUM_RE
        .replace_all(scenario.name.as_str(), "_")
        .to_string();
    let name = match escaped_name.len() > 100 {
        true => escaped_name[..100].to_string(),
        false => escaped_name,
    };
    let unique = format!("{}-{}-{}", prefix, name, frozen_time.num_seconds());
    let unique_alnum = NON_ALNUM_RE.replace_all(unique.as_str(), "").to_string();

    let uuid_first = frozen_time.num_seconds().to_string();
    let uuid = format!(
        "{}-0000-0000-0000-{}00",
        uuid_first[..8].to_string(),
        uuid_first[..10].to_string()
    );

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
        "uuid": uuid,
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
        if test_server_enabled() {
            stop_test_server_session(world).await;
            return;
        }
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
    let api_name = api.replace("-", "");
    initialize_api_instance(world, api_name.clone());
    world.api_name = Some(api_name);
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
                        given_parameters.insert(param_name.clone(), value.clone());

                        // Store in path_parameters for undo operations
                        world
                            .path_parameters
                            .insert(param_name.clone(), value.clone());
                        let snake_param = param_name.to_case(Case::Snake);
                        if snake_param != param_name {
                            world.path_parameters.insert(snake_param, value);
                        }
                    }
                };
                if let Some(template_value) = param.get("value") {
                    let rendered = template(
                        template_value.as_str().unwrap().to_string(),
                        &world.fixtures,
                    );
                    let parsed_value: Value = serde_json::from_str(rendered.as_str()).unwrap();
                    given_parameters.insert(param_name.clone(), parsed_value.clone());

                    // Store in path_parameters for undo operations
                    world
                        .path_parameters
                        .insert(param_name.clone(), parsed_value.clone());
                    let snake_param = param_name.to_case(Case::Snake);
                    if snake_param != param_name {
                        world.path_parameters.insert(snake_param, parsed_value);
                    }
                };
            }
        }

        if test_runner_enabled() {
            let request = next_test_server_request(world)
                .await
                .expect("generated setup request not found");
            send_test_runner_page(world, &request).await;
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
            return;
        }

        let api_name = if let Some(tag) = given.get("tag") {
            let mut api_name = tag
                .as_str()
                .expect("failed to parse given tag as str")
                .to_string();
            api_name.retain(|c| !c.is_whitespace() && c != '-');

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
    if test_runner_enabled() {
        return;
    }
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
    if test_runner_enabled() {
        return;
    }
    let rendered = template(body, &world.fixtures);
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.parameters.insert("body".to_string(), body_struct);
}

#[given(expr = "body from file {string}")]
fn body_from_file(world: &mut DatadogWorld, path: String) {
    if test_runner_enabled() {
        return;
    }
    let body = read_to_string(feature_data_path(world.api_version, &path)).unwrap();
    let rendered = template(body, &world.fixtures);
    let body_struct = serde_json::from_str(rendered.as_str()).unwrap();
    world.parameters.insert("body".to_string(), body_struct);
}

#[given(expr = "request contains {string} parameter from {string}")]
fn request_parameter_from_path(world: &mut DatadogWorld, param: String, path: String) {
    if test_runner_enabled() {
        return;
    }
    let lookup = lookup(&path, &world.fixtures).expect("failed to lookup parameter");
    world.parameters.insert(param.clone(), lookup.clone());
    // Store path parameter for undo operations with naming variants
    world.path_parameters.insert(param.clone(), lookup.clone());
    let snake_param = param.to_case(Case::Snake);
    if snake_param != param {
        world.path_parameters.insert(snake_param, lookup);
    }
}

#[given(expr = "request contains {string} parameter with value {}")]
fn request_parameter_with_value(world: &mut DatadogWorld, param: String, value: String) {
    if test_runner_enabled() {
        return;
    }
    let trimmed_value = value.trim_matches('"').to_string();
    let rendered = template(trimmed_value.clone(), &world.fixtures);
    // check if the value was an explicit string
    if trimmed_value != value {
        let val = Value::String(rendered);
        world.parameters.insert(param.clone(), val.clone());
        world.path_parameters.insert(param.clone(), val.clone());
        let snake_param = param.to_case(Case::Snake);
        if snake_param != param {
            world.path_parameters.insert(snake_param, val);
        }
        return;
    }
    if NUMBER_RE.is_match(&rendered) {
        let number =
            serde_json::Number::from_str(&rendered).expect("couldn't parse param as number");
        let val = Value::Number(number);
        world.parameters.insert(param.clone(), val.clone());
        world.path_parameters.insert(param.clone(), val.clone());
        let snake_param = param.to_case(Case::Snake);
        if snake_param != param {
            world.path_parameters.insert(snake_param, val);
        }
    } else if BOOL_RE.is_match(&rendered) {
        let boolean = Value::Bool(rendered == "true");
        world.parameters.insert(param.clone(), boolean.clone());
        world.path_parameters.insert(param.clone(), boolean.clone());
        let snake_param = param.to_case(Case::Snake);
        if snake_param != param {
            world.path_parameters.insert(snake_param, boolean);
        }
    } else if ARRAY_RE.is_match(&rendered) {
        let vec: Vec<Value> = serde_json::from_str(&rendered).expect("couldn't parse param as vec");
        let val = Value::Array(vec);
        world.parameters.insert(param.clone(), val.clone());
        world.path_parameters.insert(param.clone(), val.clone());
        let snake_param = param.to_case(Case::Snake);
        if snake_param != param {
            world.path_parameters.insert(snake_param, val);
        }
    } else {
        let val = Value::from(rendered);
        world.parameters.insert(param.clone(), val.clone());
        world.path_parameters.insert(param.clone(), val.clone());
        let snake_param = param.to_case(Case::Snake);
        if snake_param != param {
            world.path_parameters.insert(snake_param, val);
        }
    }
}

#[when(regex = r"^the request is sent$")]
async fn request_sent(world: &mut DatadogWorld) {
    apply_test_runner_plan(world, false);
    if test_runner_enabled() {
        send_test_runner_request(world).await;
        return;
    }
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

#[when(regex = r"^the request with pagination is sent$")]
async fn request_with_pagination_sent(world: &mut DatadogWorld) {
    apply_test_runner_plan(world, true);
    if test_runner_enabled() {
        send_test_runner_paginated_request(world).await;
        return;
    }
    world
        .function_mappings
        .get(&format!(
            "v{}.{}WithPagination",
            world.api_version, &world.operation_id
        ))
        .expect(&format!(
            "{:?} request operation id not found",
            world.operation_id
        ))(world, &world.parameters.clone());
}

#[then(expr = "the response has {int} items")]
fn response_has_items(world: &mut DatadogWorld, size: usize) {
    assert!(world.response.object.as_array().unwrap().len() == size);
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

#[then(expr = "the response {string} does not have field {string}")]
fn response_does_not_have_field(world: &mut DatadogWorld, path: String, field_path: String) {
    let found = lookup(&path, &world.response.object).expect("value not found in response");
    let field = lookup(&field_path, &found);
    assert!(field.is_none());
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
    let mut lhs_query = urlencoding::decode(
        lhs.uri
            .query()
            .unwrap_or_default()
            .to_string()
            .replace("+", "%20")
            .as_str(),
    )
    .expect("UTF-8")
    .to_string();

    let mut rhs_query = urlencoding::decode(
        rhs.uri
            .query()
            .unwrap_or_default()
            .to_string()
            .replace("+", "%20")
            .as_str(),
    )
    .expect("UTF-8")
    .to_string();

    lhs_query = reformat_rfc3339_datetime(&lhs_query);
    rhs_query = reformat_rfc3339_datetime(&rhs_query);

    let lhs_queries: HashSet<_> = lhs_query.split("&").into_iter().collect();
    let rhs_queries: HashSet<_> = rhs_query.split("&").into_iter().collect();

    let lhs_body = lhs
        .body
        .string
        .parse::<serde_json::Value>()
        .unwrap_or_default();
    let rhs_body = rhs
        .body
        .string
        .parse::<serde_json::Value>()
        .unwrap_or_default();

    lhs.uri.scheme() == rhs.uri.scheme()
        && lhs.uri.host() == rhs.uri.host()
        && lhs.uri.port() == rhs.uri.port()
        && lhs.uri.path() == rhs.uri.path()
        && lhs_queries == rhs_queries
        && lhs_body == rhs_body
        && lhs.method == rhs.method
}

fn reformat_rfc3339_datetime(input: &str) -> String {
    let re: Regex =
        Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})").unwrap();
    let result = re.replace_all(input, |captures: &regex::Captures| {
        let matched_date_time = &captures[0];
        let parsed_date_time = DateTime::parse_from_rfc3339(matched_date_time)
            .expect("Failed to parse datetime")
            .with_timezone(&Utc);
        let formatted_date_time =
            parsed_date_time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        formatted_date_time
    });
    result.to_string()
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

    // Handle leading indexes and current object references
    json_pointer = LEADING_ARR_RE.replace(&json_pointer, "/").to_string();
    if json_pointer == "/" {
        return Some(object.clone());
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
            "s" => time.add(Duration::try_seconds(diff).unwrap()),
            "m" => time.add(Duration::try_minutes(diff).unwrap()),
            "h" => time.add(Duration::try_hours(diff).unwrap()),
            "d" => time.add(Duration::try_days(diff).unwrap()),
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

fn process_param_from_path(
    param: &Value,
    undo_operation: &mut UndoOperation,
    path_parameters: &HashMap<String, Value>,
) {
    let param_name = param.get("name").unwrap().as_str().unwrap().to_string();

    if let Some(source) = param.get("source") {
        let source_str = source.as_str().unwrap();
        // Try multiple naming variants
        if let Some(value) = path_parameters.get(source_str) {
            undo_operation
                .parameters
                .insert(param_name.clone(), value.clone());
        } else {
            let snake_source = source_str.to_case(Case::Snake);
            if let Some(value) = path_parameters.get(&snake_source) {
                undo_operation
                    .parameters
                    .insert(param_name.clone(), value.clone());
            } else {
                panic!(
                    "Path parameter '{}' not found in path_parameters",
                    source_str
                );
            }
        }
    } else {
        panic!("Path origin requires 'source' field");
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
    let undo = UNDO_MAP.get(operation_id).unwrap().get("undo").unwrap();
    match undo.get("type").unwrap().as_str() {
        Some("unsafe") => {
            let mut api_name = if let Some(tag) = undo.get("tag") {
                tag.as_str().unwrap().to_string()
            } else {
                UNDO_MAP
                    .get(operation_id)
                    .unwrap()
                    .get("tag")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            };
            api_name.retain(|c| !c.is_whitespace() && c != '-');

            if undo.get("operationId").is_none() {
                return Ok(None);
            }

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
                        } else if origin == "path" {
                            process_param_from_path(
                                param,
                                &mut undo_operation,
                                &world.path_parameters,
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

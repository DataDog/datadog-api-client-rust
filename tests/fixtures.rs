use cucumber::{gherkin::Feature, gherkin::Rule, gherkin::Scenario, given, then, when, World};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api_fastly_integration::{CreateFastlyAccount, CreateFastlyAccountParams};

#[derive(Debug, Default, World)]
pub struct DatadogWorld {
    config: Configuration,
    operation_id: String,
    body: serde_json::Value,
}

pub async fn before_scenario(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _world: &mut DatadogWorld,
) {
    let mut config = Configuration::new();
    config.api_key_auth = Some("00000000000000000000000000000000".to_string());
    config.app_key_auth = Some("0000000000000000000000000000000000000000".to_string());
    _world.config = config;
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
fn a_new_request(_world: &mut DatadogWorld, operation_id: String) {
    _world.operation_id = operation_id
}

#[given(regex = r"^body with value (.*)$")]
fn body_with_value(_world: &mut DatadogWorld, body: String) {
    let body_struct = serde_json::from_str(body.as_str()).unwrap();
    _world.body = body_struct;
}

#[when(regex = r"^the request is sent$")]
async fn when_request_sent(_world: &mut DatadogWorld) {
    println!("{:#?}", _world.config);
    let response = CreateFastlyAccount(&_world.config, CreateFastlyAccountParams{body: serde_json::from_value(_world.body.clone()).unwrap()}).await.unwrap();
    println!("{:#?}", response)
}

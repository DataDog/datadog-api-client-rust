// List Anthropic Usage and Costs accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_anthropic_usage_and_costs_integration::AnthropicUsageAndCostsIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.list_anthropic_usage_and_costs_integration_accounts",
        true,
    );
    let api = AnthropicUsageAndCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .list_anthropic_usage_and_costs_integration_accounts()
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

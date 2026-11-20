// Delete an Anthropic Usage and Costs account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_anthropic_usage_and_costs_integration::AnthropicUsageAndCostsIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled(
        "v2.delete_anthropic_usage_and_costs_integration_account",
        true,
    );
    let api = AnthropicUsageAndCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_anthropic_usage_and_costs_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

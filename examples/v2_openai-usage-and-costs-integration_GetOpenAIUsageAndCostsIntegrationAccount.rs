// Get an OpenAI Usage and Costs account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_open_ai_usageand_costs_integration::OpenAIUsageandCostsIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.get_open_ai_usage_and_costs_integration_account", true);
    let api = OpenAIUsageandCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .get_open_ai_usage_and_costs_integration_account("account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

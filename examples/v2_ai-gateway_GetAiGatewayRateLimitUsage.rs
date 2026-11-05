// Get AI Gateway rate limit usage returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_ai_gateway_rate_limit_usage", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.get_ai_gateway_rate_limit_usage().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

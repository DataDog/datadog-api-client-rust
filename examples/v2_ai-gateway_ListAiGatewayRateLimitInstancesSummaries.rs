// List AI Gateway rate limit counter summaries returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.list_ai_gateway_rate_limit_instances_summaries", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.list_ai_gateway_rate_limit_instances_summaries().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

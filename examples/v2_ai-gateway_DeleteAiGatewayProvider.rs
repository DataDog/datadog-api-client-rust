// Delete an AI Gateway provider returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::DeleteAiGatewayProviderOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_ai_gateway_provider", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .delete_ai_gateway_provider(
            9223372036854775807,
            DeleteAiGatewayProviderOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

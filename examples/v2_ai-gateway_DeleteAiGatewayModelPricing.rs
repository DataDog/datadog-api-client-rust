// Delete a model pricing override returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_ai_gateway_model_pricing", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .delete_ai_gateway_model_pricing(9223372036854775807, "canonical".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// List models available from a provider integration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::ListAiGatewayProviderModelsOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_ai_gateway_provider_models", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .list_ai_gateway_provider_models(
            "integration_type".to_string(),
            "integration_account_id".to_string(),
            ListAiGatewayProviderModelsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

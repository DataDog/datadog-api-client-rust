// Update an AI Gateway provider returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayProviderType;
use datadog_api_client::datadogV2::model::AiGatewayProviderUpdateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayProviderUpdateData;
use datadog_api_client::datadogV2::model::AiGatewayProviderUpdateRequest;

#[tokio::main]
async fn main() {
    let body = AiGatewayProviderUpdateRequest::new(AiGatewayProviderUpdateData::new(
        AiGatewayProviderUpdateAttributes::new()
            .cost_multiplier(1.0 as f64)
            .name("production-us-v2".to_string()),
        "42".to_string(),
        AiGatewayProviderType::PROVIDERS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_provider", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_provider(9223372036854775807, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

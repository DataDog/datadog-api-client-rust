// Update a model for an AI Gateway provider returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayModelCatalogEntryType;
use datadog_api_client::datadogV2::model::AiGatewayModelUpdateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelUpdateData;
use datadog_api_client::datadogV2::model::AiGatewayModelUpdateRequest;
use datadog_api_client::datadogV2::model::AiGatewaySupportedOperation;

#[tokio::main]
async fn main() {
    let body = AiGatewayModelUpdateRequest::new(AiGatewayModelUpdateData::new(
        AiGatewayModelUpdateAttributes::new()
            .model_id_wire("gpt-5.2-codex-20260801".to_string())
            .supported_operations(vec![AiGatewaySupportedOperation::OPENAI_CHAT_COMPLETIONS]),
        "101".to_string(),
        AiGatewayModelCatalogEntryType::MODEL_CATALOG_ENTRIES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_model", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_model(9223372036854775807, "canonical".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

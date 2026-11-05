// Add multiple models to an AI Gateway provider returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayModelCatalogEntryType;
use datadog_api_client::datadogV2::model::AiGatewayModelCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelsBulkCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelsBulkCreateData;
use datadog_api_client::datadogV2::model::AiGatewayModelsBulkCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewaySupportedOperation;

#[tokio::main]
async fn main() {
    let body = AiGatewayModelsBulkCreateRequest::new(AiGatewayModelsBulkCreateData::new(
        AiGatewayModelsBulkCreateAttributes::new(vec![AiGatewayModelCreateAttributes::new(
            "gpt-5.2-codex".to_string(),
            "gpt-5.2-codex-20260801".to_string(),
        )
        .supported_operations(vec![AiGatewaySupportedOperation::OPENAI_CHAT_COMPLETIONS])]),
        AiGatewayModelCatalogEntryType::MODEL_CATALOG_ENTRIES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.bulk_create_ai_gateway_models", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .bulk_create_ai_gateway_models(9223372036854775807, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create an AI Gateway data capture override returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequestData;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequestDataAttributes;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequestDataAttributesMode;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequestDataAttributesOperation;
use datadog_api_client::datadogV2::model::AiGatewayCaptureCreateRequestDataType;
use datadog_api_client::datadogV2::model::AiGatewayCaptureScope;
use datadog_api_client::datadogV2::model::AiGatewayMetadataCaptureItem;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayCaptureCreateRequest::new(AiGatewayCaptureCreateRequestData::new(
        AiGatewayCaptureCreateRequestDataAttributes::new(
            "capture-revision-1".to_string(),
            AiGatewayCaptureCreateRequestDataAttributesMode::METADATA_ONLY,
            "Research".to_string(),
            AiGatewayCaptureScope::new()
                .tags(BTreeMap::from([("env".to_string(), "prod".to_string())])),
        )
        .metadata_capture(vec![
            AiGatewayMetadataCaptureItem::TOOL_USAGE,
            AiGatewayMetadataCaptureItem::TOOL_OUTPUT_SIZE,
            AiGatewayMetadataCaptureItem::TOOL_NAMES,
        ])
        .operation(AiGatewayCaptureCreateRequestDataAttributesOperation::CREATE)
        .revision(1),
        Uuid::parse_str("11111111-1111-4111-8111-111111111111").expect("invalid UUID"),
        AiGatewayCaptureCreateRequestDataType::DATA_CAPTURE_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_ai_gateway_data_capture_override", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.create_ai_gateway_data_capture_override(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Replace an AI Gateway data capture override returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayCaptureScope;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequest;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequestData;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesMode;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesOperation;
use datadog_api_client::datadogV2::model::AiGatewayCaptureUpdateRequestDataType;
use datadog_api_client::datadogV2::model::AiGatewayMetadataCaptureItem;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayCaptureUpdateRequest::new(AiGatewayCaptureUpdateRequestData::new(
        AiGatewayCaptureUpdateRequestDataAttributes::new(
            "capture-revision-1".to_string(),
            AiGatewayCaptureUpdateRequestDataAttributesMode::METADATA_ONLY,
            "Research".to_string(),
            1,
        )
        .metadata_capture(vec![
            AiGatewayMetadataCaptureItem::TOOL_USAGE,
            AiGatewayMetadataCaptureItem::TOOL_OUTPUT_SIZE,
            AiGatewayMetadataCaptureItem::TOOL_NAMES,
        ])
        .operation(AiGatewayCaptureUpdateRequestDataAttributesOperation::CREATE)
        .scope(
            AiGatewayCaptureScope::new()
                .tags(BTreeMap::from([("env".to_string(), "prod".to_string())])),
        ),
        Uuid::parse_str("11111111-1111-4111-8111-111111111111").expect("invalid UUID"),
        AiGatewayCaptureUpdateRequestDataType::DATA_CAPTURE_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_ai_gateway_data_capture_override", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_data_capture_override(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

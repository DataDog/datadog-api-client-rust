// Delete an AI Gateway data capture override returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequest;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequestData;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequestDataAttributes;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequestDataAttributesMode;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequestDataAttributesOperation;
use datadog_api_client::datadogV2::model::AiGatewayCaptureDeleteRequestDataType;
use datadog_api_client::datadogV2::model::AiGatewayCaptureScope;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayCaptureDeleteRequest::new(AiGatewayCaptureDeleteRequestData::new(
        AiGatewayCaptureDeleteRequestDataAttributes::new("capture-revision-1".to_string(), 1)
            .mode(AiGatewayCaptureDeleteRequestDataAttributesMode::METADATA_ONLY)
            .name("Research".to_string())
            .operation(AiGatewayCaptureDeleteRequestDataAttributesOperation::CREATE)
            .scope(
                AiGatewayCaptureScope::new()
                    .tags(BTreeMap::from([("env".to_string(), "prod".to_string())])),
            ),
        Uuid::parse_str("11111111-1111-4111-8111-111111111111").expect("invalid UUID"),
        AiGatewayCaptureDeleteRequestDataType::DATA_CAPTURE_OVERRIDES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.delete_ai_gateway_data_capture_override", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .delete_ai_gateway_data_capture_override(
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

// Replace AI Gateway governance controls returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayDataCaptureMode;
use datadog_api_client::datadogV2::model::AiGatewayGovernanceReplaceAttributes;
use datadog_api_client::datadogV2::model::AiGatewayGovernanceReplaceData;
use datadog_api_client::datadogV2::model::AiGatewayGovernanceReplaceRequest;
use datadog_api_client::datadogV2::model::AiGatewayGovernanceType;
use datadog_api_client::datadogV2::model::AiGatewayMetadataCaptureItem;
use datadog_api_client::datadogV2::model::AiGatewayZDRMode;

#[tokio::main]
async fn main() {
    let body = AiGatewayGovernanceReplaceRequest::new(
        AiGatewayGovernanceReplaceData::new(
            AiGatewayGovernanceReplaceAttributes::new(AiGatewayDataCaptureMode::METADATA_ONLY)
                .metadata_capture(vec![
                    AiGatewayMetadataCaptureItem::TOOL_USAGE,
                    AiGatewayMetadataCaptureItem::TOOL_OUTPUT_SIZE,
                    AiGatewayMetadataCaptureItem::TOOL_NAMES,
                ])
                .zdr_mode(AiGatewayZDRMode::OFF),
            AiGatewayGovernanceType::GOVERNANCE,
        )
        .id("organization".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_governance", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.update_ai_gateway_governance(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

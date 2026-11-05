// Update an AI Gateway model group returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupType;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupUpdateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupUpdateData;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupUpdateRequest;
use datadog_api_client::datadogV2::model::AiGatewayNullableMaxReasoningLevel;

#[tokio::main]
async fn main() {
    let body = AiGatewayModelGroupUpdateRequest::new(AiGatewayModelGroupUpdateData::new(
        AiGatewayModelGroupUpdateAttributes::new()
            .allow_fast_mode(true)
            .max_reasoning_level(Some(AiGatewayNullableMaxReasoningLevel::HIGH))
            .model_ids(vec![101])
            .name("Coding models".to_string())
            .provider_ids(vec![])
            .routing_rule_slugs(vec!["coding-route".to_string()]),
        "7".to_string(),
        AiGatewayModelGroupType::MODEL_GROUPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_model_group", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_model_group("id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

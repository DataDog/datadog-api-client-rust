// Create an AI Gateway model group returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayMaxReasoningLevel;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupBinding;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupBindingRelation;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupCreateData;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewayModelGroupType;

#[tokio::main]
async fn main() {
    let body = AiGatewayModelGroupCreateRequest::new(AiGatewayModelGroupCreateData::new(
        AiGatewayModelGroupCreateAttributes::new("Coding models".to_string())
            .allow_fast_mode(true)
            .bindings(vec![AiGatewayModelGroupBinding::new(
                vec!["user:55555555-5555-5555-5555-555555555555".to_string()],
                AiGatewayModelGroupBindingRelation::RUNNER,
            )])
            .max_reasoning_level(AiGatewayMaxReasoningLevel::HIGH)
            .model_ids(vec![101])
            .provider_ids(vec![])
            .routing_rule_slugs(vec!["coding-route".to_string()]),
        AiGatewayModelGroupType::MODEL_GROUPS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_ai_gateway_model_group", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.create_ai_gateway_model_group(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

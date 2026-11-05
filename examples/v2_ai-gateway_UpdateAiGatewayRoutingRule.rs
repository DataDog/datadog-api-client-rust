// Replace an AI Gateway routing rule returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayNullableRoutingRuleTierOrdering;
use datadog_api_client::datadogV2::model::AiGatewayRoutingReasoningLevel;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleReplaceAttributes;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleReplaceData;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleReplaceRequest;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleTargetRequest;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleTargetType;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleTierRequest;
use datadog_api_client::datadogV2::model::AiGatewayRoutingRuleType;

#[tokio::main]
async fn main() {
    let body = AiGatewayRoutingRuleReplaceRequest::new(AiGatewayRoutingRuleReplaceData::new(
        AiGatewayRoutingRuleReplaceAttributes::new(vec![AiGatewayRoutingRuleTierRequest::new(
            vec![
                AiGatewayRoutingRuleTargetRequest::new(AiGatewayRoutingRuleTargetType::MODEL)
                    .id(901)
                    .model("gpt-5.2-codex".to_string())
                    .model_id(101)
                    .provider("prod-us".to_string())
                    .reasoning_level(Some(AiGatewayRoutingReasoningLevel::MEDIUM))
                    .slug("coding-fallback".to_string())
                    .weight(Some(70)),
            ],
        )
        .ordering(Some(AiGatewayNullableRoutingRuleTierOrdering::CONFIGURED))]),
        "coding".to_string(),
        AiGatewayRoutingRuleType::ROUTING_RULES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_routing_rule", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_routing_rule("slug".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

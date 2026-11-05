// Create or replace an AI Gateway routing policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::UpsertAiGatewayPolicyOptionalParams;
use datadog_api_client::datadogV2::model::AiGatewayPolicyRequest;
use datadog_api_client::datadogV2::model::AiGatewayPolicyRequestAttributes;
use datadog_api_client::datadogV2::model::AiGatewayPolicyRequestData;
use datadog_api_client::datadogV2::model::AiGatewayPolicyTargetRequest;
use datadog_api_client::datadogV2::model::AiGatewayPolicyTierRequest;
use datadog_api_client::datadogV2::model::AiGatewayPolicyType;

#[tokio::main]
async fn main() {
    let body = AiGatewayPolicyRequest::new(AiGatewayPolicyRequestData::new(
        AiGatewayPolicyRequestAttributes::new(
            "coding-agent".to_string(),
            vec![
                AiGatewayPolicyTierRequest::new(vec![AiGatewayPolicyTargetRequest::new()
                    .id(901)
                    .model("gpt-5.2-codex".to_string())
                    .provider("prod-us".to_string())])
                .priority(1),
            ],
        )
        .priority(1),
        AiGatewayPolicyType::ROUTING_DOCUMENTS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.upsert_ai_gateway_policy", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .upsert_ai_gateway_policy(body, UpsertAiGatewayPolicyOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

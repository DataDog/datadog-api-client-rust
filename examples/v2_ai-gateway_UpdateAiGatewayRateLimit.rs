// Update an AI Gateway rate limit returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitMode;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitType;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitUpdateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitUpdateData;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayRateLimitUpdateRequest::new(AiGatewayRateLimitUpdateData::new(
        AiGatewayRateLimitUpdateAttributes::new()
            .cost_microcents_per_minute(None)
            .input_tokens_per_minute(None)
            .mode(AiGatewayRateLimitMode::ALERT)
            .output_tokens_per_minute(None)
            .requests_per_minute(None)
            .tags(None)
            .team_id(None)
            .user_id(None),
        Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("invalid UUID"),
        AiGatewayRateLimitType::RATE_LIMITS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_rate_limit", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_rate_limit(
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

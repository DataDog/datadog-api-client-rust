// Create an AI Gateway rate limit returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitCreateData;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitMode;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitSelectorEntry;
use datadog_api_client::datadogV2::model::AiGatewayRateLimitType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayRateLimitCreateRequest::new(AiGatewayRateLimitCreateData::new(
        AiGatewayRateLimitCreateAttributes::new("Default rate limit".to_string())
            .input_tokens_per_minute(None)
            .mode(AiGatewayRateLimitMode::ALERT)
            .output_tokens_per_minute(None)
            .requests_per_minute(Some(600))
            .selector(Some(vec![AiGatewayRateLimitSelectorEntry::new(
                "user_id".to_string(),
                true,
            )
            .values(vec!["11111111-1111-1111-1111-111111111111".to_string()])]))
            .team_id(Uuid::parse_str("00000000-0000-4000-8000-000000000002").expect("invalid UUID"))
            .user_id(
                Uuid::parse_str("00000000-0000-4000-8000-000000000001").expect("invalid UUID"),
            ),
        AiGatewayRateLimitType::RATE_LIMITS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_ai_gateway_rate_limit", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.create_ai_gateway_rate_limit(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

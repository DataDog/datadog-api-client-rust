// Update an AI Gateway budget returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayBudgetEnforcement;
use datadog_api_client::datadogV2::model::AiGatewayBudgetType;
use datadog_api_client::datadogV2::model::AiGatewayBudgetUpdateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayBudgetUpdateData;
use datadog_api_client::datadogV2::model::AiGatewayBudgetUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayBudgetUpdateRequest::new(AiGatewayBudgetUpdateData::new(
        AiGatewayBudgetUpdateAttributes::new()
            .apply_to_current_period(false)
            .capacity_microcents(120000000)
            .enabled(true)
            .enforcement(AiGatewayBudgetEnforcement::BLOCK)
            .name("Coding agents".to_string())
            .reason("Temporary capacity increase approved for an incident".to_string())
            .track_existing(false)
            .warning_threshold(0.8 as f64),
        Uuid::parse_str("11111111-1111-1111-1111-111111111111").expect("invalid UUID"),
        AiGatewayBudgetType::BUDGETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.update_ai_gateway_budget", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .update_ai_gateway_budget(
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

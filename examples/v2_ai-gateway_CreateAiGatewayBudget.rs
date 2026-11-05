// Create an AI Gateway budget returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayBudgetCreateAttributes;
use datadog_api_client::datadogV2::model::AiGatewayBudgetCreateData;
use datadog_api_client::datadogV2::model::AiGatewayBudgetCreateRequest;
use datadog_api_client::datadogV2::model::AiGatewayBudgetEnforcement;
use datadog_api_client::datadogV2::model::AiGatewayBudgetSelectorEntry;
use datadog_api_client::datadogV2::model::AiGatewayBudgetType;
use datadog_api_client::datadogV2::model::AiGatewayBudgetWindowPolicy;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = AiGatewayBudgetCreateRequest::new(AiGatewayBudgetCreateData::new(
        AiGatewayBudgetCreateAttributes::new(
            100000000,
            AiGatewayBudgetEnforcement::BLOCK,
            "Coding agents".to_string(),
            AiGatewayBudgetWindowPolicy::MONTHLY,
        )
        .selector(Some(vec![AiGatewayBudgetSelectorEntry::new(
            "user_id".to_string(),
            true,
        )
        .values(vec!["11111111-1111-1111-1111-111111111111".to_string()])]))
        .tags(BTreeMap::from([(
            "model".to_string(),
            "gpt-5.2-codex".to_string(),
        )]))
        .track_existing(false)
        .warning_threshold(0.9 as f64),
        AiGatewayBudgetType::BUDGETS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.create_ai_gateway_budget", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api.create_ai_gateway_budget(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

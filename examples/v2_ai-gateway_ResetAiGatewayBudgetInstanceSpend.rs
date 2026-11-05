// Reset a budget instance's spend returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::model::AiGatewayBudgetInstanceResetAttributes;
use datadog_api_client::datadogV2::model::AiGatewayBudgetInstanceResetData;
use datadog_api_client::datadogV2::model::AiGatewayBudgetInstanceResetRequest;
use datadog_api_client::datadogV2::model::AiGatewayBudgetInstanceType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = AiGatewayBudgetInstanceResetRequest::new(AiGatewayBudgetInstanceResetData::new(
        AiGatewayBudgetInstanceResetAttributes::new(
            "v1|7:user_id=36:22222222-2222-2222-2222-222222222222".to_string(),
        )
        .reason("Credit approved after incorrect usage attribution".to_string()),
        AiGatewayBudgetInstanceType::BUDGET_INSTANCES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.reset_ai_gateway_budget_instance_spend", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .reset_ai_gateway_budget_instance_spend(
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

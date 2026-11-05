// Get an AI Gateway budget instance summary returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::GetAiGatewayBudgetInstancesSummaryOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.get_ai_gateway_budget_instances_summary", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .get_ai_gateway_budget_instances_summary(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            GetAiGatewayBudgetInstancesSummaryOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

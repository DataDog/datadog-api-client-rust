// List AI Gateway budget instances returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::ListAiGatewayBudgetInstancesOptionalParams;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.list_ai_gateway_budget_instances", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .list_ai_gateway_budget_instances(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            ListAiGatewayBudgetInstancesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

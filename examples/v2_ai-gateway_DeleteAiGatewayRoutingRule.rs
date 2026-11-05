// Delete an AI Gateway routing rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::DeleteAiGatewayRoutingRuleOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_ai_gateway_routing_rule", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .delete_ai_gateway_routing_rule(
            "slug".to_string(),
            DeleteAiGatewayRoutingRuleOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

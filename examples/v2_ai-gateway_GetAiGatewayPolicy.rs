// Get an AI Gateway routing policy returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ai_gateway::AIGatewayAPI;
use datadog_api_client::datadogV2::api_ai_gateway::GetAiGatewayPolicyOptionalParams;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.get_ai_gateway_policy", true);
    let api = AIGatewayAPI::with_config(configuration);
    let resp = api
        .get_ai_gateway_policy(
            "inbound".to_string(),
            GetAiGatewayPolicyOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// List Agent Health issues returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_health::AgentHealthAPI;
use datadog_api_client::datadogV2::api_agent_health::ListAgentHealthIssuesOptionalParams;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AgentHealthAPI::with_config(configuration);
    let resp = api
        .list_agent_health_issues(ListAgentHealthIssuesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// List Agent Health issues returns "OK" response with pagination
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_health::AgentHealthAPI;
use datadog_api_client::datadogV2::api_agent_health::ListAgentHealthIssuesOptionalParams;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AgentHealthAPI::with_config(configuration);
    let response = api
        .list_agent_health_issues_with_pagination(ListAgentHealthIssuesOptionalParams::default());
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

// List active Agent Health issues returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_agent_health::AgentHealthAPI;
use datadog_api_client::datadogV2::api_agent_health::ListAgentHealthHostIssuesOptionalParams;
use datadog_api_client::datadogV2::model::AgentHealthAgentType;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = AgentHealthAPI::with_config(configuration);
    let resp = api
        .list_agent_health_host_issues(
            "123e4567-e89b-12d3-a456-426614174000".to_string(),
            ListAgentHealthHostIssuesOptionalParams::default()
                .agent_type(AgentHealthAgentType::NODE),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

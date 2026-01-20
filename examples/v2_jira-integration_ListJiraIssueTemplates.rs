// List Jira issue templates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_jira_integration::JiraIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListJiraIssueTemplates", true);
    let api = JiraIntegrationAPI::with_config(configuration);
    let resp = api.list_jira_issue_templates().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

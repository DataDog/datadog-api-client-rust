// Delete Jira issue template returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_jira_integration::JiraIntegrationAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteJiraIssueTemplate", true);
    let api = JiraIntegrationAPI::with_config(configuration);
    let resp = api
        .delete_jira_issue_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

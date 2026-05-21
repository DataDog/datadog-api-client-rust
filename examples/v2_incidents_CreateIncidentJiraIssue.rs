// Create an incident Jira issue returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_incidents::IncidentsAPI;
use datadog_api_client::datadogV2::model::IncidentJiraIssueDataAttributesRequest;
use datadog_api_client::datadogV2::model::IncidentJiraIssueDataRequest;
use datadog_api_client::datadogV2::model::IncidentJiraIssueRequest;
use datadog_api_client::datadogV2::model::IncidentJiraIssueType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = IncidentJiraIssueRequest::new(IncidentJiraIssueDataRequest::new(
        IncidentJiraIssueDataAttributesRequest::new(
            "123456".to_string(),
            "10001".to_string(),
            "10000".to_string(),
        )
        .template_id(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
        ),
        IncidentJiraIssueType::INCIDENT_JIRA_ISSUES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIncidentJiraIssue", true);
    let api = IncidentsAPI::with_config(configuration);
    let resp = api
        .create_incident_jira_issue("incident_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

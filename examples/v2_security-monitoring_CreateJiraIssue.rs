// Create Jira issues for security findings returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::FindingStatus;
use datadog_api_client::datadogV2::model::JiraIssueDataAttributesRequest;
use datadog_api_client::datadogV2::model::JiraIssueDataAttributesRequestMode;
use datadog_api_client::datadogV2::model::JiraIssueDataMeta;
use datadog_api_client::datadogV2::model::JiraIssueDataRequest;
use datadog_api_client::datadogV2::model::JiraIssueFinding;
use datadog_api_client::datadogV2::model::JiraIssueFindingId;
use datadog_api_client::datadogV2::model::JiraIssueRequest;
use datadog_api_client::datadogV2::model::JiraIssueType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = JiraIssueRequest::new(JiraIssueDataRequest::new(
        JiraIssueDataAttributesRequest::new(
            "f7ccdf99-0e22-4378-bdf9-03fde5379fea".to_string(),
            "story".to_string(),
            "1235".to_string(),
            "1234".to_string(),
            "SEC".to_string(),
        )
        .fields(BTreeMap::from([(
            "customfield_10000".to_string(),
            Value::from("Custom field value"),
        )]))
        .mode(JiraIssueDataAttributesRequestMode::SINGLE),
        "ID".to_string(),
        JiraIssueDataMeta::new()
            .findings(vec![JiraIssueFinding::new(
                "Description".to_string(),
                vec![JiraIssueFindingId::new(
                    123213123,
                    "afa-afa-hze".to_string(),
                    "Resource".to_string(),
                    "akjasd:asdsad".to_string(),
                )],
                "".to_string(),
                "Remediation".to_string(),
                FindingStatus::CRITICAL,
                "Title".to_string(),
                "ciem".to_string(),
            )
            .impacted(1)])
            .vulnerabilities(vec![JiraIssueFinding::new(
                "Description".to_string(),
                vec![JiraIssueFindingId::new(
                    123213123,
                    "afa-afa-hze".to_string(),
                    "Resource".to_string(),
                    "akjasd:asdsad".to_string(),
                )],
                "".to_string(),
                "Remediation".to_string(),
                FindingStatus::CRITICAL,
                "Title".to_string(),
                "ciem".to_string(),
            )
            .impacted(1)]),
        JiraIssueType::JIRA_ISSUE,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateJiraIssue", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.create_jira_issue(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

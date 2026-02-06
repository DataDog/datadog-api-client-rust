// Assign or un-assign Jira issues to security findings returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::IntegrationAssignmentDataAttributesRequest;
use datadog_api_client::datadogV2::model::IntegrationAssignmentDataAttributesRequestAction;
use datadog_api_client::datadogV2::model::IntegrationAssignmentDataAttributesRequestAssignment;
use datadog_api_client::datadogV2::model::IntegrationAssignmentDataAttributesRequestType;
use datadog_api_client::datadogV2::model::IntegrationAssignmentDataRequest;
use datadog_api_client::datadogV2::model::IntegrationAssignmentRequest;
use datadog_api_client::datadogV2::model::IntegrationAssignmentType;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        IntegrationAssignmentRequest::new(
            IntegrationAssignmentDataRequest::new(
                IntegrationAssignmentDataAttributesRequest::new(
                    IntegrationAssignmentDataAttributesRequestAction::ASSIGN,
                    IntegrationAssignmentDataAttributesRequestAssignment::new(
                        BTreeMap::from(
                            [
                                (
                                    "https://jira.example.com/browse/SEC-123".to_string(),
                                    vec![
                                        "MDBjMzdhYzgyNGZkZGJiZmY0OGNmYjNiMWQ2ODY0YmR-OTc0YjMzNjM1Y2UyODA2YTEyNWQxYmNkZjhmODllNzg=".to_string()
                                    ],
                                ),
                            ],
                        ),
                    ),
                    IntegrationAssignmentDataAttributesRequestType::FINDINGS,
                ),
                "some_id".to_string(),
                IntegrationAssignmentType::ISSUE_ASSIGNMENT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AssignIntegrationIssues", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.assign_integration_issues(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

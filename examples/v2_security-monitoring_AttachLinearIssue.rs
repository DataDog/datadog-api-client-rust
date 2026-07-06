// Attach security findings to a Linear issue returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AttachLinearIssueRequest;
use datadog_api_client::datadogV2::model::AttachLinearIssueRequestData;
use datadog_api_client::datadogV2::model::AttachLinearIssueRequestDataAttributes;
use datadog_api_client::datadogV2::model::AttachLinearIssueRequestDataRelationships;
use datadog_api_client::datadogV2::model::CaseManagementProject;
use datadog_api_client::datadogV2::model::CaseManagementProjectData;
use datadog_api_client::datadogV2::model::CaseManagementProjectDataType;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;
use datadog_api_client::datadogV2::model::LinearIssuesDataType;

#[tokio::main]
async fn main() {
    let body = AttachLinearIssueRequest::new(AttachLinearIssueRequestData::new(
        AttachLinearIssueRequestDataAttributes::new(
            "https://linear.app/your-workspace/issue/ENG-123".to_string(),
        ),
        AttachLinearIssueRequestDataRelationships::new(
            Findings::new().data(vec![FindingData::new(
                "ZGVmLTAwcC1pZXJ-aS0wZjhjNjMyZDNmMzRlZTgzNw==".to_string(),
                FindingDataType::FINDINGS,
            )]),
            CaseManagementProject::new(CaseManagementProjectData::new(
                "aeadc05e-98a8-11ec-ac2c-da7ad0900001".to_string(),
                CaseManagementProjectDataType::PROJECTS,
            )),
        ),
        LinearIssuesDataType::LINEAR_ISSUES,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.AttachLinearIssue", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.attach_linear_issue(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

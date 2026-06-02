// Assign or unassign security findings returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_security_monitoring::SecurityMonitoringAPI;
use datadog_api_client::datadogV2::model::AssigneeDataType;
use datadog_api_client::datadogV2::model::AssigneeRequest;
use datadog_api_client::datadogV2::model::AssigneeRequestData;
use datadog_api_client::datadogV2::model::AssigneeRequestDataAttributes;
use datadog_api_client::datadogV2::model::AssigneeRequestDataRelationships;
use datadog_api_client::datadogV2::model::FindingData;
use datadog_api_client::datadogV2::model::FindingDataType;
use datadog_api_client::datadogV2::model::Findings;

#[tokio::main]
async fn main() {
    let body = AssigneeRequest::new(
        AssigneeRequestData::new(
            AssigneeRequestDataRelationships::new(Findings::new().data(vec![FindingData::new(
                "ZGVmLTAwcC1pZXJ-aS0wZjhjNjMyZDNmMzRlZTgzNw==".to_string(),
                FindingDataType::FINDINGS,
            )])),
            AssigneeDataType::ASSIGNEE,
        )
        .attributes(
            AssigneeRequestDataAttributes::new()
                .assignee_id("f315bdaf-9ee7-4808-a9c1-99c15bf0f4d0".to_string()),
        )
        .id("00000000-0000-0000-0000-000000000001".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateFindingsAssignee", true);
    let api = SecurityMonitoringAPI::with_config(configuration);
    let resp = api.update_findings_assignee(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

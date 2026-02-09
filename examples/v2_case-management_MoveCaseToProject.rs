// Update case project returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::ProjectRelationship;
use datadog_api_client::datadogV2::model::ProjectRelationshipData;
use datadog_api_client::datadogV2::model::ProjectResourceType;

#[tokio::main]
async fn main() {
    let body = ProjectRelationship::new(ProjectRelationshipData::new(
        "e555e290-ed65-49bd-ae18-8acbfcf18db7".to_string(),
        ProjectResourceType::PROJECT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.MoveCaseToProject", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.move_case_to_project("case_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Update a project returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::ProjectResourceType;
use datadog_api_client::datadogV2::model::ProjectUpdate;
use datadog_api_client::datadogV2::model::ProjectUpdateAttributes;
use datadog_api_client::datadogV2::model::ProjectUpdateRequest;

#[tokio::main]
async fn main() {
    let body = ProjectUpdateRequest::new(
        ProjectUpdate::new(ProjectResourceType::PROJECT).attributes(
            ProjectUpdateAttributes::new()
                .name("Updated Project Name Example-Case-Management".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_project("d4bbe1af-f36e-42f1-87c1-493ca35c320e".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

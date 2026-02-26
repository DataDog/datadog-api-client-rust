// Create maintenance returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateMaintenanceOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateMaintenanceRequest;
use datadog_api_client::datadogV2::model::CreateMaintenanceRequestData;
use datadog_api_client::datadogV2::model::CreateMaintenanceRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateMaintenanceRequestDataAttributesComponentsAffectedItems;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequestDataAttributesComponentsAffectedItemsStatus;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_attributes_components_0_components_0_id = uuid::Uuid::parse_str(
        &std::env::var("STATUS_PAGE_DATA_ATTRIBUTES_COMPONENTS_0_COMPONENTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body = CreateMaintenanceRequest::new().data(CreateMaintenanceRequestData::new(
        CreateMaintenanceRequestDataAttributes::new(
            vec![
                CreateMaintenanceRequestDataAttributesComponentsAffectedItems::new(
                    status_page_data_attributes_components_0_components_0_id.clone(),
                    PatchMaintenanceRequestDataAttributesComponentsAffectedItemsStatus::OPERATIONAL,
                ),
            ],
            "API Maintenance".to_string(),
        )
        .completed_date(
            DateTime::parse_from_rfc3339("2021-11-11T13:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        )
        .completed_description(
            "We have completed maintenance on the API to improve performance.".to_string(),
        )
        .in_progress_description(
            "We are currently performing maintenance on the API to improve performance."
                .to_string(),
        )
        .scheduled_description(
            "We will be performing maintenance on the API to improve performance.".to_string(),
        )
        .start_date(
            DateTime::parse_from_rfc3339("2021-11-11T12:11:11+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        PatchMaintenanceRequestDataType::MAINTENANCES,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_maintenance(
            status_page_data_id.clone(),
            body,
            CreateMaintenanceOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

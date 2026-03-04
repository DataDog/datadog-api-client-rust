// Update maintenance returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateMaintenanceOptionalParams;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequest;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequestData;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchMaintenanceRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");

    // there is a valid "maintenance" in the system
    let maintenance_data_id = uuid::Uuid::parse_str(&std::env::var("MAINTENANCE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body =
        PatchMaintenanceRequest
        ::new().data(
            PatchMaintenanceRequestData::new(
                PatchMaintenanceRequestDataAttributes::new()
                    .in_progress_description(
                        "We are currently performing maintenance on the API to improve performance for 40 minutes.".to_string(),
                    )
                    .scheduled_description(
                        "We will be performing maintenance on the API to improve performance for 40 minutes.".to_string(),
                    ),
                maintenance_data_id.clone(),
                PatchMaintenanceRequestDataType::MAINTENANCES,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_maintenance(
            status_page_data_id.clone(),
            maintenance_data_id.clone(),
            body,
            UpdateMaintenanceOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

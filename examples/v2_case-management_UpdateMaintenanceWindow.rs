// Update a maintenance window returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::MaintenanceWindowResourceType;
use datadog_api_client::datadogV2::model::MaintenanceWindowUpdate;
use datadog_api_client::datadogV2::model::MaintenanceWindowUpdateAttributes;
use datadog_api_client::datadogV2::model::MaintenanceWindowUpdateRequest;

#[tokio::main]
async fn main() {
    let body = MaintenanceWindowUpdateRequest::new(
        MaintenanceWindowUpdate::new(MaintenanceWindowResourceType::MAINTENANCE_WINDOW)
            .attributes(MaintenanceWindowUpdateAttributes::new()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateMaintenanceWindow", true);
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api
        .update_maintenance_window("maintenance_window_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

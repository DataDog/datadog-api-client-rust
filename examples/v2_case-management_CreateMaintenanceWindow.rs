// Create a maintenance window returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::MaintenanceWindowCreate;
use datadog_api_client::datadogV2::model::MaintenanceWindowCreateAttributes;
use datadog_api_client::datadogV2::model::MaintenanceWindowCreateRequest;
use datadog_api_client::datadogV2::model::MaintenanceWindowResourceType;

#[tokio::main]
async fn main() {
    let body = MaintenanceWindowCreateRequest::new(MaintenanceWindowCreate::new(
        MaintenanceWindowCreateAttributes::new(
            DateTime::parse_from_rfc3339("2026-06-01T06:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
            "Weekly maintenance".to_string(),
            "project:SEC".to_string(),
            DateTime::parse_from_rfc3339("2026-06-01T00:00:00+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        MaintenanceWindowResourceType::MAINTENANCE_WINDOW,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.create_maintenance_window(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Schedule maintenance returns "Created" response
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
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateMaintenanceRequest::new().data(CreateMaintenanceRequestData::new(
        CreateMaintenanceRequestDataAttributes::new(
            vec![
                CreateMaintenanceRequestDataAttributesComponentsAffectedItems::new(
                    Uuid::parse_str("1234abcd-12ab-34cd-56ef-123456abcdef").expect("invalid UUID"),
                    PatchMaintenanceRequestDataAttributesComponentsAffectedItemsStatus::OPERATIONAL,
                ),
            ],
            "API Maintenance".to_string(),
        )
        .completed_date(
            DateTime::parse_from_rfc3339("2026-02-18T19:51:13.332360+00:00")
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
            DateTime::parse_from_rfc3339("2026-02-18T19:21:13.332360+00:00")
                .expect("Failed to parse datetime")
                .with_timezone(&Utc),
        ),
        PatchMaintenanceRequestDataType::MAINTENANCES,
    ));
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_maintenance(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
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

// Create backfilled maintenance returns "Created" response
use chrono::{DateTime, Utc};
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateBackfilledMaintenanceOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateBackfilledMaintenanceRequest;
use datadog_api_client::datadogV2::model::CreateBackfilledMaintenanceRequestData;
use datadog_api_client::datadogV2::model::CreateBackfilledMaintenanceRequestDataAttributes;
use datadog_api_client::datadogV2::model::CreateBackfilledMaintenanceRequestDataAttributesUpdatesItems;
use datadog_api_client::datadogV2::model::CreateBackfilledMaintenanceRequestDataAttributesUpdatesItemsComponentsAffectedItems;
use datadog_api_client::datadogV2::model::CreateMaintenanceRequestDataAttributesUpdatesItemsStatus;
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
    let body =
        CreateBackfilledMaintenanceRequest
        ::new().data(
            CreateBackfilledMaintenanceRequestData::new(
                PatchMaintenanceRequestDataType::MAINTENANCES,
            ).attributes(
                CreateBackfilledMaintenanceRequestDataAttributes::new(
                    "Past Database Maintenance".to_string(),
                    vec![
                        CreateBackfilledMaintenanceRequestDataAttributesUpdatesItems::new(
                            DateTime::parse_from_rfc3339("2021-11-11T10:11:11+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                        )
                            .components_affected(
                                vec![
                                    CreateBackfilledMaintenanceRequestDataAttributesUpdatesItemsComponentsAffectedItems
                                    ::new(
                                        status_page_data_attributes_components_0_components_0_id.clone(),
                                        PatchMaintenanceRequestDataAttributesComponentsAffectedItemsStatus
                                        ::MAINTENANCE,
                                    )
                                ],
                            )
                            .description("Database maintenance is in progress.".to_string())
                            .status(CreateMaintenanceRequestDataAttributesUpdatesItemsStatus::IN_PROGRESS),
                        CreateBackfilledMaintenanceRequestDataAttributesUpdatesItems::new(
                            DateTime::parse_from_rfc3339("2021-11-11T11:11:11+00:00")
                                .expect("Failed to parse datetime")
                                .with_timezone(&Utc),
                        )
                            .components_affected(
                                vec![
                                    CreateBackfilledMaintenanceRequestDataAttributesUpdatesItemsComponentsAffectedItems
                                    ::new(
                                        status_page_data_attributes_components_0_components_0_id.clone(),
                                        PatchMaintenanceRequestDataAttributesComponentsAffectedItemsStatus
                                        ::OPERATIONAL,
                                    )
                                ],
                            )
                            .description("Database maintenance has been completed successfully.".to_string())
                            .status(CreateMaintenanceRequestDataAttributesUpdatesItemsStatus::COMPLETED)
                    ],
                ),
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_backfilled_maintenance(
            status_page_data_id.clone(),
            body,
            CreateBackfilledMaintenanceOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

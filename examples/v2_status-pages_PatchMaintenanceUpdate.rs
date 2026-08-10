// Edit maintenance update returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::PatchMaintenanceUpdateRequest;
use datadog_api_client::datadogV2::model::PatchMaintenanceUpdateRequestData;
use datadog_api_client::datadogV2::model::PatchMaintenanceUpdateRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchMaintenanceUpdateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PatchMaintenanceUpdateRequest::new().data(
        PatchMaintenanceUpdateRequestData::new(
            "00000000-0000-0000-0000-000000000000".to_string(),
            PatchMaintenanceUpdateRequestDataType::MAINTENANCE_UPDATES,
        )
        .attributes(
            PatchMaintenanceUpdateRequestDataAttributes::new().description(
                "We have completed maintenance on the API to improve performance.".to_string(),
            ),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .patch_maintenance_update(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

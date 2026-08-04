// Update maintenance template returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateMaintenanceTemplateOptionalParams;
use datadog_api_client::datadogV2::model::PatchMaintenanceTemplateRequest;
use datadog_api_client::datadogV2::model::PatchMaintenanceTemplateRequestData;
use datadog_api_client::datadogV2::model::PatchMaintenanceTemplateRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchMaintenanceTemplateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = PatchMaintenanceTemplateRequest::new().data(
        PatchMaintenanceTemplateRequestData::new(
            "".to_string(),
            PatchMaintenanceTemplateRequestDataType::MAINTENANCE_TEMPLATES,
        )
        .attributes(PatchMaintenanceTemplateRequestDataAttributes::new().component_ids(vec![])),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_maintenance_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            UpdateMaintenanceTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

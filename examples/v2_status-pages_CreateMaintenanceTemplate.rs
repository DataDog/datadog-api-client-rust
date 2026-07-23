// Create maintenance template returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::CreateMaintenanceTemplateOptionalParams;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::model::CreateMaintenanceTemplateRequest;
use datadog_api_client::datadogV2::model::CreateMaintenanceTemplateRequestData;
use datadog_api_client::datadogV2::model::CreateMaintenanceTemplateRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchMaintenanceTemplateRequestDataType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateMaintenanceTemplateRequest::new().data(
        CreateMaintenanceTemplateRequestData::new(
            PatchMaintenanceTemplateRequestDataType::MAINTENANCE_TEMPLATES,
        )
        .attributes(
            CreateMaintenanceTemplateRequestDataAttributes::new("".to_string())
                .component_ids(vec![]),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .create_maintenance_template(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
            CreateMaintenanceTemplateOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

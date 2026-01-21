// Update component returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateComponentOptionalParams;
use datadog_api_client::datadogV2::model::PatchComponentRequest;
use datadog_api_client::datadogV2::model::PatchComponentRequestData;
use datadog_api_client::datadogV2::model::PatchComponentRequestDataAttributes;
use datadog_api_client::datadogV2::model::StatusPagesComponentGroupType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_attributes_components_0_id = uuid::Uuid::parse_str(
        &std::env::var("STATUS_PAGE_DATA_ATTRIBUTES_COMPONENTS_0_ID").unwrap(),
    )
    .expect("Invalid UUID");
    let status_page_data_id = uuid::Uuid::parse_str(&std::env::var("STATUS_PAGE_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let body = PatchComponentRequest::new().data(
        PatchComponentRequestData::new(StatusPagesComponentGroupType::COMPONENTS)
            .attributes(
                PatchComponentRequestDataAttributes::new().name("Logs Indexing".to_string()),
            )
            .id(status_page_data_attributes_components_0_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_component(
            status_page_data_id.clone(),
            status_page_data_attributes_components_0_id.clone(),
            body,
            UpdateComponentOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

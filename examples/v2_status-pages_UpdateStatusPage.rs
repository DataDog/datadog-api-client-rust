// Update status page returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateStatusPageOptionalParams;
use datadog_api_client::datadogV2::model::PatchStatusPageRequest;
use datadog_api_client::datadogV2::model::PatchStatusPageRequestData;
use datadog_api_client::datadogV2::model::PatchStatusPageRequestDataAttributes;
use datadog_api_client::datadogV2::model::StatusPageDataType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();
    let body = PatchStatusPageRequest::new().data(
        PatchStatusPageRequestData::new(StatusPageDataType::STATUS_PAGES)
            .attributes(
                PatchStatusPageRequestDataAttributes::new()
                    .name("[DD Integration Tests] 5e2fd69be33e79aa".to_string()),
            )
            .id(status_page_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_status_page(
            status_page_data_id.clone(),
            body,
            UpdateStatusPageOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

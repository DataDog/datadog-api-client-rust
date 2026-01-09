// Update degradation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_status_pages::StatusPagesAPI;
use datadog_api_client::datadogV2::api_status_pages::UpdateDegradationOptionalParams;
use datadog_api_client::datadogV2::model::PatchDegradationRequest;
use datadog_api_client::datadogV2::model::PatchDegradationRequestData;
use datadog_api_client::datadogV2::model::PatchDegradationRequestDataAttributes;
use datadog_api_client::datadogV2::model::PatchDegradationRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "status_page" in the system
    let status_page_data_id = std::env::var("STATUS_PAGE_DATA_ID").unwrap();

    // there is a valid "degradation" in the system
    let degradation_data_id = std::env::var("DEGRADATION_DATA_ID").unwrap();
    let body = PatchDegradationRequest::new().data(
        PatchDegradationRequestData::new(PatchDegradationRequestDataType::DEGRADATIONS)
            .attributes(
                PatchDegradationRequestDataAttributes::new().title("5e2fd69be33e79aa".to_string()),
            )
            .id(degradation_data_id.clone()),
    );
    let configuration = datadog::Configuration::new();
    let api = StatusPagesAPI::with_config(configuration);
    let resp = api
        .update_degradation(
            status_page_data_id.clone(),
            degradation_data_id.clone(),
            body,
            UpdateDegradationOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// List annotations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_annotations::AnnotationsAPI;
use datadog_api_client::datadogV2::api_annotations::ListAnnotationsOptionalParams;

#[tokio::main]
async fn main() {
    // there is a valid "annotation" in the system
    let annotation_data_attributes_page_id =
        std::env::var("ANNOTATION_DATA_ATTRIBUTES_PAGE_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListAnnotations", true);
    let api = AnnotationsAPI::with_config(configuration);
    let resp = api
        .list_annotations(
            annotation_data_attributes_page_id.clone(),
            1704067200000,
            1704153600000,
            ListAnnotationsOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Delete an annotation returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_annotations::AnnotationsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "annotation" in the system
    let annotation_data_id =
        uuid::Uuid::parse_str(&std::env::var("ANNOTATION_DATA_ID").unwrap()).expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteAnnotation", true);
    let api = AnnotationsAPI::with_config(configuration);
    let resp = api.delete_annotation(annotation_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

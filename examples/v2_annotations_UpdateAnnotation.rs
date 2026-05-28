// Update an annotation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_annotations::AnnotationsAPI;
use datadog_api_client::datadogV2::model::AnnotationColor;
use datadog_api_client::datadogV2::model::AnnotationCreateAttributes;
use datadog_api_client::datadogV2::model::AnnotationKind;
use datadog_api_client::datadogV2::model::AnnotationRequestData;
use datadog_api_client::datadogV2::model::AnnotationType;
use datadog_api_client::datadogV2::model::AnnotationUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "annotation" in the system
    let annotation_data_id =
        uuid::Uuid::parse_str(&std::env::var("ANNOTATION_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = AnnotationUpdateRequest::new(AnnotationRequestData::new(
        AnnotationCreateAttributes::new(
            AnnotationColor::GREEN,
            "Updated annotation.".to_string(),
            "dashboard:abc-def-xyz".to_string(),
            1704067200000,
            AnnotationKind::POINT_IN_TIME,
        ),
        AnnotationType::ANNOTATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateAnnotation", true);
    let api = AnnotationsAPI::with_config(configuration);
    let resp = api
        .update_annotation(annotation_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

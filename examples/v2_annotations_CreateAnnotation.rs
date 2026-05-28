// Create an annotation returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_annotations::AnnotationsAPI;
use datadog_api_client::datadogV2::model::AnnotationColor;
use datadog_api_client::datadogV2::model::AnnotationCreateAttributes;
use datadog_api_client::datadogV2::model::AnnotationCreateRequest;
use datadog_api_client::datadogV2::model::AnnotationKind;
use datadog_api_client::datadogV2::model::AnnotationRequestData;
use datadog_api_client::datadogV2::model::AnnotationType;

#[tokio::main]
async fn main() {
    let body = AnnotationCreateRequest::new(AnnotationRequestData::new(
        AnnotationCreateAttributes::new(
            AnnotationColor::BLUE,
            "Deployed v2.3.1 to production.".to_string(),
            "dashboard:abc-def-xyz".to_string(),
            1704067200000,
            AnnotationKind::POINT_IN_TIME,
        )
        .widget_ids(vec!["1234567890".to_string()]),
        AnnotationType::ANNOTATION,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAnnotation", true);
    let api = AnnotationsAPI::with_config(configuration);
    let resp = api.create_annotation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Publish a form version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormPublicationType;
use datadog_api_client::datadogV2::model::PublishFormData;
use datadog_api_client::datadogV2::model::PublishFormDataAttributes;
use datadog_api_client::datadogV2::model::PublishFormRequest;

#[tokio::main]
async fn main() {
    // there is a valid "form" in the system
    let form_data_id =
        uuid::Uuid::parse_str(&std::env::var("FORM_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = PublishFormRequest::new(PublishFormData::new(
        PublishFormDataAttributes::new(1),
        FormPublicationType::FORM_PUBLICATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PublishForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.publish_form(form_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

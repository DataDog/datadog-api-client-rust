// Publish a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormPublicationAttributes;
use datadog_api_client::datadogV2::model::FormPublicationDataRequest;
use datadog_api_client::datadogV2::model::FormPublicationRequest;
use datadog_api_client::datadogV2::model::FormPublicationType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = FormPublicationRequest::new(FormPublicationDataRequest::new(
        FormPublicationAttributes::new(1),
        FormPublicationType::FORM_PUBLICATIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.PublishForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .publish_form(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Submit a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormSubmissionAttributes;
use datadog_api_client::datadogV2::model::FormSubmissionDataRequest;
use datadog_api_client::datadogV2::model::FormSubmissionRequest;
use datadog_api_client::datadogV2::model::FormSubmissionType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = FormSubmissionRequest::new(FormSubmissionDataRequest::new(
        FormSubmissionAttributes::new(BTreeMap::new()),
        FormSubmissionType::FORM_SUBMISSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.SubmitForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .submit_form(
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

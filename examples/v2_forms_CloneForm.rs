// Clone a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::CloneFormData;
use datadog_api_client::datadogV2::model::CloneFormDataAttributes;
use datadog_api_client::datadogV2::model::CloneFormRequest;
use datadog_api_client::datadogV2::model::FormType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CloneFormRequest::new(
        CloneFormData::new(FormType::FORMS)
            .attributes(CloneFormDataAttributes::new().name("Copy of My Form".to_string())),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CloneForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .clone_form(
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

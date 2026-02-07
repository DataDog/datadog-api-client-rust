// Create a new form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormCreateRequest;
use datadog_api_client::datadogV2::model::FormDataAttributesRequest;
use datadog_api_client::datadogV2::model::FormDataRequest;
use datadog_api_client::datadogV2::model::FormType;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = FormCreateRequest::new(
        FormDataRequest::new(
            FormDataAttributesRequest::new(
                BTreeMap::new(),
                "test description".to_string(),
                "test form happy path".to_string(),
                BTreeMap::new(),
            ),
            FormType::FORMS,
        )
        .id(Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID")),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.create_form(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

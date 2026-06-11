// Update a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormDatastoreConfigAttributes;
use datadog_api_client::datadogV2::model::FormType;
use datadog_api_client::datadogV2::model::FormUpdateAttributes;
use datadog_api_client::datadogV2::model::UpdateFormData;
use datadog_api_client::datadogV2::model::UpdateFormDataAttributes;
use datadog_api_client::datadogV2::model::UpdateFormRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // there is a valid "form" in the system
    let form_data_id =
        uuid::Uuid::parse_str(&std::env::var("FORM_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = UpdateFormRequest::new(
        UpdateFormData::new(
            UpdateFormDataAttributes::new(
                FormUpdateAttributes::new()
                    .datastore_config(FormDatastoreConfigAttributes::new(
                        Uuid::parse_str("5108ea24-dd83-4696-9caa-f069f73d0fad")
                            .expect("invalid UUID"),
                        "id".to_string(),
                        "none".to_string(),
                    ))
                    .description("An updated description.".to_string())
                    .name("Updated Form Name".to_string()),
            ),
            FormType::FORMS,
        )
        .id(form_data_id.clone()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.update_form(form_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

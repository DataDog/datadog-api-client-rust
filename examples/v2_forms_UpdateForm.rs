// Update a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormType;
use datadog_api_client::datadogV2::model::FormUpdateAttributes;
use datadog_api_client::datadogV2::model::FormUpdateAttributesFormUpdate;
use datadog_api_client::datadogV2::model::FormUpdateDataRequest;
use datadog_api_client::datadogV2::model::FormUpdateRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = FormUpdateRequest::new(
        FormUpdateDataRequest::new(
            FormUpdateAttributes::new().form_update(
                FormUpdateAttributesFormUpdate::new()
                    .description("Updated description".to_string())
                    .name("New Form Name".to_string()),
            ),
            FormType::FORMS,
        )
        .id(Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID")),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .update_form(
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

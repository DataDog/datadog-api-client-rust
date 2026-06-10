// Create and publish a form returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::CreateFormData;
use datadog_api_client::datadogV2::model::CreateFormDataAttributes;
use datadog_api_client::datadogV2::model::CreateFormRequest;
use datadog_api_client::datadogV2::model::FormDataDefinition;
use datadog_api_client::datadogV2::model::FormType;
use datadog_api_client::datadogV2::model::FormUiDefinition;

#[tokio::main]
async fn main() {
    let body = CreateFormRequest::new(CreateFormData::new(
        CreateFormDataAttributes::new(
            FormDataDefinition::new(),
            "User Feedback Form".to_string(),
            FormUiDefinition::new(),
        )
        .anonymous(false)
        .description("A form to collect user feedback.".to_string())
        .idp_survey(false)
        .single_response(false),
        FormType::FORMS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateAndPublishForm", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api.create_and_publish_form(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

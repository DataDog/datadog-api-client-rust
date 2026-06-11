// Upsert and publish a form version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormDataDefinition;
use datadog_api_client::datadogV2::model::FormDataDefinitionType;
use datadog_api_client::datadogV2::model::FormUiDefinition;
use datadog_api_client::datadogV2::model::FormUiDefinitionUiTheme;
use datadog_api_client::datadogV2::model::FormUiDefinitionUiThemePrimaryColor;
use datadog_api_client::datadogV2::model::FormVersionType;
use datadog_api_client::datadogV2::model::UpsertAndPublishFormVersionData;
use datadog_api_client::datadogV2::model::UpsertAndPublishFormVersionDataAttributes;
use datadog_api_client::datadogV2::model::UpsertAndPublishFormVersionRequest;
use datadog_api_client::datadogV2::model::UpsertAndPublishFormVersionUpsertParams;

#[tokio::main]
async fn main() {
    // there is a valid "form" in the system
    let form_data_id =
        uuid::Uuid::parse_str(&std::env::var("FORM_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = UpsertAndPublishFormVersionRequest::new(UpsertAndPublishFormVersionData::new(
        UpsertAndPublishFormVersionDataAttributes::new(
            FormDataDefinition::new()
                .description("Welcome to the Engineering Experience Survey.".to_string())
                .required(vec![])
                .title("Developer Experience Survey".to_string())
                .type_(FormDataDefinitionType::OBJECT),
            FormUiDefinition::new().ui_order(vec![]).ui_theme(
                FormUiDefinitionUiTheme::new()
                    .primary_color(FormUiDefinitionUiThemePrimaryColor::GRAY),
            ),
            UpsertAndPublishFormVersionUpsertParams::new(
                "b51f08b698d88d8027a935d9db649774949f5fb41a0c559bfee6a9a13225c72d".to_string(),
            ),
        ),
        FormVersionType::FORM_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertAndPublishFormVersion", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .upsert_and_publish_form_version(form_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

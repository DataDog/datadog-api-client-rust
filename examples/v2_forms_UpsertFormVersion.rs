// Create a form version returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_forms::FormsAPI;
use datadog_api_client::datadogV2::model::FormVersionAttributes;
use datadog_api_client::datadogV2::model::FormVersionDataRequest;
use datadog_api_client::datadogV2::model::FormVersionRequest;
use datadog_api_client::datadogV2::model::FormVersionState;
use datadog_api_client::datadogV2::model::FormVersionType;
use datadog_api_client::datadogV2::model::FormVersionUpsertParams;
use std::collections::BTreeMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = FormVersionRequest::new(FormVersionDataRequest::new(
        FormVersionAttributes::new(
            BTreeMap::new(),
            BTreeMap::new(),
            FormVersionUpsertParams::new()
                .etag(
                    "b51f08b698d88d8027a935d9db649774949f5fb41a0c559bfee6a9a13225c72d".to_string(),
                )
                .match_policy("none".to_string()),
        )
        .state(FormVersionState::DRAFT),
        FormVersionType::FORM_VERSIONS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertFormVersion", true);
    let api = FormsAPI::with_config(configuration);
    let resp = api
        .upsert_form_version(
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

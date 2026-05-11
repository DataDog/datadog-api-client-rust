// Update a web integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_web_integrations::WebIntegrationsAPI;
use datadog_api_client::datadogV2::model::WebIntegrationAccountType;
use datadog_api_client::datadogV2::model::WebIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::WebIntegrationAccountUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::WebIntegrationAccountUpdateRequestData;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        WebIntegrationAccountUpdateRequest::new(WebIntegrationAccountUpdateRequestData::new(
            WebIntegrationAccountUpdateRequestAttributes::new()
                .name("my-databricks-account".to_string())
                .secrets(BTreeMap::from([(
                    "client_secret".to_string(),
                    Value::from("my-client-secret"),
                )]))
                .settings(BTreeMap::from([(
                    "workspace_url".to_string(),
                    Value::from("https://example.azuredatabricks.net"),
                )])),
            WebIntegrationAccountType::ACCOUNT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateWebIntegrationAccount", true);
    let api = WebIntegrationsAPI::with_config(configuration);
    let resp = api
        .update_web_integration_account(
            "integration_name".to_string(),
            "account_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

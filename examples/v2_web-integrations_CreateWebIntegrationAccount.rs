// Create a web integration account returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_web_integrations::WebIntegrationsAPI;
use datadog_api_client::datadogV2::model::WebIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::WebIntegrationAccountCreateRequestAttributes;
use datadog_api_client::datadogV2::model::WebIntegrationAccountCreateRequestData;
use datadog_api_client::datadogV2::model::WebIntegrationAccountType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        WebIntegrationAccountCreateRequest::new(WebIntegrationAccountCreateRequestData::new(
            WebIntegrationAccountCreateRequestAttributes::new(
                "my-databricks-account".to_string(),
                BTreeMap::from([("client_secret".to_string(), Value::from("my-client-secret"))]),
                BTreeMap::from([(
                    "workspace_url".to_string(),
                    Value::from("https://example.azuredatabricks.net"),
                )]),
            ),
            WebIntegrationAccountType::ACCOUNT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateWebIntegrationAccount", true);
    let api = WebIntegrationsAPI::with_config(configuration);
    let resp = api
        .create_web_integration_account("integration_name".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

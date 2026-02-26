// Create or update IDP configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_software_catalog::SoftwareCatalogAPI;
use datadog_api_client::datadogV2::model::IDPConfigRequest;
use datadog_api_client::datadogV2::model::IDPConfigRequestAttributes;
use datadog_api_client::datadogV2::model::IDPConfigRequestData;
use datadog_api_client::datadogV2::model::IDPConfigType;
use serde_json::Value;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = IDPConfigRequest::new(IDPConfigRequestData::new(
        IDPConfigRequestAttributes::new(vec![BTreeMap::from([
            ("displayName".to_string(), Value::from("My Dashboard")),
            ("id".to_string(), Value::from("dashboard-1")),
        ])]),
        IDPConfigType::IDP_CONFIG,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpsertIDPConfigValue", true);
    let api = SoftwareCatalogAPI::with_config(configuration);
    let resp = api
        .upsert_idp_config_value("idp_pinned_dashboards".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

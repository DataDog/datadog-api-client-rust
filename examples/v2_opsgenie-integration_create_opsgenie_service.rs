// Create a new service object returns "CREATED" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_opsgenie_integration::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = OpsgenieServiceCreateRequest::new(OpsgenieServiceCreateData::new(
        OpsgenieServiceCreateAttributes::new(
            "Example-Opsgenie-Integration".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            OpsgenieServiceRegionType::US,
        ),
        OpsgenieServiceType::OPSGENIE_SERVICE,
    ));
    let configuration = Configuration::new();
    let api = OpsgenieIntegrationAPI::with_config(configuration);
    let resp = api.create_opsgenie_service(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

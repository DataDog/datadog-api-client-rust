// List roles returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_roles::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "role" in the system
    let role_data_attributes_name = std::env::var("ROLE_DATA_ATTRIBUTES_NAME").unwrap();
    let configuration = Configuration::new();
    let api = RolesAPI::with_config(configuration);
    let resp = api
        .list_roles(ListRolesOptionalParams::default().filter(role_data_attributes_name.clone()))
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

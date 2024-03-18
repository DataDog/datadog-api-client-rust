// Get a notebook returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_notebooks::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "notebook" in the system
    let notebook_data_id: i64 = std::env::var("NOTEBOOK_DATA_ID").unwrap().parse().unwrap();
    let configuration = Configuration::new();
    let api = NotebooksAPI::with_config(configuration);
    let resp = api.get_notebook(notebook_data_id.clone()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

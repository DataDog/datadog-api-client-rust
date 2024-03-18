// Get all processes returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_processes::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = ProcessesAPI::with_config(configuration);
    let resp = api
        .list_processes(
            ListProcessesOptionalParams::default()
                .search("process-agent".to_string())
                .tags("testing:true".to_string())
                .page_limit(2),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

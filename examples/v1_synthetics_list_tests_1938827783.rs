// Get the list of all Synthetic tests returns "OK - Returns the list of all
// Synthetic tests." response with pagination
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::*;
use datadog_api_client::datadogV1::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let response = api.list_tests_with_pagination(ListTestsOptionalParams::default().page_size(2));
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

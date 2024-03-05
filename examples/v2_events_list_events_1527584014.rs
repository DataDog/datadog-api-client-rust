// Get a list of events returns "OK" response with pagination
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_events::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = EventsAPI::with_config(configuration);
    let resp =
        api
            .list_events(
                ListEventsOptionalParams::default()
                    .filter_from("now-15m".to_string())
                    .filter_to("now".to_string())
                    .page_limit(2),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

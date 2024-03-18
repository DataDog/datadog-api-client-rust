// Get a list of tests events returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_tests::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CIVisibilityTestsAPI::with_config(configuration);
    let resp = api
        .list_ci_app_test_events(
            ListCIAppTestEventsOptionalParams::default()
                .filter_query("@test.service:web-ui-tests".to_string())
                .filter_from(
                    DateTime::parse_from_rfc3339("2021-11-11T11:10:41+00:00")
                        .expect("Failed to parse datetime"),
                )
                .filter_to(
                    DateTime::parse_from_rfc3339("2021-11-11T11:11:11+00:00")
                        .expect("Failed to parse datetime"),
                )
                .page_limit(5),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

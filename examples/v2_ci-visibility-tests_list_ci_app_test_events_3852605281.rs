// Get a list of tests events returns "OK" response with pagination
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_tests::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = CIVisibilityTestsAPI::with_config(configuration);
    let resp =
        api
            .list_ci_app_test_events(
                ListCIAppTestEventsOptionalParams::default()
                    .filter_from((Utc::now() + chrono::Duration::seconds(-30)).to_rfc3339())
                    .filter_to((Utc::now()).to_rfc3339())
                    .page_limit(2),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

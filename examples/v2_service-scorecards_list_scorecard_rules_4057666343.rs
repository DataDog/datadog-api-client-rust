// List all rules returns "OK" response with pagination
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_scorecards::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListScorecardRules", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp =
        api
            .list_scorecard_rules(
                ListScorecardRulesOptionalParams::default()
                    .page_size(2)
                    .fields_rule("name".to_string())
                    .filter_rule_custom(true),
            )
            .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

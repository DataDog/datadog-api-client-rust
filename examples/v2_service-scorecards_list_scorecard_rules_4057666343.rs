// List all rules returns "OK" response with pagination
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_scorecards::*;
use datadog_api_client::datadogV2::model::*;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let mut configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListScorecardRules", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let response = api.list_scorecard_rules_with_pagination(
        ListScorecardRulesOptionalParams::default()
            .page_size(2)
            .fields_rule("name".to_string())
            .filter_rule_custom(true),
    );
    pin_mut!(response);
    while let Some(resp) = response.next().await {
        if let Ok(value) = resp {
            println!("{:#?}", value);
        } else {
            println!("{:#?}", resp.unwrap_err());
        }
    }
}

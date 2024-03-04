// Delete a rule returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "create_scorecard_rule" in the system
    let create_scorecard_rule_data_id = std::env::var("CREATE_SCORECARD_RULE_DATA_ID").unwrap();
    let configuration = Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteScorecardRule", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api.delete_scorecard_rule().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

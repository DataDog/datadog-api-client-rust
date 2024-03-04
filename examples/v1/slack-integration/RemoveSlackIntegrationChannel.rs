// Remove a Slack integration channel returns "The channel was removed successfully." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_slack_integration::SlackIntegrationAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SlackIntegrationAPI::with_config(configuration);
    let resp = api.remove_slack_integration_channel().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

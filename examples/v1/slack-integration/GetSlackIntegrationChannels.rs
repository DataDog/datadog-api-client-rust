// Get all channels in a Slack integration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_slack_integration::SlackIntegrationAPI;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SlackIntegrationAPI::with_config(configuration);
    let resp = api.get_slack_integration_channels().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

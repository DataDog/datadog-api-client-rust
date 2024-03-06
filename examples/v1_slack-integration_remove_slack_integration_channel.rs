// Remove a Slack integration channel returns "The channel was removed successfully." response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_slack_integration::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SlackIntegrationAPI::with_config(configuration);
    let resp = api.remove_slack_integration_channel("account_name".to_string(), "channel_name".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

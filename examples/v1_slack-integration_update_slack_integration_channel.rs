// Update a Slack integration channel returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_slack_integration::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        SlackIntegrationChannel::new()
            .display(SlackIntegrationChannelDisplay::new().message(true).notified(true).snapshot(true).tags(true))
            .name("#general".to_string());
    let configuration = Configuration::new();
    let api = SlackIntegrationAPI::with_config(configuration);
    let resp =
        api.update_slack_integration_channel("account_name".to_string(), "channel_name".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

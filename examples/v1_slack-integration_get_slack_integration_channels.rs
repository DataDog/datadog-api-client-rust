// Get all channels in a Slack integration returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_slack_integration::SlackIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = SlackIntegrationAPI::with_config(configuration);
    let resp = api
        .get_slack_integration_channels("account_name".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

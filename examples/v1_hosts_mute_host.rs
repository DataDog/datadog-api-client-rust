// Mute a host returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_hosts::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = HostMuteSettings::new()
        .end(1579098130)
        .message("Muting this host for a test!".to_string())
        .override_(false);
    let configuration = Configuration::new();
    let api = HostsAPI::with_config(configuration);
    let resp = api.mute_host("host_name".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

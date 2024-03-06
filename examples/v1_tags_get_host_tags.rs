// Get host tags returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_tags::*;

#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
    let api = TagsAPI::with_config(configuration);
    let resp = api.get_host_tags("host_name".to_string(), GetHostTagsOptionalParams::default()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

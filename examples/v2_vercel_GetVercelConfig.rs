// Get Vercel configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_vercel::VercelAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = VercelAPI::with_config(configuration);
    let resp = api.get_vercel_config("configuration_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

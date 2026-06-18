// Create Vercel access token returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_vercel::VercelAPI;
use datadog_api_client::datadogV2::model::VercelTokenCreateRequest;

#[tokio::main]
async fn main() {
    let body = VercelTokenCreateRequest::new("code".to_string(), "icfg_abc123".to_string());
    let configuration = datadog::Configuration::new();
    let api = VercelAPI::with_config(configuration);
    let resp = api.create_vercel_token(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

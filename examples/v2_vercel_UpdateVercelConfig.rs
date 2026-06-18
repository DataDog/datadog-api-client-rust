// Update Vercel configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_vercel::VercelAPI;
use datadog_api_client::datadogV2::model::VercelApiKey;
use datadog_api_client::datadogV2::model::VercelConfigAttributes;
use datadog_api_client::datadogV2::model::VercelEnvironment;
use datadog_api_client::datadogV2::model::VercelLogSource;
use datadog_api_client::datadogV2::model::VercelLogsConfig;
use datadog_api_client::datadogV2::model::VercelTraceConfig;

#[tokio::main]
async fn main() {
    let body = VercelConfigAttributes::new(
        VercelApiKey::new(
            "00000000-0000-0000-0000-000000000001".to_string(),
            "".to_string(),
        ),
        VercelLogsConfig::new(
            true,
            vec![VercelEnvironment::PRODUCTION],
            vec![VercelLogSource::LAMBDA],
            100,
        ),
        VercelTraceConfig::new(true, false, 100),
    );
    let configuration = datadog::Configuration::new();
    let api = VercelAPI::with_config(configuration);
    let resp = api
        .update_vercel_config("configuration_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

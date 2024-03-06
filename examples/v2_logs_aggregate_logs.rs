// Aggregate events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_logs::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        LogsAggregateRequest
        ::new().filter(
            LogsQueryFilter::new()
                .from("now-15m".to_string())
                .indexes(vec!["main".to_string()])
                .query("*".to_string())
                .to("now".to_string()),
        );
    let configuration = Configuration::new();
    let api = LogsAPI::with_config(configuration);
    let resp = api.aggregate_logs(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

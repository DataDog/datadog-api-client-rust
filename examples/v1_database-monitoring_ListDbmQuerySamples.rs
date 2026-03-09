// List DBM query samples returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_database_monitoring::DatabaseMonitoringAPI;
use datadog_api_client::datadogV1::model::DbmQuerySamplesRequest;

#[tokio::main]
async fn main() {
    let body = DbmQuerySamplesRequest::new(5, "*".to_string());
    let configuration = datadog::Configuration::new();
    let api = DatabaseMonitoringAPI::with_config(configuration);
    let resp = api.list_dbm_query_samples(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

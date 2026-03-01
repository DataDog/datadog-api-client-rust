// List DBM query samples returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_database_monitoring::DatabaseMonitoringAPI;
use datadog_api_client::datadogV1::model::DbmQuerySamplesRequest;
use datadog_api_client::datadogV1::model::DbmQuerySamplesRequestList;
use datadog_api_client::datadogV1::model::DbmQuerySamplesSearch;
use datadog_api_client::datadogV1::model::DbmType;

#[tokio::main]
async fn main() {
    let body = DbmQuerySamplesRequest::new(DbmQuerySamplesRequestList::new(
        vec!["databasequery".to_string()],
        5,
        DbmQuerySamplesSearch::new("dbm_type:activity".to_string()),
    ));
    let configuration = datadog::Configuration::new();
    let api = DatabaseMonitoringAPI::with_config(configuration);
    let resp = api
        .list_dbm_query_samples(DbmType::DATABASEQUERY, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

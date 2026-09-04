// Execute a tabular DDSQL query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ddsql::DDSQLAPI;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryRequest;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryRequestAttributes;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryRequestData;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryRequestType;
use datadog_api_client::datadogV2::model::DdsqlTabularQueryTimeWindow;

#[tokio::main]
async fn main() {
    let body = DdsqlTabularQueryRequest::new(DdsqlTabularQueryRequestData::new(
        DdsqlTabularQueryRequestAttributes::new(
            "SELECT cloud_provider, count(*) FROM dd.hosts group by cloud_provider".to_string(),
            DdsqlTabularQueryTimeWindow::new(1736942400000, 1736946000000),
        )
        .row_limit(1000),
        DdsqlTabularQueryRequestType::DDSQL_QUERY_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = DDSQLAPI::with_config(configuration);
    let resp = api.execute_ddsql_tabular_query(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

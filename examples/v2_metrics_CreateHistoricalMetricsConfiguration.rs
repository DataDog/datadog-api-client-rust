// Enable historical metrics ingestion returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;
use datadog_api_client::datadogV2::model::HistoricalMetricsConfigurationCreateData;
use datadog_api_client::datadogV2::model::HistoricalMetricsConfigurationCreateRequest;
use datadog_api_client::datadogV2::model::HistoricalMetricsConfigurationType;

#[tokio::main]
async fn main() {
    let body = HistoricalMetricsConfigurationCreateRequest::new(
        HistoricalMetricsConfigurationCreateData::new(
            "dd.test.metric".to_string(),
            HistoricalMetricsConfigurationType::HISTORICAL_METRICS_CONFIGURATIONS,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateHistoricalMetricsConfiguration", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api.create_historical_metrics_configuration(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

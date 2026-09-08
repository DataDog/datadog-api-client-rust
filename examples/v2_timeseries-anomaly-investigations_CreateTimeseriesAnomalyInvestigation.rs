// Investigate a timeseries anomaly returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_timeseries_anomaly_investigations::TimeseriesAnomalyInvestigationsAPI;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationDataSource;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationFormula;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationMetricQuery;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationRequest;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationRequestAttributes;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationRequestData;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationTimeseriesRequest;
use datadog_api_client::datadogV2::model::TimeseriesAnomalyInvestigationType;

#[tokio::main]
async fn main() {
    let body =
        TimeseriesAnomalyInvestigationRequest::new(TimeseriesAnomalyInvestigationRequestData::new(
            TimeseriesAnomalyInvestigationRequestAttributes::new(vec![
                TimeseriesAnomalyInvestigationTimeseriesRequest::new(
                    vec![TimeseriesAnomalyInvestigationFormula::new(
                        "anomalies(query1, 'agile', 3)".to_string(),
                    )],
                    1754406000000,
                    vec![TimeseriesAnomalyInvestigationMetricQuery::new(
                        TimeseriesAnomalyInvestigationDataSource::METRICS,
                        "query1".to_string(),
                        "avg:system.cpu.user{env:prod} by {service}".to_string(),
                    )],
                    1754423940000,
                ),
            ]),
            TimeseriesAnomalyInvestigationType::TIMESERIES_ANOMALY_INVESTIGATION,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTimeseriesAnomalyInvestigation", true);
    let api = TimeseriesAnomalyInvestigationsAPI::with_config(configuration);
    let resp = api.create_timeseries_anomaly_investigation(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

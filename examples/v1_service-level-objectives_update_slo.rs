// Update an SLO returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objectives::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "slo" in the system
    let slo_data_0_id = std::env::var("SLO_DATA_0_ID").unwrap();
    let slo_data_0_name = std::env::var("SLO_DATA_0_NAME").unwrap();
    let body = ServiceLevelObjective::new(
        slo_data_0_name.clone(),
        vec![SLOThreshold::new(97.0, SLOTimeframe::SEVEN_DAYS).warning(98.0 as f64)],
        SLOType::METRIC,
    )
    .query(ServiceLevelObjectiveQuery::new(
        "sum:httpservice.hits{!code:3xx}.as_count()".to_string(),
        "sum:httpservice.hits{code:2xx}.as_count()".to_string(),
    ))
    .target_threshold(97.0 as f64)
    .timeframe(SLOTimeframe::SEVEN_DAYS)
    .warning_threshold(98 as f64);
    let configuration = Configuration::new();
    let api = ServiceLevelObjectivesAPI::with_config(configuration);
    let resp = api.update_slo(slo_data_0_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
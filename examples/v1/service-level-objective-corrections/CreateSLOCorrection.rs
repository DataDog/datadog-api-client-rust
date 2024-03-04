// Create an SLO correction returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "slo" in the system
    let slo_data_0_id = std::env::var("SLO_DATA_0_ID").unwrap();
    let body =
        SLOCorrectionCreateRequest
        ::new().data(
            SLOCorrectionCreateData::new(
                SLOCorrectionType::CORRECTION,
            ).attributes(
                SLOCorrectionCreateRequestAttributes::new(
                    SLOCorrectionCategory::SCHEDULED_MAINTENANCE,
                    slo_data_0_id,
                    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                )
                    .description("Example-Service-Level-Objective-Correction".to_string())
                    .end(
                        SystemTime::now()
                            .add(Duration::from_secs(1 * 3600))
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as
                            i64,
                    )
                    .timezone("UTC".to_string()),
            ),
        );
    let configuration = Configuration::new();
    let api = ServiceLevelObjectiveCorrectionsAPI::with_config(configuration);
    let resp = api.create_slo_correction(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

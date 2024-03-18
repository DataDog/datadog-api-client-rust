// Update an SLO correction returns "OK" response
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_service_level_objective_corrections::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "correction" for "slo"
    let correction_data_id = std::env::var("CORRECTION_DATA_ID").unwrap();
    let body = SLOCorrectionUpdateRequest::new().data(
        SLOCorrectionUpdateData::new()
            .attributes(
                SLOCorrectionUpdateRequestAttributes::new()
                    .category(SLOCorrectionCategory::DEPLOYMENT)
                    .description("Example-Service-Level-Objective-Correction".to_string())
                    .end(1636632671)
                    .start(1636629071)
                    .timezone("UTC".to_string()),
            )
            .type_(SLOCorrectionType::CORRECTION),
    );
    let configuration = Configuration::new();
    let api = ServiceLevelObjectiveCorrectionsAPI::with_config(configuration);
    let resp = api
        .update_slo_correction(correction_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

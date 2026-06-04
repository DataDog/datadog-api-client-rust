// Create an SLO correction with slo_query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_service_level_objective_corrections::ServiceLevelObjectiveCorrectionsAPI;
use datadog_api_client::datadogV1::model::SLOCorrectionCategory;
use datadog_api_client::datadogV1::model::SLOCorrectionCreateData;
use datadog_api_client::datadogV1::model::SLOCorrectionCreateRequest;
use datadog_api_client::datadogV1::model::SLOCorrectionCreateRequestAttributes;
use datadog_api_client::datadogV1::model::SLOCorrectionType;

#[tokio::main]
async fn main() {
    let body = SLOCorrectionCreateRequest::new().data(
        SLOCorrectionCreateData::new(SLOCorrectionType::CORRECTION).attributes(
            SLOCorrectionCreateRequestAttributes::new(
                SLOCorrectionCategory::SCHEDULED_MAINTENANCE,
                1636629071,
            )
            .description("Example-Service-Level-Objective-Correction".to_string())
            .end(1636632671)
            .slo_query("env:prod service:checkout".to_string())
            .timezone("UTC".to_string()),
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ServiceLevelObjectiveCorrectionsAPI::with_config(configuration);
    let resp = api.create_slo_correction(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

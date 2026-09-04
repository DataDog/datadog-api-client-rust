// Create a DEM journey variant returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::DEMAPI;
use datadog_api_client::datadogV2::model::DemRumNode;
use datadog_api_client::datadogV2::model::DemRumStep;
use datadog_api_client::datadogV2::model::DemRumStepType;
use datadog_api_client::datadogV2::model::DemVariantAttributes;
use datadog_api_client::datadogV2::model::DemVariantRequest;
use datadog_api_client::datadogV2::model::DemVariantRequestData;
use datadog_api_client::datadogV2::model::DemVariantType;

#[tokio::main]
async fn main() {
    let body = DemVariantRequest::new(DemVariantRequestData::new(
        DemVariantAttributes::new(
            "Mobile checkout".to_string(),
            vec![
                DemRumStep::new(
                    vec![DemRumNode::new("action.name:'checkout'".to_string())],
                    DemRumStepType::START,
                ),
                DemRumStep::new(
                    vec![DemRumNode::new("action.name:'confirmation'".to_string())],
                    DemRumStepType::STOP,
                ),
            ],
        )
        .filter("device.type:mobile".to_string()),
        DemVariantType::VARIANTS,
    ));
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api
        .create_journey_variant("journey_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

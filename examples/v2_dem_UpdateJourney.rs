// Update a DEM journey returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dem::DEMAPI;
use datadog_api_client::datadogV2::model::DemJourneyCreateAttributes;
use datadog_api_client::datadogV2::model::DemJourneyCreateData;
use datadog_api_client::datadogV2::model::DemJourneyCreateRequest;
use datadog_api_client::datadogV2::model::DemJourneyRum;
use datadog_api_client::datadogV2::model::DemJourneyType;
use datadog_api_client::datadogV2::model::DemRumNode;
use datadog_api_client::datadogV2::model::DemRumStep;
use datadog_api_client::datadogV2::model::DemRumStepType;
use datadog_api_client::datadogV2::model::DemVariant;

#[tokio::main]
async fn main() {
    let body = DemJourneyCreateRequest::new(DemJourneyCreateData::new(
        DemJourneyCreateAttributes::new(
            DemJourneyRum::new(vec![
                DemRumStep::new(
                    vec![DemRumNode::new("action.name:'checkout'".to_string())],
                    DemRumStepType::START,
                ),
                DemRumStep::new(
                    vec![DemRumNode::new("action.name:'confirmation'".to_string())],
                    DemRumStepType::STOP,
                ),
            ])
            .filter("env:prod".to_string())
            .variants(vec![DemVariant::new(
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
            )]),
            "Checkout Flow".to_string(),
            vec!["team:synthetics".to_string(), "env:prod".to_string()],
        )
        .description("Tracks the user checkout flow from cart to confirmation.".to_string())
        .variants(vec![DemVariant::new(
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
        )]),
        DemJourneyType::JOURNEYS,
    ));
    let configuration = datadog::Configuration::new();
    let api = DEMAPI::with_config(configuration);
    let resp = api.update_journey("journey_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

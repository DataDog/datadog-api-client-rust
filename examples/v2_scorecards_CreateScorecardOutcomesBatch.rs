// Create outcomes batch returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_scorecards::ScorecardsAPI;
use datadog_api_client::datadogV2::model::OutcomesBatchAttributes;
use datadog_api_client::datadogV2::model::OutcomesBatchRequest;
use datadog_api_client::datadogV2::model::OutcomesBatchRequestData;
use datadog_api_client::datadogV2::model::OutcomesBatchRequestItem;
use datadog_api_client::datadogV2::model::OutcomesBatchType;
use datadog_api_client::datadogV2::model::State;

#[tokio::main]
async fn main() {
    let body = OutcomesBatchRequest::new().data(
        OutcomesBatchRequestData::new()
            .attributes(
                OutcomesBatchAttributes::new().results(vec![OutcomesBatchRequestItem::new(
                    "q8MQxk8TCqrHnWkx".to_string(),
                    "my-service".to_string(),
                    State::PASS,
                )
                .remarks(
                    r#"See: <a href="https://app.datadoghq.com/services">Services</a>"#.to_string(),
                )]),
            )
            .type_(OutcomesBatchType::BATCHED_OUTCOME),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateScorecardOutcomesBatch", true);
    let api = ScorecardsAPI::with_config(configuration);
    let resp = api.create_scorecard_outcomes_batch(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

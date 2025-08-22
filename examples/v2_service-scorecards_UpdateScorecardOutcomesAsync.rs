// Update Scorecard outcomes asynchronously returns "Accepted" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_scorecards::ServiceScorecardsAPI;
use datadog_api_client::datadogV2::model::State;
use datadog_api_client::datadogV2::model::UpdateOutcomesAsyncAttributes;
use datadog_api_client::datadogV2::model::UpdateOutcomesAsyncRequest;
use datadog_api_client::datadogV2::model::UpdateOutcomesAsyncRequestData;
use datadog_api_client::datadogV2::model::UpdateOutcomesAsyncRequestItem;
use datadog_api_client::datadogV2::model::UpdateOutcomesAsyncType;

#[tokio::main]
async fn main() {
    // there is a valid "create_scorecard_rule" in the system
    let create_scorecard_rule_data_id = std::env::var("CREATE_SCORECARD_RULE_DATA_ID").unwrap();
    let body = UpdateOutcomesAsyncRequest::new().data(
        UpdateOutcomesAsyncRequestData::new()
            .attributes(UpdateOutcomesAsyncAttributes::new().results(vec![
                    UpdateOutcomesAsyncRequestItem::new(
                        "service:my-service".to_string(),
                        create_scorecard_rule_data_id.clone(),
                        State::PASS,
                    )
                    .remarks(
                        r#"See: <a href="https://app.datadoghq.com/services">Services</a>"#
                            .to_string(),
                    ),
                ]))
            .type_(UpdateOutcomesAsyncType::BATCHED_OUTCOME),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateScorecardOutcomesAsync", true);
    let api = ServiceScorecardsAPI::with_config(configuration);
    let resp = api.update_scorecard_outcomes_async(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

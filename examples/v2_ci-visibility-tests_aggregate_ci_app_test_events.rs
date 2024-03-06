// Aggregate tests events returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_ci_visibility_tests::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    let body =
        CIAppTestsAggregateRequest::new()
            .compute(
                vec![
                    CIAppCompute::new(CIAppAggregationFunction::COUNT)
                        .metric("@test.is_flaky".to_string())
                        .type_(CIAppComputeType::TOTAL)
                ],
            )
            .filter(
                CIAppTestsQueryFilter::new()
                    .from("now-15m".to_string())
                    .query("@language:(python OR go)".to_string())
                    .to("now".to_string()),
            )
            .group_by(
                vec![
                    CIAppTestsGroupBy::new("@git.branch".to_string())
                        .limit(10)
                        .sort(CIAppAggregateSort::new().order(CIAppSortOrder::ASCENDING))
                        .total(CIAppGroupByTotal::CIAppGroupByTotalBoolean(false))
                ],
            )
            .options(CIAppQueryOptions::new().timezone("GMT".to_string()));
    let configuration = Configuration::new();
    let api = CIVisibilityTestsAPI::with_config(configuration);
    let resp = api.aggregate_ci_app_test_events(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

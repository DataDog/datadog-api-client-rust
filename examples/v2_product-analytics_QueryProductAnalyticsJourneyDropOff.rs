// Analyze journey drop-off returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsBaseQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQueryDataSource;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventSearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsJoinKeys;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyAudienceAccountQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyAudienceFilters;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyAudienceSegmentQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyAudienceUserQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyDropOffQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyDropOffRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyDropOffRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyDropOffRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyDropOffRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyNodeTarget;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyNodeTargetType;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneySearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneySearchFilters;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneySearchGraphFilter;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterName;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterOperator;
use datadog_api_client::datadogV2::model::ProductAnalyticsJourneyTarget;
use datadog_api_client::datadogV2::model::ProductAnalyticsSampling;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        ProductAnalyticsJourneyDropOffRequest::new(
            ProductAnalyticsJourneyDropOffRequestData::new(
                ProductAnalyticsJourneyDropOffRequestAttributes::new(
                    1756425600000,
                    ProductAnalyticsJourneyDropOffQuery::new(
                        ProductAnalyticsJourneySearch::new(
                            "A -> B".to_string(),
                            BTreeMap::from(
                                [
                                    (
                                        "A".to_string(),
                                        ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(
                                            Box::new(
                                                ProductAnalyticsEventQuery::new(
                                                    ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                                                    ProductAnalyticsEventSearch
                                                    ::new().query("@type:view @view.name:Login".to_string()),
                                                ),
                                            ),
                                        ),
                                    ),
                                    (
                                        "B".to_string(),
                                        ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(
                                            Box::new(
                                                ProductAnalyticsEventQuery::new(
                                                    ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                                                    ProductAnalyticsEventSearch
                                                    ::new().query(
                                                        "@type:action @action.target.name:Submit".to_string(),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ],
                            ),
                        )
                            .filters(
                                ProductAnalyticsJourneySearchFilters::new()
                                    .audience_filters(
                                        ProductAnalyticsJourneyAudienceFilters::new()
                                            .accounts(
                                                vec![
                                                    ProductAnalyticsJourneyAudienceAccountQuery::new(
                                                        "enterprise_accounts".to_string(),
                                                    )
                                                ],
                                            )
                                            .formula("power_users AND NOT trial_segment".to_string())
                                            .segments(
                                                vec![
                                                    ProductAnalyticsJourneyAudienceSegmentQuery::new(
                                                        "trial_segment".to_string(),
                                                        "00000000-0000-0000-0000-000000000000".to_string(),
                                                    )
                                                ],
                                            )
                                            .users(
                                                vec![
                                                    ProductAnalyticsJourneyAudienceUserQuery::new(
                                                        "power_users".to_string(),
                                                    )
                                                ],
                                            ),
                                    )
                                    .graph_filters(
                                        vec![
                                            ProductAnalyticsJourneySearchGraphFilter::new(
                                                ProductAnalyticsJourneySearchGraphFilterName::TIME_TO_CONVERT,
                                                ProductAnalyticsJourneySearchGraphFilterOperator::LESS_THAN_OR_EQUAL,
                                                60000,
                                            ).target(
                                                ProductAnalyticsJourneyTarget::ProductAnalyticsJourneyNodeTarget(
                                                    Box::new(
                                                        ProductAnalyticsJourneyNodeTarget::new(
                                                            ProductAnalyticsJourneyNodeTargetType::NODE,
                                                            "A".to_string(),
                                                        ),
                                                    ),
                                                ),
                                            )
                                        ],
                                    ),
                            )
                            .join_keys(
                                ProductAnalyticsJoinKeys::new().primary("@session.id".to_string()).secondary(vec![]),
                            ),
                    )
                        .sampling(ProductAnalyticsSampling::new().enabled(false))
                        .target(
                            ProductAnalyticsJourneyNodeTarget::new(
                                ProductAnalyticsJourneyNodeTargetType::NODE,
                                "B".to_string(),
                            ),
                        )
                        .top_issue_count(5),
                    1756857600000,
                ),
                ProductAnalyticsJourneyDropOffRequestType::JOURNEY_DROP_OFF_ANALYSIS_REQUEST,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.query_product_analytics_journey_drop_off", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_journey_drop_off(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

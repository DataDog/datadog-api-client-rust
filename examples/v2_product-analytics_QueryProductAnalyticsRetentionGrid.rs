// Compute a retention grid returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_product_analytics::ProductAnalyticsAPI;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceAccountSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceFilters;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceSegmentSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsAudienceUserSubquery;
use datadog_api_client::datadogV2::model::ProductAnalyticsBaseQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsCalendarInterval;
use datadog_api_client::datadogV2::model::ProductAnalyticsCalendarIntervalType;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventQueryDataSource;
use datadog_api_client::datadogV2::model::ProductAnalyticsEventSearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsGroupBySort;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCalendarTimeInterval;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCalendarTimeIntervalType;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCohortCriteria;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCohortScope;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCohortScopeType;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCohortTarget;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionCompute;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionComputeMetric;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionEntity;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionFilters;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGridQuery;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGridRequest;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGridRequestAttributes;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGridRequestData;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGridRequestType;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGroupBy;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionGroupByTarget;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionIndexTarget;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionIndexTargetType;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionReturnCondition;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionReturnCriteria;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionScope;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionSearch;
use datadog_api_client::datadogV2::model::ProductAnalyticsRetentionTimeInterval;
use datadog_api_client::datadogV2::model::QuerySortOrder;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body =
        ProductAnalyticsRetentionGridRequest::new(
            ProductAnalyticsRetentionGridRequestData::new(
                ProductAnalyticsRetentionGridRequestAttributes::new(
                    1756425600000,
                    ProductAnalyticsRetentionGridQuery::new(
                        ProductAnalyticsRetentionCompute::new(
                            "count".to_string(),
                            ProductAnalyticsRetentionComputeMetric::RETENTION_RATE,
                        ),
                        ProductAnalyticsRetentionSearch::new(
                            ProductAnalyticsRetentionCohortCriteria::new(
                                ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(
                                    Box::new(
                                        ProductAnalyticsEventQuery::new(
                                            ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                                            ProductAnalyticsEventSearch::new().query("@type:view".to_string()),
                                        ),
                                    ),
                                ),
                                ProductAnalyticsRetentionTimeInterval::ProductAnalyticsRetentionCalendarTimeInterval(
                                    Box::new(
                                        ProductAnalyticsRetentionCalendarTimeInterval::new(
                                            ProductAnalyticsRetentionCalendarTimeIntervalType::CALENDAR,
                                            ProductAnalyticsCalendarInterval::new(
                                                ProductAnalyticsCalendarIntervalType::WEEK,
                                            )
                                                .alignment("monday".to_string())
                                                .quantity(1)
                                                .timezone("UTC".to_string()),
                                        ),
                                    ),
                                ),
                            ),
                            ProductAnalyticsRetentionEntity::USER_ID,
                            ProductAnalyticsRetentionReturnCondition::CONVERSION_ON_OR_AFTER,
                        )
                            .filters(
                                ProductAnalyticsRetentionFilters
                                ::new().audience_filters(
                                    ProductAnalyticsAudienceFilters::new()
                                        .accounts(vec![ProductAnalyticsAudienceAccountSubquery::new("".to_string())])
                                        .formula("u".to_string())
                                        .segments(
                                            vec![
                                                ProductAnalyticsAudienceSegmentSubquery::new(
                                                    "".to_string(),
                                                    Uuid::parse_str(
                                                        "00000000-0000-0000-0000-000000000000",
                                                    ).expect("invalid UUID"),
                                                )
                                            ],
                                        )
                                        .users(
                                            vec![
                                                ProductAnalyticsAudienceUserSubquery::new(
                                                    "u".to_string(),
                                                ).query("*".to_string())
                                            ],
                                        ),
                                ),
                            )
                            .return_criteria(
                                ProductAnalyticsRetentionReturnCriteria::new(
                                    ProductAnalyticsBaseQuery::ProductAnalyticsEventQuery(
                                        Box::new(
                                            ProductAnalyticsEventQuery::new(
                                                ProductAnalyticsEventQueryDataSource::PRODUCT_ANALYTICS,
                                                ProductAnalyticsEventSearch::new().query("@type:view".to_string()),
                                            ),
                                        ),
                                    ),
                                ).time_interval(
                                    ProductAnalyticsRetentionTimeInterval
                                    ::ProductAnalyticsRetentionCalendarTimeInterval(
                                        Box::new(
                                            ProductAnalyticsRetentionCalendarTimeInterval::new(
                                                ProductAnalyticsRetentionCalendarTimeIntervalType::CALENDAR,
                                                ProductAnalyticsCalendarInterval::new(
                                                    ProductAnalyticsCalendarIntervalType::WEEK,
                                                )
                                                    .alignment("monday".to_string())
                                                    .quantity(1)
                                                    .timezone("UTC".to_string()),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                    )
                        .computation_scope(
                            ProductAnalyticsRetentionScope::ProductAnalyticsRetentionCohortScope(
                                Box::new(
                                    ProductAnalyticsRetentionCohortScope::new(
                                        ProductAnalyticsRetentionCohortTarget::ProductAnalyticsRetentionIndexTarget(
                                            Box::new(
                                                ProductAnalyticsRetentionIndexTarget::new(
                                                    ProductAnalyticsRetentionIndexTargetType::INDEX,
                                                    0,
                                                ),
                                            ),
                                        ),
                                        ProductAnalyticsRetentionCohortScopeType::COHORT,
                                    ),
                                ),
                            ),
                        )
                        .group_by(
                            vec![
                                ProductAnalyticsRetentionGroupBy::new(
                                    "@geo.country".to_string(),
                                    ProductAnalyticsRetentionGroupByTarget::COHORT,
                                )
                                    .limit(10)
                                    .should_exclude_missing(false)
                                    .sort(
                                        ProductAnalyticsGroupBySort::new()
                                            .aggregation("count".to_string())
                                            .order(QuerySortOrder::DESC),
                                    )
                            ],
                        ),
                    1756857600000,
                ).exclude_anonymous_traffic(false),
                ProductAnalyticsRetentionGridRequestType::RETENTION_GRID_REQUEST,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryProductAnalyticsRetentionGrid", true);
    let api = ProductAnalyticsAPI::with_config(configuration);
    let resp = api.query_product_analytics_retention_grid(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Query event filtered users returns "Successful response with filtered user
// data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequest;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequestData;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequestDataAttributes;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequestDataAttributesEventQuery;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequestDataAttributesEventQueryTimeFrame;
use datadog_api_client::datadogV2::model::QueryEventFilteredUsersRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        QueryEventFilteredUsersRequest
        ::new().data(
            QueryEventFilteredUsersRequestData::new(
                QueryEventFilteredUsersRequestDataType::QUERY_EVENT_FILTERED_USERS_REQUEST,
            )
                .attributes(
                    QueryEventFilteredUsersRequestDataAttributes::new()
                        .event_query(
                            QueryEventFilteredUsersRequestDataAttributesEventQuery::new()
                                .query(
                                    "@type:view AND @view.loading_time:>3000 AND @application.name:ecommerce-platform".to_string(),
                                )
                                .time_frame(
                                    QueryEventFilteredUsersRequestDataAttributesEventQueryTimeFrame::new()
                                        .end(1761309676)
                                        .start(1760100076),
                                ),
                        )
                        .include_row_count(true)
                        .limit(25)
                        .query("user_org_id:5001 AND first_country_code:US AND first_browser_name:Chrome".to_string())
                        .select_columns(
                            vec![
                                "user_id".to_string(),
                                "user_email".to_string(),
                                "first_country_code".to_string(),
                                "first_browser_name".to_string(),
                                "events_count".to_string(),
                                "session_count".to_string(),
                                "error_count".to_string(),
                                "avg_loading_time".to_string()
                            ],
                        ),
                )
                .id("query_event_filtered_users_request".to_string()),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryEventFilteredUsers", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.query_event_filtered_users(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

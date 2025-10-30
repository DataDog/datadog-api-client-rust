// Query users returns "Successful response with user data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::QueryUsersRequest;
use datadog_api_client::datadogV2::model::QueryUsersRequestData;
use datadog_api_client::datadogV2::model::QueryUsersRequestDataAttributes;
use datadog_api_client::datadogV2::model::QueryUsersRequestDataAttributesSort;
use datadog_api_client::datadogV2::model::QueryUsersRequestDataType;

#[tokio::main]
async fn main() {
    let body =
        QueryUsersRequest
        ::new().data(
            QueryUsersRequestData::new(QueryUsersRequestDataType::QUERY_USERS_REQUEST)
                .attributes(
                    QueryUsersRequestDataAttributes::new()
                        .limit(25)
                        .query(
                            "user_email:*@techcorp.com AND first_country_code:US AND first_browser_name:Chrome".to_string(),
                        )
                        .select_columns(
                            vec![
                                "user_id".to_string(),
                                "user_email".to_string(),
                                "user_name".to_string(),
                                "user_org_id".to_string(),
                                "first_country_code".to_string(),
                                "first_browser_name".to_string(),
                                "first_device_type".to_string(),
                                "last_seen".to_string()
                            ],
                        )
                        .sort(
                            QueryUsersRequestDataAttributesSort::new()
                                .field("first_seen".to_string())
                                .order("DESC".to_string()),
                        )
                        .wildcard_search_term("john".to_string()),
                )
                .id("query_users_request".to_string()),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryUsers", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.query_users(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

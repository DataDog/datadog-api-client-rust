// Query accounts returns "Successful response with account data" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_audience_management::RumAudienceManagementAPI;
use datadog_api_client::datadogV2::model::QueryAccountRequest;
use datadog_api_client::datadogV2::model::QueryAccountRequestData;
use datadog_api_client::datadogV2::model::QueryAccountRequestDataAttributes;
use datadog_api_client::datadogV2::model::QueryAccountRequestDataAttributesSort;
use datadog_api_client::datadogV2::model::QueryAccountRequestDataType;

#[tokio::main]
async fn main() {
    let body = QueryAccountRequest::new().data(
        QueryAccountRequestData::new(QueryAccountRequestDataType::QUERY_ACCOUNT_REQUEST)
            .attributes(
                QueryAccountRequestDataAttributes::new()
                    .limit(20)
                    .query(
                        "plan_type:enterprise AND user_count:>100 AND subscription_status:active"
                            .to_string(),
                    )
                    .select_columns(vec![
                        "account_id".to_string(),
                        "account_name".to_string(),
                        "user_count".to_string(),
                        "plan_type".to_string(),
                        "subscription_status".to_string(),
                        "created_at".to_string(),
                        "mrr".to_string(),
                        "industry".to_string(),
                    ])
                    .sort(
                        QueryAccountRequestDataAttributesSort::new()
                            .field("user_count".to_string())
                            .order("DESC".to_string()),
                    )
                    .wildcard_search_term("tech".to_string()),
            )
            .id("query_account_request".to_string()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryAccounts", true);
    let api = RumAudienceManagementAPI::with_config(configuration);
    let resp = api.query_accounts(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

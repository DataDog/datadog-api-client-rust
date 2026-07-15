// Update account filters returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_cost_management::CloudCostManagementAPI;
use datadog_api_client::datadogV2::model::AccountFilteringConfig;
use datadog_api_client::datadogV2::model::AccountFiltersPatchData;
use datadog_api_client::datadogV2::model::AccountFiltersPatchRequest;
use datadog_api_client::datadogV2::model::AccountFiltersPatchRequestAttributes;
use datadog_api_client::datadogV2::model::AccountFiltersPatchRequestType;

#[tokio::main]
async fn main() {
    let body = AccountFiltersPatchRequest::new(AccountFiltersPatchData::new(
        AccountFiltersPatchRequestAttributes::new(
            AccountFilteringConfig::new()
                .excluded_accounts(vec!["123456789123".to_string(), "123456789143".to_string()])
                .include_new_accounts(Some(true))
                .included_accounts(vec!["123456789123".to_string(), "123456789143".to_string()]),
        ),
        AccountFiltersPatchRequestType::ACCOUNT_FILTERS_PATCH_REQUEST,
    ));
    let configuration = datadog::Configuration::new();
    let api = CloudCostManagementAPI::with_config(configuration);
    let resp = api
        .update_cost_account_filters(9223372036854775807, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

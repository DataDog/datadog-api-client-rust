// Update a WAF exclusion filter returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterOnMatch;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterType;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterUpdateAttributes;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterUpdateData;
use datadog_api_client::datadogV2::model::ApplicationSecurityWafExclusionFilterUpdateRequest;

#[tokio::main]
async fn main() {
    // there is a valid "exclusion_filter" in the system
    let exclusion_filter_data_id = std::env::var("EXCLUSION_FILTER_DATA_ID").unwrap();
    let body = ApplicationSecurityWafExclusionFilterUpdateRequest::new(
        ApplicationSecurityWafExclusionFilterUpdateData::new(
            ApplicationSecurityWafExclusionFilterUpdateAttributes::new(
                "Exclude false positives on a path".to_string(),
                false,
            )
            .ip_list(vec!["198.51.100.72".to_string()])
            .on_match(ApplicationSecurityWafExclusionFilterOnMatch::MONITOR),
            ApplicationSecurityWafExclusionFilterType::EXCLUSION_FILTER,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .update_application_security_waf_exclusion_filter(exclusion_filter_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

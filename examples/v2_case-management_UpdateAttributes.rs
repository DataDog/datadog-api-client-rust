// Update case attributes returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseResourceType;
use datadog_api_client::datadogV2::model::CaseUpdateAttributes;
use datadog_api_client::datadogV2::model::CaseUpdateAttributesAttributes;
use datadog_api_client::datadogV2::model::CaseUpdateAttributesRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "case" in the system
    let case_id = std::env::var("CASE_ID").unwrap();
    let body = CaseUpdateAttributesRequest::new(CaseUpdateAttributes::new(
        CaseUpdateAttributesAttributes::new(BTreeMap::from([
            ("env".to_string(), vec!["test".to_string()]),
            (
                "service".to_string(),
                vec!["web-store".to_string(), "web-api".to_string()],
            ),
            ("team".to_string(), vec!["engineer".to_string()]),
        ])),
        CaseResourceType::CASE,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.update_attributes(case_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

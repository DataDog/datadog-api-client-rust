// Bulk update cases returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_case_management::CaseManagementAPI;
use datadog_api_client::datadogV2::model::CaseBulkActionType;
use datadog_api_client::datadogV2::model::CaseBulkResourceType;
use datadog_api_client::datadogV2::model::CaseBulkUpdateRequest;
use datadog_api_client::datadogV2::model::CaseBulkUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::CaseBulkUpdateRequestData;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CaseBulkUpdateRequest::new(CaseBulkUpdateRequestData::new(
        CaseBulkUpdateRequestAttributes::new(
            vec!["case-id-1".to_string(), "case-id-2".to_string()],
            CaseBulkActionType::PRIORITY,
        )
        .payload(BTreeMap::from([("priority".to_string(), "P1".to_string())])),
        CaseBulkResourceType::BULK,
    ));
    let configuration = datadog::Configuration::new();
    let api = CaseManagementAPI::with_config(configuration);
    let resp = api.bulk_update_cases(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Create Scanning Rule returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_sensitive_data_scanner::*;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "scanning_group" in the system
    let group_data_id = std::env::var("GROUP_DATA_ID").unwrap();
    let body =
        SensitiveDataScannerRuleCreateRequest::new(
            SensitiveDataScannerRuleCreate::new(
                SensitiveDataScannerRuleAttributes::new()
                    .excluded_namespaces(vec!["admin.name".to_string()])
                    .is_enabled(true)
                    .name("Example-Sensitive-Data-Scanner".to_string())
                    .namespaces(vec!["admin".to_string()])
                    .pattern("pattern".to_string())
                    .priority(1)
                    .tags(vec!["sensitive_data:true".to_string()])
                    .text_replacement(
                        SensitiveDataScannerTextReplacement
                        ::new().type_(SensitiveDataScannerTextReplacementType::NONE),
                    ),
                SensitiveDataScannerRuleRelationships
                ::new().group(
                    SensitiveDataScannerGroupData
                    ::new().data(
                        SensitiveDataScannerGroup::new()
                            .id(group_data_id.clone())
                            .type_(SensitiveDataScannerGroupType::SENSITIVE_DATA_SCANNER_GROUP),
                    ),
                ),
                SensitiveDataScannerRuleType::SENSITIVE_DATA_SCANNER_RULE,
            ),
            SensitiveDataScannerMetaVersionOnly::new(),
        );
    let configuration = Configuration::new();
    let api = SensitiveDataScannerAPI::with_config(configuration);
    let resp = api.create_scanning_rule(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

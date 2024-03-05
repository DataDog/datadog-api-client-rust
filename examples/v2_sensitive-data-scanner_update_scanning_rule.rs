// Update Scanning Rule returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_sensitive_data_scanner::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // the "scanning_group" has a "scanning_rule"
    let rule_data_id = std::env::var("RULE_DATA_ID").unwrap();

    // there is a valid "scanning_group" in the system
    let group_data_id = std::env::var("GROUP_DATA_ID").unwrap();
    let body =
        SensitiveDataScannerRuleUpdateRequest::new(
            SensitiveDataScannerRuleUpdate::new()
                .attributes(
                    SensitiveDataScannerRuleAttributes::new()
                        .is_enabled(true)
                        .name("Example-Sensitive-Data-Scanner".to_string())
                        .pattern("pattern".to_string())
                        .priority(5)
                        .tags(vec!["sensitive_data:true".to_string()])
                        .text_replacement(
                            SensitiveDataScannerTextReplacement
                            ::new().type_(SensitiveDataScannerTextReplacementType::NONE),
                        ),
                )
                .id(rule_data_id)
                .relationships(
                    SensitiveDataScannerRuleRelationships
                    ::new().group(
                        SensitiveDataScannerGroupData
                        ::new().data(
                            SensitiveDataScannerGroup::new()
                                .id(group_data_id)
                                .type_(SensitiveDataScannerGroupType::SENSITIVE_DATA_SCANNER_GROUP),
                        ),
                    ),
                )
                .type_(SensitiveDataScannerRuleType::SENSITIVE_DATA_SCANNER_RULE),
            SensitiveDataScannerMetaVersionOnly::new(),
        );
    let configuration = Configuration::new();
    let api = SensitiveDataScannerAPI::with_config(configuration);
    let resp = api.update_scanning_rule(rule_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

// Update Scanning Group returns "OK" response
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_sensitive_data_scanner::*;
use datadog_api_client::datadogV2::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    // there is a valid "scanning_group" in the system
    let group_data_id = std::env::var("GROUP_DATA_ID").unwrap();

    // a valid "configuration" in the system
    let configuration_data_id = std::env::var("CONFIGURATION_DATA_ID").unwrap();
    let body =
        SensitiveDataScannerGroupUpdateRequest::new(
            SensitiveDataScannerGroupUpdate::new()
                .attributes(
                    SensitiveDataScannerGroupAttributes::new()
                        .filter(SensitiveDataScannerFilter::new().query("*".to_string()))
                        .is_enabled(false)
                        .name("Example-Sensitive-Data-Scanner".to_string())
                        .product_list(vec![SensitiveDataScannerProduct::LOGS]),
                )
                .id(group_data_id)
                .relationships(
                    SensitiveDataScannerGroupRelationships::new()
                        .configuration(
                            SensitiveDataScannerConfigurationData
                            ::new().data(
                                SensitiveDataScannerConfiguration::new()
                                    .id(configuration_data_id)
                                    .type_(
                                        SensitiveDataScannerConfigurationType::SENSITIVE_DATA_SCANNER_CONFIGURATIONS,
                                    ),
                            ),
                        )
                        .rules(SensitiveDataScannerRuleData::new().data(vec![])),
                )
                .type_(SensitiveDataScannerGroupType::SENSITIVE_DATA_SCANNER_GROUP),
            SensitiveDataScannerMetaVersionOnly::new(),
        );
    let configuration = Configuration::new();
    let api = SensitiveDataScannerAPI::with_config(configuration);
    let resp = api.update_scanning_group(group_data_id, body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

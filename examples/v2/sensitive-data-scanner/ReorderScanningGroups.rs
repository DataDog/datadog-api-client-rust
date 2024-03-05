// Reorder Groups returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV2::api::api_sensitive_data_scanner::SensitiveDataScannerAPI;
use datadog_api_client::datadogV2::model::*;

#[tokio::main]
async fn main() {
    // there is a valid "scanning_group" in the system
    let group_data_id = std::env::var("GROUP_DATA_ID").unwrap();

    // a valid "configuration" in the system
    let configuration_data_id = std::env::var("CONFIGURATION_DATA_ID").unwrap();
    let body =
        SensitiveDataScannerConfigRequest::new(
            SensitiveDataScannerReorderConfig::new()
                .id(configuration_data_id)
                .relationships(
                    SensitiveDataScannerConfigurationRelationships
                    ::new().groups(
                        SensitiveDataScannerGroupList
                        ::new().data(
                            vec![
                                SensitiveDataScannerGroupItem::new()
                                    .id(group_data_id)
                                    .type_(SensitiveDataScannerGroupType::SENSITIVE_DATA_SCANNER_GROUP)
                            ],
                        ),
                    ),
                )
                .type_(SensitiveDataScannerConfigurationType::SENSITIVE_DATA_SCANNER_CONFIGURATIONS),
            SensitiveDataScannerMetaVersionOnly::new(),
        );
    let configuration = Configuration::new();
    let api = SensitiveDataScannerAPI::with_config(configuration);
    let resp = api.reorder_scanning_groups(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

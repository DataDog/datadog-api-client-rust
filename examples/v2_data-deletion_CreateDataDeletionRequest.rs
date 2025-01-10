// Creates a data deletion request returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_data_deletion::DataDeletionAPI;
use datadog_api_client::datadogV2::model::CreateDataDeletionRequestBody;
use datadog_api_client::datadogV2::model::CreateDataDeletionRequestBodyAttributes;
use datadog_api_client::datadogV2::model::CreateDataDeletionRequestBodyData;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = CreateDataDeletionRequestBody::new(CreateDataDeletionRequestBodyData::new(
        CreateDataDeletionRequestBodyAttributes::new(
            1672527600000,
            BTreeMap::from([
                ("host".to_string(), "abc".to_string()),
                ("service".to_string(), "xyz".to_string()),
            ]),
            1704063600000,
        )
        .indexes(vec!["test-index".to_string(), "test-index-2".to_string()]),
    ));

    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDataDeletionRequest", true);
    let api = DataDeletionAPI::with_config(configuration);
    let resp = api
        .create_data_deletion_request("logs".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

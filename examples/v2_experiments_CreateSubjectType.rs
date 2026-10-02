// Create subject type returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_experiments::ExperimentsAPI;
use datadog_api_client::datadogV2::model::ExperimentsCreateSubjectTypeV2Request;
use datadog_api_client::datadogV2::model::ExperimentsCreateSubjectTypeV2RequestData;
use datadog_api_client::datadogV2::model::ExperimentsCreateSubjectTypeV2RequestDataAttributes;
use datadog_api_client::datadogV2::model::ExperimentsSubjectTypeV2DTODataType;

#[tokio::main]
async fn main() {
    let body =
        ExperimentsCreateSubjectTypeV2Request::new(ExperimentsCreateSubjectTypeV2RequestData::new(
            ExperimentsCreateSubjectTypeV2RequestDataAttributes::new(
                "ex-14bb9543f523edde".to_string(),
            )
            .product_analytics_attribute("@account.id".to_string())
            .warehouse_column_names(vec!["account_id".to_string()]),
            ExperimentsSubjectTypeV2DTODataType::SUBJECT_TYPES,
        ));
    let configuration = datadog::Configuration::new();
    let api = ExperimentsAPI::with_config(configuration);
    let resp = api.create_subject_type(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

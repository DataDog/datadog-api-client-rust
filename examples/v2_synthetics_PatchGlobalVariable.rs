// Patch a global variable returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequest;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequestData;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequestDataAttributes;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchType;
use datadog_api_client::datadogV2::model::JsonPatchOperation;
use datadog_api_client::datadogV2::model::JsonPatchOperationOp;

#[tokio::main]
async fn main() {
    let body = GlobalVariableJsonPatchRequest::new(
        GlobalVariableJsonPatchRequestData::new()
            .attributes(
                GlobalVariableJsonPatchRequestDataAttributes::new().json_patch(vec![
                    JsonPatchOperation::new(JsonPatchOperationOp::ADD, "/name".to_string()),
                ]),
            )
            .type_(GlobalVariableJsonPatchType::GLOBAL_VARIABLES_JSON_PATCH),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .patch_global_variable("variable_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

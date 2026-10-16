// Patch a persistent email global variable preserves its address and type
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequest;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequestData;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchRequestDataAttributes;
use datadog_api_client::datadogV2::model::GlobalVariableJsonPatchType;
use datadog_api_client::datadogV2::model::JsonPatchOperation;
use datadog_api_client::datadogV2::model::JsonPatchOperationOp;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_email_global_variable" in the system
    let synthetics_email_global_variable_id =
        std::env::var("SYNTHETICS_EMAIL_GLOBAL_VARIABLE_ID").unwrap();
    let body = GlobalVariableJsonPatchRequest::new(
        GlobalVariableJsonPatchRequestData::new()
            .attributes(
                GlobalVariableJsonPatchRequestDataAttributes::new().json_patch(vec![
                    JsonPatchOperation::new(
                        JsonPatchOperationOp::REPLACE,
                        "/description".to_string(),
                    )
                    .value(Value::from("Updated persistent email variable")),
                ]),
            )
            .type_(GlobalVariableJsonPatchType::GLOBAL_VARIABLES_JSON_PATCH),
    );
    let configuration = datadog::Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api
        .patch_global_variable(synthetics_email_global_variable_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

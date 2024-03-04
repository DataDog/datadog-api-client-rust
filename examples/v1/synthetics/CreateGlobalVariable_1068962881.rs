// Create a global variable from test returns "OK" response
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_synthetics::SyntheticsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    // there is a valid "synthetics_api_test_multi_step" in the system
    let synthetics_api_test_multi_step_public_id = std::env::var("SYNTHETICS_API_TEST_MULTI_STEP_PUBLIC_ID").unwrap();
    let body =
        SyntheticsGlobalVariable::new(
            "".to_string(),
            "GLOBAL_VARIABLE_PAYLOAD_EXAMPLESYNTHETIC".to_string(),
            vec![],
            SyntheticsGlobalVariableValue::new()
                .options(
                    SyntheticsGlobalVariableOptions
                    ::new().totp_parameters(
                        SyntheticsGlobalVariableTOTPParameters::new().digits(6).refresh_interval(30),
                    ),
                )
                .secure(false)
                .value("".to_string()),
        )
            .parse_test_options(
                SyntheticsGlobalVariableParseTestOptions::new(
                    SyntheticsGlobalVariableParseTestOptionsType::LOCAL_VARIABLE,
                ).local_variable_name("EXTRACTED_VALUE".to_string()),
            )
            .parse_test_public_id(synthetics_api_test_multi_step_public_id);
    let configuration = Configuration::new();
    let api = SyntheticsAPI::with_config(configuration);
    let resp = api.create_global_variable(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}

use crate::scenarios::fixtures::DatadogWorld;
use datadog_api_client::datadogV2::api_fastly_integration::*;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("CreateFastlyAccount".to_string(), TestCreateFastlyAccount);
    world
        .function_mappings
        .insert("DeleteFastlyAccount".to_string(), TestDeleteFastlyAccount);
}

fn TestCreateFastlyAccount(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = CreateFastlyAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(CreateFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => panic!("{:#?}", error),
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestDeleteFastlyAccount(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = DeleteFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(DeleteFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => panic!("{:#?}", error),
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

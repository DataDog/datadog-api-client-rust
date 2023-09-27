use crate::scenarios::fixtures::DatadogWorld;
use datadog_api_client::datadogV2::api_fastly_integration::*;
use futures::executor::block_on;
use std::collections::HashMap;

pub type TestCall = fn(&mut DatadogWorld);

pub fn GetTestCalls(world: &mut DatadogWorld) {
    let mut map: HashMap<String, TestCall> = HashMap::new();
    map.insert("CreateFastlyAccount".to_string(), TestCreateFastlyAccount);
    map.insert("DeleteFastlyAccount".to_string(), TestDeleteFastlyAccount);
    world.function_mappings = map;
}

fn TestCreateFastlyAccount(world: &mut DatadogWorld) {
    let params = CreateFastlyAccountParams {
        body: serde_json::from_value(world.body.clone()).unwrap(),
    };
    let response = match block_on(CreateFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => panic!("{:#?}", error),
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestDeleteFastlyAccount(world: &mut DatadogWorld) {
    let params = DeleteFastlyAccountParams {
        account_id: world.parameters.get("account_id").unwrap().to_string(),
    };
    let response = match block_on(DeleteFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => panic!("{:#?}", error),
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

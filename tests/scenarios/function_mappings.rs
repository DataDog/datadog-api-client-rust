use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;
use datadog_api_client::datadogV2::api_fastly_integration::*;

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("ListFastlyAccounts".to_string(), TestListFastlyAccounts);
    world
        .function_mappings
        .insert("CreateFastlyAccount".to_string(), TestCreateFastlyAccount);
    world
        .function_mappings
        .insert("DeleteFastlyAccount".to_string(), TestDeleteFastlyAccount);
    world
        .function_mappings
        .insert("GetFastlyAccount".to_string(), TestGetFastlyAccount);
    world
        .function_mappings
        .insert("UpdateFastlyAccount".to_string(), TestUpdateFastlyAccount);
    world
        .function_mappings
        .insert("ListFastlyServices".to_string(), TestListFastlyServices);
    world
        .function_mappings
        .insert("CreateFastlyService".to_string(), TestCreateFastlyService);
    world
        .function_mappings
        .insert("DeleteFastlyService".to_string(), TestDeleteFastlyService);
    world
        .function_mappings
        .insert("GetFastlyService".to_string(), TestGetFastlyService);
    world
        .function_mappings
        .insert("UpdateFastlyService".to_string(), TestUpdateFastlyService);
}

fn TestListFastlyAccounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let response = match block_on(ListFastlyAccounts(&world.config)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestCreateFastlyAccount(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = CreateFastlyAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(CreateFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
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
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestGetFastlyAccount(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = GetFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(GetFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestUpdateFastlyAccount(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = UpdateFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(UpdateFastlyAccount(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestListFastlyServices(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = ListFastlyServicesParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(ListFastlyServices(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestCreateFastlyService(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = CreateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(CreateFastlyService(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestDeleteFastlyService(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = DeleteFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(DeleteFastlyService(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestGetFastlyService(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = GetFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(GetFastlyService(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

fn TestUpdateFastlyService(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = UpdateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(UpdateFastlyService(&world.config, params)) {
        Ok(response) => response,
        Err(error) => {
            return match error {
                Error::Reqwest(e) => panic!("reqwest error: {}", e.to_string()),
                Error::Serde(e) => panic!("serde error: {}", e.to_string()),
                Error::Io(e) => panic!("io error: {}", e.to_string()),
                Error::ResponseError(e) => world.response.code = e.status.as_u16(),
            };
        }
    };
    world.response.object = serde_json::to_value(response.entity).unwrap();
    world.response.code = response.status.as_u16();
}

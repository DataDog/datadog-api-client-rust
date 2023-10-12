use crate::scenarios::fixtures::DatadogWorld;
use futures::executor::block_on;
use serde_json::Value;
use std::collections::HashMap;

use datadog_api_client::datadog::*;
use datadog_api_client::datadogV2::api::api_fastly_integration::*;

pub fn collect_function_calls(world: &mut DatadogWorld) {
    world
        .function_mappings
        .insert("ListFastlyAccounts".to_string(), test_list_fastly_accounts);
    world.function_mappings.insert(
        "CreateFastlyAccount".to_string(),
        test_create_fastly_account,
    );
    world.function_mappings.insert(
        "DeleteFastlyAccount".to_string(),
        test_delete_fastly_account,
    );
    world
        .function_mappings
        .insert("GetFastlyAccount".to_string(), test_get_fastly_account);
    world.function_mappings.insert(
        "UpdateFastlyAccount".to_string(),
        test_update_fastly_account,
    );
    world
        .function_mappings
        .insert("ListFastlyServices".to_string(), test_list_fastly_services);
    world.function_mappings.insert(
        "CreateFastlyService".to_string(),
        test_create_fastly_service,
    );
    world.function_mappings.insert(
        "DeleteFastlyService".to_string(),
        test_delete_fastly_service,
    );
    world
        .function_mappings
        .insert("GetFastlyService".to_string(), test_get_fastly_service);
    world.function_mappings.insert(
        "UpdateFastlyService".to_string(),
        test_update_fastly_service,
    );
}

fn test_list_fastly_accounts(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let response = match block_on(list_fastly_accounts(&world.config)) {
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

fn test_create_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = CreateFastlyAccountParams {
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(create_fastly_account(&world.config, params)) {
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

fn test_delete_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = DeleteFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(delete_fastly_account(&world.config, params)) {
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

fn test_get_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = GetFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(get_fastly_account(&world.config, params)) {
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

fn test_update_fastly_account(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = UpdateFastlyAccountParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(update_fastly_account(&world.config, params)) {
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

fn test_list_fastly_services(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = ListFastlyServicesParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(list_fastly_services(&world.config, params)) {
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

fn test_create_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = CreateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(create_fastly_service(&world.config, params)) {
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

fn test_delete_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = DeleteFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(delete_fastly_service(&world.config, params)) {
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

fn test_get_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = GetFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
    };
    let response = match block_on(get_fastly_service(&world.config, params)) {
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

fn test_update_fastly_service(world: &mut DatadogWorld, _parameters: &HashMap<String, Value>) {
    let params = UpdateFastlyServiceParams {
        account_id: serde_json::from_value(_parameters.get("account_id").unwrap().clone()).unwrap(),
        service_id: serde_json::from_value(_parameters.get("service_id").unwrap().clone()).unwrap(),
        body: serde_json::from_value(_parameters.get("body").unwrap().clone()).unwrap(),
    };
    let response = match block_on(update_fastly_service(&world.config, params)) {
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

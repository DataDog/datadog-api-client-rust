// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetAPITestLatestResultsOptionalParams is a struct for passing parameters to the method [`SyntheticsAPI::get_api_test_latest_results`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetAPITestLatestResultsOptionalParams {
    /// Timestamp in milliseconds from which to start querying results.
    pub from_ts: Option<i64>,
    /// Timestamp in milliseconds up to which to query results.
    pub to_ts: Option<i64>,
    /// Locations for which to query results.
    pub probe_dc: Option<Vec<String>>,
}

impl GetAPITestLatestResultsOptionalParams {
    /// Timestamp in milliseconds from which to start querying results.
    pub fn from_ts(mut self, value: i64) -> Self {
        self.from_ts = Some(value);
        self
    }
    /// Timestamp in milliseconds up to which to query results.
    pub fn to_ts(mut self, value: i64) -> Self {
        self.to_ts = Some(value);
        self
    }
    /// Locations for which to query results.
    pub fn probe_dc(mut self, value: Vec<String>) -> Self {
        self.probe_dc = Some(value);
        self
    }
}

/// GetBrowserTestLatestResultsOptionalParams is a struct for passing parameters to the method [`SyntheticsAPI::get_browser_test_latest_results`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetBrowserTestLatestResultsOptionalParams {
    /// Timestamp in milliseconds from which to start querying results.
    pub from_ts: Option<i64>,
    /// Timestamp in milliseconds up to which to query results.
    pub to_ts: Option<i64>,
    /// Locations for which to query results.
    pub probe_dc: Option<Vec<String>>,
}

impl GetBrowserTestLatestResultsOptionalParams {
    /// Timestamp in milliseconds from which to start querying results.
    pub fn from_ts(mut self, value: i64) -> Self {
        self.from_ts = Some(value);
        self
    }
    /// Timestamp in milliseconds up to which to query results.
    pub fn to_ts(mut self, value: i64) -> Self {
        self.to_ts = Some(value);
        self
    }
    /// Locations for which to query results.
    pub fn probe_dc(mut self, value: Vec<String>) -> Self {
        self.probe_dc = Some(value);
        self
    }
}

/// ListTestsOptionalParams is a struct for passing parameters to the method [`SyntheticsAPI::list_tests`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTestsOptionalParams {
    /// Used for pagination. The number of tests returned in the page.
    pub page_size: Option<i64>,
    /// Used for pagination. Which page you want to retrieve. Starts at zero.
    pub page_number: Option<i64>,
}

impl ListTestsOptionalParams {
    /// Used for pagination. The number of tests returned in the page.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Used for pagination. Which page you want to retrieve. Starts at zero.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
}

/// CreateGlobalVariableError is a struct for typed errors of method [`SyntheticsAPI::create_global_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreatePrivateLocationError is a struct for typed errors of method [`SyntheticsAPI::create_private_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePrivateLocationError {
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsAPITestError is a struct for typed errors of method [`SyntheticsAPI::create_synthetics_api_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsAPITestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsBrowserTestError is a struct for typed errors of method [`SyntheticsAPI::create_synthetics_browser_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsBrowserTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteGlobalVariableError is a struct for typed errors of method [`SyntheticsAPI::delete_global_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePrivateLocationError is a struct for typed errors of method [`SyntheticsAPI::delete_private_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTestsError is a struct for typed errors of method [`SyntheticsAPI::delete_tests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditGlobalVariableError is a struct for typed errors of method [`SyntheticsAPI::edit_global_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestError is a struct for typed errors of method [`SyntheticsAPI::get_api_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestLatestResultsError is a struct for typed errors of method [`SyntheticsAPI::get_api_test_latest_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestLatestResultsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestResultError is a struct for typed errors of method [`SyntheticsAPI::get_api_test_result`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestResultError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestError is a struct for typed errors of method [`SyntheticsAPI::get_browser_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestLatestResultsError is a struct for typed errors of method [`SyntheticsAPI::get_browser_test_latest_results`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestLatestResultsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestResultError is a struct for typed errors of method [`SyntheticsAPI::get_browser_test_result`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestResultError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetGlobalVariableError is a struct for typed errors of method [`SyntheticsAPI::get_global_variable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGlobalVariableError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPrivateLocationError is a struct for typed errors of method [`SyntheticsAPI::get_private_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsCIBatchError is a struct for typed errors of method [`SyntheticsAPI::get_synthetics_ci_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsCIBatchError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsDefaultLocationsError is a struct for typed errors of method [`SyntheticsAPI::get_synthetics_default_locations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsDefaultLocationsError {
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTestError is a struct for typed errors of method [`SyntheticsAPI::get_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListGlobalVariablesError is a struct for typed errors of method [`SyntheticsAPI::list_global_variables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGlobalVariablesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLocationsError is a struct for typed errors of method [`SyntheticsAPI::list_locations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLocationsError {
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTestsError is a struct for typed errors of method [`SyntheticsAPI::list_tests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTestsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// PatchTestError is a struct for typed errors of method [`SyntheticsAPI::patch_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// TriggerCITestsError is a struct for typed errors of method [`SyntheticsAPI::trigger_ci_tests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerCITestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// TriggerTestsError is a struct for typed errors of method [`SyntheticsAPI::trigger_tests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAPITestError is a struct for typed errors of method [`SyntheticsAPI::update_api_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPITestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateBrowserTestError is a struct for typed errors of method [`SyntheticsAPI::update_browser_test`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBrowserTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePrivateLocationError is a struct for typed errors of method [`SyntheticsAPI::update_private_location`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTestPauseStatusError is a struct for typed errors of method [`SyntheticsAPI::update_test_pause_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTestPauseStatusError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SyntheticsAPI {
    config: configuration::Configuration,
}

impl Default for SyntheticsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SyntheticsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a Synthetic global variable.
    pub async fn create_global_variable(
        &self,
        body: crate::datadogV1::model::SyntheticsGlobalVariable,
    ) -> Result<crate::datadogV1::model::SyntheticsGlobalVariable, Error<CreateGlobalVariableError>>
    {
        match self.create_global_variable_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic global variable.
    pub async fn create_global_variable_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsGlobalVariable,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<CreateGlobalVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_global_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsGlobalVariable>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateGlobalVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a new Synthetic private location.
    pub async fn create_private_location(
        &self,
        body: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> Result<
        crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse,
        Error<CreatePrivateLocationError>,
    > {
        match self.create_private_location_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a new Synthetic private location.
    pub async fn create_private_location_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse>,
        Error<CreatePrivateLocationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_private_location";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreatePrivateLocationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Synthetic API test.
    pub async fn create_synthetics_api_test(
        &self,
        body: crate::datadogV1::model::SyntheticsAPITest,
    ) -> Result<crate::datadogV1::model::SyntheticsAPITest, Error<CreateSyntheticsAPITestError>>
    {
        match self.create_synthetics_api_test_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic API test.
    pub async fn create_synthetics_api_test_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsAPITest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITest>,
        Error<CreateSyntheticsAPITestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_synthetics_api_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsAPITest>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSyntheticsAPITestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a Synthetic browser test.
    pub async fn create_synthetics_browser_test(
        &self,
        body: crate::datadogV1::model::SyntheticsBrowserTest,
    ) -> Result<
        crate::datadogV1::model::SyntheticsBrowserTest,
        Error<CreateSyntheticsBrowserTestError>,
    > {
        match self
            .create_synthetics_browser_test_with_http_info(body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic browser test.
    pub async fn create_synthetics_browser_test_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsBrowserTest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<CreateSyntheticsBrowserTestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_synthetics_browser_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsBrowserTest>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSyntheticsBrowserTestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Synthetic global variable.
    pub async fn delete_global_variable(
        &self,
        variable_id: String,
    ) -> Result<(), Error<DeleteGlobalVariableError>> {
        match self
            .delete_global_variable_with_http_info(variable_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Synthetic global variable.
    pub async fn delete_global_variable_with_http_info(
        &self,
        variable_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteGlobalVariableError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_global_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.get_operation_host(operation_id),
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteGlobalVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a Synthetic private location.
    pub async fn delete_private_location(
        &self,
        location_id: String,
    ) -> Result<(), Error<DeletePrivateLocationError>> {
        match self
            .delete_private_location_with_http_info(location_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a Synthetic private location.
    pub async fn delete_private_location_with_http_info(
        &self,
        location_id: String,
    ) -> Result<ResponseContent<()>, Error<DeletePrivateLocationError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_private_location";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.get_operation_host(operation_id),
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeletePrivateLocationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete multiple Synthetic tests by ID.
    pub async fn delete_tests(
        &self,
        body: crate::datadogV1::model::SyntheticsDeleteTestsPayload,
    ) -> Result<crate::datadogV1::model::SyntheticsDeleteTestsResponse, Error<DeleteTestsError>>
    {
        match self.delete_tests_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Delete multiple Synthetic tests by ID.
    pub async fn delete_tests_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsDeleteTestsPayload,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsDeleteTestsResponse>,
        Error<DeleteTestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_tests";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/delete",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsDeleteTestsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<DeleteTestsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit a Synthetic global variable.
    pub async fn edit_global_variable(
        &self,
        variable_id: String,
        body: crate::datadogV1::model::SyntheticsGlobalVariable,
    ) -> Result<crate::datadogV1::model::SyntheticsGlobalVariable, Error<EditGlobalVariableError>>
    {
        match self
            .edit_global_variable_with_http_info(variable_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Edit a Synthetic global variable.
    pub async fn edit_global_variable_with_http_info(
        &self,
        variable_id: String,
        body: crate::datadogV1::model::SyntheticsGlobalVariable,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<EditGlobalVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.edit_global_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.get_operation_host(operation_id),
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsGlobalVariable>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<EditGlobalVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the detailed configuration associated with
    /// a Synthetic API test.
    pub async fn get_api_test(
        &self,
        public_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsAPITest, Error<GetAPITestError>> {
        match self.get_api_test_with_http_info(public_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration associated with
    /// a Synthetic API test.
    pub async fn get_api_test_with_http_info(
        &self,
        public_id: String,
    ) -> Result<ResponseContent<crate::datadogV1::model::SyntheticsAPITest>, Error<GetAPITestError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.get_api_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsAPITest>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetAPITestError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic API test.
    pub async fn get_api_test_latest_results(
        &self,
        public_id: String,
        params: GetAPITestLatestResultsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse,
        Error<GetAPITestLatestResultsError>,
    > {
        match self
            .get_api_test_latest_results_with_http_info(public_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic API test.
    pub async fn get_api_test_latest_results_with_http_info(
        &self,
        public_id: String,
        params: GetAPITestLatestResultsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse>,
        Error<GetAPITestLatestResultsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_api_test_latest_results";

        // unbox and build optional parameters
        let from_ts = params.from_ts;
        let to_ts = params.to_ts;
        let probe_dc = params.probe_dc;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/results",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from_ts {
            local_req_builder =
                local_req_builder.query(&[("from_ts", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to_ts {
            local_req_builder =
                local_req_builder.query(&[("to_ts", &local_query_param.to_string())]);
        };
        if let Some(ref local) = probe_dc {
            local_req_builder = local_req_builder.query(&[(
                "probe_dc",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetAPITestLatestResultsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific full result from a given Synthetic API test.
    pub async fn get_api_test_result(
        &self,
        public_id: String,
        result_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsAPITestResultFull, Error<GetAPITestResultError>>
    {
        match self
            .get_api_test_result_with_http_info(public_id, result_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a specific full result from a given Synthetic API test.
    pub async fn get_api_test_result_with_http_info(
        &self,
        public_id: String,
        result_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITestResultFull>,
        Error<GetAPITestResultError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_api_test_result";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/results/{result_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id),
            result_id = urlencode(result_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsAPITestResultFull>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetAPITestResultError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the detailed configuration (including steps) associated with
    /// a Synthetic browser test.
    pub async fn get_browser_test(
        &self,
        public_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsBrowserTest, Error<GetBrowserTestError>> {
        match self.get_browser_test_with_http_info(public_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration (including steps) associated with
    /// a Synthetic browser test.
    pub async fn get_browser_test_with_http_info(
        &self,
        public_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<GetBrowserTestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_browser_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsBrowserTest>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBrowserTestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic browser test.
    pub async fn get_browser_test_latest_results(
        &self,
        public_id: String,
        params: GetBrowserTestLatestResultsOptionalParams,
    ) -> Result<
        crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse,
        Error<GetBrowserTestLatestResultsError>,
    > {
        match self
            .get_browser_test_latest_results_with_http_info(public_id, params)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic browser test.
    pub async fn get_browser_test_latest_results_with_http_info(
        &self,
        public_id: String,
        params: GetBrowserTestLatestResultsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse>,
        Error<GetBrowserTestLatestResultsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_browser_test_latest_results";

        // unbox and build optional parameters
        let from_ts = params.from_ts;
        let to_ts = params.to_ts;
        let probe_dc = params.probe_dc;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}/results",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = from_ts {
            local_req_builder =
                local_req_builder.query(&[("from_ts", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = to_ts {
            local_req_builder =
                local_req_builder.query(&[("to_ts", &local_query_param.to_string())]);
        };
        if let Some(ref local) = probe_dc {
            local_req_builder = local_req_builder.query(&[(
                "probe_dc",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBrowserTestLatestResultsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific full result from a given Synthetic browser test.
    pub async fn get_browser_test_result(
        &self,
        public_id: String,
        result_id: String,
    ) -> Result<
        crate::datadogV1::model::SyntheticsBrowserTestResultFull,
        Error<GetBrowserTestResultError>,
    > {
        match self
            .get_browser_test_result_with_http_info(public_id, result_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a specific full result from a given Synthetic browser test.
    pub async fn get_browser_test_result_with_http_info(
        &self,
        public_id: String,
        result_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTestResultFull>,
        Error<GetBrowserTestResultError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_browser_test_result";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}/results/{result_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id),
            result_id = urlencode(result_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsBrowserTestResultFull>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetBrowserTestResultError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the detailed configuration of a global variable.
    pub async fn get_global_variable(
        &self,
        variable_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsGlobalVariable, Error<GetGlobalVariableError>>
    {
        match self.get_global_variable_with_http_info(variable_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration of a global variable.
    pub async fn get_global_variable_with_http_info(
        &self,
        variable_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<GetGlobalVariableError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_global_variable";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.get_operation_host(operation_id),
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsGlobalVariable>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetGlobalVariableError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a Synthetic private location.
    pub async fn get_private_location(
        &self,
        location_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsPrivateLocation, Error<GetPrivateLocationError>>
    {
        match self.get_private_location_with_http_info(location_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a Synthetic private location.
    pub async fn get_private_location_with_http_info(
        &self,
        location_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<GetPrivateLocationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_private_location";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.get_operation_host(operation_id),
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsPrivateLocation>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetPrivateLocationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a batch's updated details.
    pub async fn get_synthetics_ci_batch(
        &self,
        batch_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsBatchDetails, Error<GetSyntheticsCIBatchError>>
    {
        match self.get_synthetics_ci_batch_with_http_info(batch_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get a batch's updated details.
    pub async fn get_synthetics_ci_batch_with_http_info(
        &self,
        batch_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBatchDetails>,
        Error<GetSyntheticsCIBatchError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_synthetics_ci_batch";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/ci/batch/{batch_id}",
            local_configuration.get_operation_host(operation_id),
            batch_id = urlencode(batch_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsBatchDetails>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSyntheticsCIBatchError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the default locations settings.
    pub async fn get_synthetics_default_locations(
        &self,
    ) -> Result<Vec<String>, Error<GetSyntheticsDefaultLocationsError>> {
        match self.get_synthetics_default_locations_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the default locations settings.
    pub async fn get_synthetics_default_locations_with_http_info(
        &self,
    ) -> Result<ResponseContent<Vec<String>>, Error<GetSyntheticsDefaultLocationsError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_synthetics_default_locations";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/settings/default_locations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<Vec<String>>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSyntheticsDefaultLocationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the detailed configuration associated with a Synthetic test.
    pub async fn get_test(
        &self,
        public_id: String,
    ) -> Result<crate::datadogV1::model::SyntheticsTestDetails, Error<GetTestError>> {
        match self.get_test_with_http_info(public_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration associated with a Synthetic test.
    pub async fn get_test_with_http_info(
        &self,
        public_id: String,
    ) -> Result<ResponseContent<crate::datadogV1::model::SyntheticsTestDetails>, Error<GetTestError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.get_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsTestDetails>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetTestError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of all Synthetic global variables.
    pub async fn list_global_variables(
        &self,
    ) -> Result<
        crate::datadogV1::model::SyntheticsListGlobalVariablesResponse,
        Error<ListGlobalVariablesError>,
    > {
        match self.list_global_variables_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the list of all Synthetic global variables.
    pub async fn list_global_variables_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsListGlobalVariablesResponse>,
        Error<ListGlobalVariablesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_global_variables";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<
                crate::datadogV1::model::SyntheticsListGlobalVariablesResponse,
            >(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListGlobalVariablesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of public and private locations available for Synthetic
    /// tests. No arguments required.
    pub async fn list_locations(
        &self,
    ) -> Result<crate::datadogV1::model::SyntheticsLocations, Error<ListLocationsError>> {
        match self.list_locations_with_http_info().await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get the list of public and private locations available for Synthetic
    /// tests. No arguments required.
    pub async fn list_locations_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsLocations>,
        Error<ListLocationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_locations";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/locations",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsLocations>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListLocationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of all Synthetic tests.
    pub async fn list_tests(
        &self,
        params: ListTestsOptionalParams,
    ) -> Result<crate::datadogV1::model::SyntheticsListTestsResponse, Error<ListTestsError>> {
        match self.list_tests_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_tests_with_pagination(
        &self,
        mut params: ListTestsOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV1::model::SyntheticsTestDetails, Error<ListTestsError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            if params.page_number.is_none() {
                params.page_number = Some(0);
            }
            loop {
                let resp = self.list_tests(params.clone()).await?;
                let Some(tests) = resp.tests else { break };

                let r = tests;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                params.page_number = Some(params.page_number.unwrap() + 1);
            }
        }
    }

    /// Get the list of all Synthetic tests.
    pub async fn list_tests_with_http_info(
        &self,
        params: ListTestsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsListTestsResponse>,
        Error<ListTestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_tests";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page_size", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page_number", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsListTestsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListTestsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Patch the configuration of a Synthetic test with partial data.
    pub async fn patch_test(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsPatchTestBody,
    ) -> Result<crate::datadogV1::model::SyntheticsTestDetails, Error<PatchTestError>> {
        match self.patch_test_with_http_info(public_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Patch the configuration of a Synthetic test with partial data.
    pub async fn patch_test_with_http_info(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsPatchTestBody,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTestDetails>,
        Error<PatchTestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.patch_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsTestDetails>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<PatchTestError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Trigger a set of Synthetic tests for continuous integration.
    pub async fn trigger_ci_tests(
        &self,
        body: crate::datadogV1::model::SyntheticsCITestBody,
    ) -> Result<crate::datadogV1::model::SyntheticsTriggerCITestsResponse, Error<TriggerCITestsError>>
    {
        match self.trigger_ci_tests_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Trigger a set of Synthetic tests for continuous integration.
    pub async fn trigger_ci_tests_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsCITestBody,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerCITestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.trigger_ci_tests";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/trigger/ci",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<TriggerCITestsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Trigger a set of Synthetic tests.
    pub async fn trigger_tests(
        &self,
        body: crate::datadogV1::model::SyntheticsTriggerBody,
    ) -> Result<crate::datadogV1::model::SyntheticsTriggerCITestsResponse, Error<TriggerTestsError>>
    {
        match self.trigger_tests_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Trigger a set of Synthetic tests.
    pub async fn trigger_tests_with_http_info(
        &self,
        body: crate::datadogV1::model::SyntheticsTriggerBody,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerTestsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.trigger_tests";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/trigger",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<TriggerTestsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit the configuration of a Synthetic API test.
    pub async fn update_api_test(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsAPITest,
    ) -> Result<crate::datadogV1::model::SyntheticsAPITest, Error<UpdateAPITestError>> {
        match self.update_api_test_with_http_info(public_id, body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Edit the configuration of a Synthetic API test.
    pub async fn update_api_test_with_http_info(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsAPITest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITest>,
        Error<UpdateAPITestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_api_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsAPITest>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateAPITestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit the configuration of a Synthetic browser test.
    pub async fn update_browser_test(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsBrowserTest,
    ) -> Result<crate::datadogV1::model::SyntheticsBrowserTest, Error<UpdateBrowserTestError>> {
        match self
            .update_browser_test_with_http_info(public_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Edit the configuration of a Synthetic browser test.
    pub async fn update_browser_test_with_http_info(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsBrowserTest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<UpdateBrowserTestError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_browser_test";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsBrowserTest>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateBrowserTestError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit a Synthetic private location.
    pub async fn update_private_location(
        &self,
        location_id: String,
        body: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> Result<crate::datadogV1::model::SyntheticsPrivateLocation, Error<UpdatePrivateLocationError>>
    {
        match self
            .update_private_location_with_http_info(location_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Edit a Synthetic private location.
    pub async fn update_private_location_with_http_info(
        &self,
        location_id: String,
        body: crate::datadogV1::model::SyntheticsPrivateLocation,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<UpdatePrivateLocationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_private_location";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.get_operation_host(operation_id),
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SyntheticsPrivateLocation>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdatePrivateLocationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Pause or start a Synthetic test by changing the status.
    pub async fn update_test_pause_status(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsUpdateTestPauseStatusPayload,
    ) -> Result<bool, Error<UpdateTestPauseStatusError>> {
        match self
            .update_test_pause_status_with_http_info(public_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Pause or start a Synthetic test by changing the status.
    pub async fn update_test_pause_status_with_http_info(
        &self,
        public_id: String,
        body: crate::datadogV1::model::SyntheticsUpdateTestPauseStatusPayload,
    ) -> Result<ResponseContent<bool>, Error<UpdateTestPauseStatusError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.update_test_pause_status";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/status",
            local_configuration.get_operation_host(operation_id),
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            local_req_builder = local_req_builder.body(ser.into_inner());
        }

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<bool>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateTestPauseStatusError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}

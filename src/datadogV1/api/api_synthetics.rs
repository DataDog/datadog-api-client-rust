// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateGlobalVariableParams is a struct for passing parameters to the method [`CreateGlobalVariable`]
#[derive(Clone, Debug)]
pub struct CreateGlobalVariableParams {
    /// Details of the global variable to create.
    pub body: crate::datadogV1::model::SyntheticsGlobalVariable,
}

/// CreatePrivateLocationParams is a struct for passing parameters to the method [`CreatePrivateLocation`]
#[derive(Clone, Debug)]
pub struct CreatePrivateLocationParams {
    /// Details of the private location to create.
    pub body: crate::datadogV1::model::SyntheticsPrivateLocation,
}

/// CreateSyntheticsAPITestParams is a struct for passing parameters to the method [`CreateSyntheticsAPITest`]
#[derive(Clone, Debug)]
pub struct CreateSyntheticsAPITestParams {
    /// Details of the test to create.
    pub body: crate::datadogV1::model::SyntheticsAPITest,
}

/// CreateSyntheticsBrowserTestParams is a struct for passing parameters to the method [`CreateSyntheticsBrowserTest`]
#[derive(Clone, Debug)]
pub struct CreateSyntheticsBrowserTestParams {
    /// Details of the test to create.
    pub body: crate::datadogV1::model::SyntheticsBrowserTest,
}

/// DeleteGlobalVariableParams is a struct for passing parameters to the method [`DeleteGlobalVariable`]
#[derive(Clone, Debug)]
pub struct DeleteGlobalVariableParams {
    /// The ID of the global variable.
    pub variable_id: String,
}

/// DeletePrivateLocationParams is a struct for passing parameters to the method [`DeletePrivateLocation`]
#[derive(Clone, Debug)]
pub struct DeletePrivateLocationParams {
    /// The ID of the private location.
    pub location_id: String,
}

/// DeleteTestsParams is a struct for passing parameters to the method [`DeleteTests`]
#[derive(Clone, Debug)]
pub struct DeleteTestsParams {
    /// Public ID list of the Synthetic tests to be deleted.
    pub body: crate::datadogV1::model::SyntheticsDeleteTestsPayload,
}

/// EditGlobalVariableParams is a struct for passing parameters to the method [`EditGlobalVariable`]
#[derive(Clone, Debug)]
pub struct EditGlobalVariableParams {
    /// The ID of the global variable.
    pub variable_id: String,
    /// Details of the global variable to update.
    pub body: crate::datadogV1::model::SyntheticsGlobalVariable,
}

/// GetAPITestParams is a struct for passing parameters to the method [`GetAPITest`]
#[derive(Clone, Debug)]
pub struct GetAPITestParams {
    /// The public ID of the test to get details from.
    pub public_id: String,
}

/// GetAPITestLatestResultsParams is a struct for passing parameters to the method [`GetAPITestLatestResults`]
#[derive(Clone, Debug)]
pub struct GetAPITestLatestResultsParams {
    /// The public ID of the test for which to search results for.
    pub public_id: String,
    /// Timestamp in milliseconds from which to start querying results.
    pub from_ts: Option<i64>,
    /// Timestamp in milliseconds up to which to query results.
    pub to_ts: Option<i64>,
    /// Locations for which to query results.
    pub probe_dc: Option<Vec<String>>,
}

/// GetAPITestResultParams is a struct for passing parameters to the method [`GetAPITestResult`]
#[derive(Clone, Debug)]
pub struct GetAPITestResultParams {
    /// The public ID of the API test to which the target result belongs.
    pub public_id: String,
    /// The ID of the result to get.
    pub result_id: String,
}

/// GetBrowserTestParams is a struct for passing parameters to the method [`GetBrowserTest`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestParams {
    /// The public ID of the test to get details from.
    pub public_id: String,
}

/// GetBrowserTestLatestResultsParams is a struct for passing parameters to the method [`GetBrowserTestLatestResults`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestLatestResultsParams {
    /// The public ID of the browser test for which to search results
    /// for.
    pub public_id: String,
    /// Timestamp in milliseconds from which to start querying results.
    pub from_ts: Option<i64>,
    /// Timestamp in milliseconds up to which to query results.
    pub to_ts: Option<i64>,
    /// Locations for which to query results.
    pub probe_dc: Option<Vec<String>>,
}

/// GetBrowserTestResultParams is a struct for passing parameters to the method [`GetBrowserTestResult`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestResultParams {
    /// The public ID of the browser test to which the target result
    /// belongs.
    pub public_id: String,
    /// The ID of the result to get.
    pub result_id: String,
}

/// GetGlobalVariableParams is a struct for passing parameters to the method [`GetGlobalVariable`]
#[derive(Clone, Debug)]
pub struct GetGlobalVariableParams {
    /// The ID of the global variable.
    pub variable_id: String,
}

/// GetPrivateLocationParams is a struct for passing parameters to the method [`GetPrivateLocation`]
#[derive(Clone, Debug)]
pub struct GetPrivateLocationParams {
    /// The ID of the private location.
    pub location_id: String,
}

/// GetSyntheticsCIBatchParams is a struct for passing parameters to the method [`GetSyntheticsCIBatch`]
#[derive(Clone, Debug)]
pub struct GetSyntheticsCIBatchParams {
    /// The ID of the batch.
    pub batch_id: String,
}

/// GetTestParams is a struct for passing parameters to the method [`GetTest`]
#[derive(Clone, Debug)]
pub struct GetTestParams {
    /// The public ID of the test to get details from.
    pub public_id: String,
}

/// ListTestsParams is a struct for passing parameters to the method [`ListTests`]
#[derive(Clone, Debug)]
pub struct ListTestsParams {
    /// Used for pagination. The number of tests returned in the page.
    pub page_size: Option<i64>,
    /// Used for pagination. Which page you want to retrieve. Starts at zero.
    pub page_number: Option<i64>,
}

/// PatchTestParams is a struct for passing parameters to the method [`PatchTest`]
#[derive(Clone, Debug)]
pub struct PatchTestParams {
    /// The public ID of the test to patch.
    pub public_id: String,
    /// [JSON Patch](https://jsonpatch.com/) compliant list of operations
    pub body: crate::datadogV1::model::SyntheticsPatchTestBody,
}

/// TriggerCITestsParams is a struct for passing parameters to the method [`TriggerCITests`]
#[derive(Clone, Debug)]
pub struct TriggerCITestsParams {
    /// Details of the test to trigger.
    pub body: crate::datadogV1::model::SyntheticsCITestBody,
}

/// TriggerTestsParams is a struct for passing parameters to the method [`TriggerTests`]
#[derive(Clone, Debug)]
pub struct TriggerTestsParams {
    /// The identifiers of the tests to trigger.
    pub body: crate::datadogV1::model::SyntheticsTriggerBody,
}

/// UpdateAPITestParams is a struct for passing parameters to the method [`UpdateAPITest`]
#[derive(Clone, Debug)]
pub struct UpdateAPITestParams {
    /// The public ID of the test to get details from.
    pub public_id: String,
    /// New test details to be saved.
    pub body: crate::datadogV1::model::SyntheticsAPITest,
}

/// UpdateBrowserTestParams is a struct for passing parameters to the method [`UpdateBrowserTest`]
#[derive(Clone, Debug)]
pub struct UpdateBrowserTestParams {
    /// The public ID of the test to edit.
    pub public_id: String,
    /// New test details to be saved.
    pub body: crate::datadogV1::model::SyntheticsBrowserTest,
}

/// UpdatePrivateLocationParams is a struct for passing parameters to the method [`UpdatePrivateLocation`]
#[derive(Clone, Debug)]
pub struct UpdatePrivateLocationParams {
    /// The ID of the private location.
    pub location_id: String,
    /// Details of the private location to be updated.
    pub body: crate::datadogV1::model::SyntheticsPrivateLocation,
}

/// UpdateTestPauseStatusParams is a struct for passing parameters to the method [`UpdateTestPauseStatus`]
#[derive(Clone, Debug)]
pub struct UpdateTestPauseStatusParams {
    /// The public ID of the Synthetic test to update.
    pub public_id: String,
    /// Status to set the given Synthetic test to.
    pub body: crate::datadogV1::model::SyntheticsUpdateTestPauseStatusPayload,
}

/// CreateGlobalVariableError is a struct for typed errors of method [`CreateGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreatePrivateLocationError is a struct for typed errors of method [`CreatePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePrivateLocationError {
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsAPITestError is a struct for typed errors of method [`CreateSyntheticsAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsAPITestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsBrowserTestError is a struct for typed errors of method [`CreateSyntheticsBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsBrowserTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status402(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteGlobalVariableError is a struct for typed errors of method [`DeleteGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeletePrivateLocationError is a struct for typed errors of method [`DeletePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTestsError is a struct for typed errors of method [`DeleteTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditGlobalVariableError is a struct for typed errors of method [`EditGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditGlobalVariableError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestError is a struct for typed errors of method [`GetAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestLatestResultsError is a struct for typed errors of method [`GetAPITestLatestResults`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestLatestResultsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetAPITestResultError is a struct for typed errors of method [`GetAPITestResult`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestResultError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestError is a struct for typed errors of method [`GetBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestLatestResultsError is a struct for typed errors of method [`GetBrowserTestLatestResults`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestLatestResultsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestResultError is a struct for typed errors of method [`GetBrowserTestResult`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestResultError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetGlobalVariableError is a struct for typed errors of method [`GetGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGlobalVariableError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetPrivateLocationError is a struct for typed errors of method [`GetPrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsCIBatchError is a struct for typed errors of method [`GetSyntheticsCIBatch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsCIBatchError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsDefaultLocationsError is a struct for typed errors of method [`GetSyntheticsDefaultLocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsDefaultLocationsError {
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTestError is a struct for typed errors of method [`GetTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTestError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListGlobalVariablesError is a struct for typed errors of method [`ListGlobalVariables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGlobalVariablesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLocationsError is a struct for typed errors of method [`ListLocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLocationsError {
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTestsError is a struct for typed errors of method [`ListTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTestsError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// PatchTestError is a struct for typed errors of method [`PatchTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// TriggerCITestsError is a struct for typed errors of method [`TriggerCITests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerCITestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// TriggerTestsError is a struct for typed errors of method [`TriggerTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTestsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateAPITestError is a struct for typed errors of method [`UpdateAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPITestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateBrowserTestError is a struct for typed errors of method [`UpdateBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBrowserTestError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdatePrivateLocationError is a struct for typed errors of method [`UpdatePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePrivateLocationError {
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTestPauseStatusError is a struct for typed errors of method [`UpdateTestPauseStatus`]
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
        params: CreateGlobalVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<CreateGlobalVariableError>,
    > {
        match self.create_global_variable_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic global variable.
    pub async fn create_global_variable_with_http_info(
        &self,
        params: CreateGlobalVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<CreateGlobalVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsGlobalVariable> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreatePrivateLocationParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse>,
        Error<CreatePrivateLocationError>,
    > {
        match self.create_private_location_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new Synthetic private location.
    pub async fn create_private_location_with_http_info(
        &self,
        params: CreatePrivateLocationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse>,
        Error<CreatePrivateLocationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<
                crate::datadogV1::model::SyntheticsPrivateLocationCreationResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateSyntheticsAPITestParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsAPITest>,
        Error<CreateSyntheticsAPITestError>,
    > {
        match self.create_synthetics_api_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic API test.
    pub async fn create_synthetics_api_test_with_http_info(
        &self,
        params: CreateSyntheticsAPITestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITest>,
        Error<CreateSyntheticsAPITestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsAPITest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: CreateSyntheticsBrowserTestParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<CreateSyntheticsBrowserTestError>,
    > {
        match self
            .create_synthetics_browser_test_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a Synthetic browser test.
    pub async fn create_synthetics_browser_test_with_http_info(
        &self,
        params: CreateSyntheticsBrowserTestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<CreateSyntheticsBrowserTestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsBrowserTest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: DeleteGlobalVariableParams,
    ) -> Result<Option<()>, Error<DeleteGlobalVariableError>> {
        match self.delete_global_variable_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Synthetic global variable.
    pub async fn delete_global_variable_with_http_info(
        &self,
        params: DeleteGlobalVariableParams,
    ) -> Result<ResponseContent<()>, Error<DeleteGlobalVariableError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let variable_id = params.variable_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.base_path,
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
        params: DeletePrivateLocationParams,
    ) -> Result<Option<()>, Error<DeletePrivateLocationError>> {
        match self.delete_private_location_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a Synthetic private location.
    pub async fn delete_private_location_with_http_info(
        &self,
        params: DeletePrivateLocationParams,
    ) -> Result<ResponseContent<()>, Error<DeletePrivateLocationError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let location_id = params.location_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.base_path,
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
        params: DeleteTestsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsDeleteTestsResponse>,
        Error<DeleteTestsError>,
    > {
        match self.delete_tests_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete multiple Synthetic tests by ID.
    pub async fn delete_tests_with_http_info(
        &self,
        params: DeleteTestsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsDeleteTestsResponse>,
        Error<DeleteTestsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/delete",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsDeleteTestsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: EditGlobalVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<EditGlobalVariableError>,
    > {
        match self.edit_global_variable_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit a Synthetic global variable.
    pub async fn edit_global_variable_with_http_info(
        &self,
        params: EditGlobalVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<EditGlobalVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let variable_id = params.variable_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.base_path,
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsGlobalVariable> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetAPITestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsAPITest>, Error<GetAPITestError>> {
        match self.get_api_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration associated with
    /// a Synthetic API test.
    pub async fn get_api_test_with_http_info(
        &self,
        params: GetAPITestParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SyntheticsAPITest>, Error<GetAPITestError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsAPITest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetAPITestLatestResultsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse>,
        Error<GetAPITestLatestResultsError>,
    > {
        match self
            .get_api_test_latest_results_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic API test.
    pub async fn get_api_test_latest_results_with_http_info(
        &self,
        params: GetAPITestLatestResultsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse>,
        Error<GetAPITestLatestResultsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let from_ts = params.from_ts;
        let to_ts = params.to_ts;
        let probe_dc = params.probe_dc;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/results",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = from_ts {
            local_req_builder = local_req_builder.query(&[("from_ts", &local_str.to_string())]);
        };
        if let Some(ref local_str) = to_ts {
            local_req_builder = local_req_builder.query(&[("to_ts", &local_str.to_string())]);
        };
        if let Some(ref local) = probe_dc {
            local_req_builder = local_req_builder.query(&[(
                "probe_dc",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV1::model::SyntheticsGetAPITestLatestResultsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetAPITestResultParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsAPITestResultFull>,
        Error<GetAPITestResultError>,
    > {
        match self.get_api_test_result_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific full result from a given Synthetic API test.
    pub async fn get_api_test_result_with_http_info(
        &self,
        params: GetAPITestResultParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITestResultFull>,
        Error<GetAPITestResultError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let result_id = params.result_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/results/{result_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id),
            result_id = urlencode(result_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsAPITestResultFull> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetBrowserTestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsBrowserTest>, Error<GetBrowserTestError>>
    {
        match self.get_browser_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration (including steps) associated with
    /// a Synthetic browser test.
    pub async fn get_browser_test_with_http_info(
        &self,
        params: GetBrowserTestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<GetBrowserTestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsBrowserTest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetBrowserTestLatestResultsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse>,
        Error<GetBrowserTestLatestResultsError>,
    > {
        match self
            .get_browser_test_latest_results_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the last 150 test results summaries for a given Synthetic browser test.
    pub async fn get_browser_test_latest_results_with_http_info(
        &self,
        params: GetBrowserTestLatestResultsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse>,
        Error<GetBrowserTestLatestResultsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let from_ts = params.from_ts;
        let to_ts = params.to_ts;
        let probe_dc = params.probe_dc;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}/results",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = from_ts {
            local_req_builder = local_req_builder.query(&[("from_ts", &local_str.to_string())]);
        };
        if let Some(ref local_str) = to_ts {
            local_req_builder = local_req_builder.query(&[("to_ts", &local_str.to_string())]);
        };
        if let Some(ref local) = probe_dc {
            local_req_builder = local_req_builder.query(&[(
                "probe_dc",
                &local
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV1::model::SyntheticsGetBrowserTestLatestResultsResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetBrowserTestResultParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsBrowserTestResultFull>,
        Error<GetBrowserTestResultError>,
    > {
        match self.get_browser_test_result_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific full result from a given Synthetic browser test.
    pub async fn get_browser_test_result_with_http_info(
        &self,
        params: GetBrowserTestResultParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTestResultFull>,
        Error<GetBrowserTestResultError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let result_id = params.result_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}/results/{result_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id),
            result_id = urlencode(result_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsBrowserTestResultFull> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetGlobalVariableParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<GetGlobalVariableError>,
    > {
        match self.get_global_variable_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration of a global variable.
    pub async fn get_global_variable_with_http_info(
        &self,
        params: GetGlobalVariableParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsGlobalVariable>,
        Error<GetGlobalVariableError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let variable_id = params.variable_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables/{variable_id}",
            local_configuration.base_path,
            variable_id = urlencode(variable_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsGlobalVariable> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetPrivateLocationParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<GetPrivateLocationError>,
    > {
        match self.get_private_location_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a Synthetic private location.
    pub async fn get_private_location_with_http_info(
        &self,
        params: GetPrivateLocationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<GetPrivateLocationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let location_id = params.location_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.base_path,
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsPrivateLocation> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetSyntheticsCIBatchParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsBatchDetails>,
        Error<GetSyntheticsCIBatchError>,
    > {
        match self.get_synthetics_ci_batch_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a batch's updated details.
    pub async fn get_synthetics_ci_batch_with_http_info(
        &self,
        params: GetSyntheticsCIBatchParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBatchDetails>,
        Error<GetSyntheticsCIBatchError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let batch_id = params.batch_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/ci/batch/{batch_id}",
            local_configuration.base_path,
            batch_id = urlencode(batch_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsBatchDetails> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<Vec<String>>, Error<GetSyntheticsDefaultLocationsError>> {
        match self.get_synthetics_default_locations_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the default locations settings.
    pub async fn get_synthetics_default_locations_with_http_info(
        &self,
    ) -> Result<ResponseContent<Vec<String>>, Error<GetSyntheticsDefaultLocationsError>> {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/settings/default_locations",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<Vec<String>> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: GetTestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsTestDetails>, Error<GetTestError>> {
        match self.get_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the detailed configuration associated with a Synthetic test.
    pub async fn get_test_with_http_info(
        &self,
        params: GetTestParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::SyntheticsTestDetails>, Error<GetTestError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsTestDetails> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        Option<crate::datadogV1::model::SyntheticsListGlobalVariablesResponse>,
        Error<ListGlobalVariablesError>,
    > {
        match self.list_global_variables_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
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

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/variables",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<
                crate::datadogV1::model::SyntheticsListGlobalVariablesResponse,
            > = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<crate::datadogV1::model::SyntheticsLocations>, Error<ListLocationsError>>
    {
        match self.list_locations_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
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

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/locations",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsLocations> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: ListTestsParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsListTestsResponse>, Error<ListTestsError>>
    {
        match self.list_tests_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of all Synthetic tests.
    pub async fn list_tests_with_http_info(
        &self,
        params: ListTestsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsListTestsResponse>,
        Error<ListTestsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/synthetics/tests", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page_size", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder = local_req_builder.query(&[("page_number", &local_str.to_string())]);
        };

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::SyntheticsListTestsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: PatchTestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsTestDetails>, Error<PatchTestError>> {
        match self.patch_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Patch the configuration of a Synthetic test with partial data.
    pub async fn patch_test_with_http_info(
        &self,
        params: PatchTestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTestDetails>,
        Error<PatchTestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsTestDetails> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: TriggerCITestsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerCITestsError>,
    > {
        match self.trigger_ci_tests_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Trigger a set of Synthetic tests for continuous integration.
    pub async fn trigger_ci_tests_with_http_info(
        &self,
        params: TriggerCITestsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerCITestsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/trigger/ci",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsTriggerCITestsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: TriggerTestsParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerTestsError>,
    > {
        match self.trigger_tests_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Trigger a set of Synthetic tests.
    pub async fn trigger_tests_with_http_info(
        &self,
        params: TriggerTestsParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsTriggerCITestsResponse>,
        Error<TriggerTestsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/trigger",
            local_configuration.base_path
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsTriggerCITestsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateAPITestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsAPITest>, Error<UpdateAPITestError>> {
        match self.update_api_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit the configuration of a Synthetic API test.
    pub async fn update_api_test_with_http_info(
        &self,
        params: UpdateAPITestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsAPITest>,
        Error<UpdateAPITestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/api/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsAPITest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateBrowserTestParams,
    ) -> Result<Option<crate::datadogV1::model::SyntheticsBrowserTest>, Error<UpdateBrowserTestError>>
    {
        match self.update_browser_test_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit the configuration of a Synthetic browser test.
    pub async fn update_browser_test_with_http_info(
        &self,
        params: UpdateBrowserTestParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsBrowserTest>,
        Error<UpdateBrowserTestError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/browser/{public_id}",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsBrowserTest> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdatePrivateLocationParams,
    ) -> Result<
        Option<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<UpdatePrivateLocationError>,
    > {
        match self.update_private_location_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit a Synthetic private location.
    pub async fn update_private_location_with_http_info(
        &self,
        params: UpdatePrivateLocationParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SyntheticsPrivateLocation>,
        Error<UpdatePrivateLocationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let location_id = params.location_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/private-locations/{location_id}",
            local_configuration.base_path,
            location_id = urlencode(location_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<crate::datadogV1::model::SyntheticsPrivateLocation> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
        params: UpdateTestPauseStatusParams,
    ) -> Result<Option<bool>, Error<UpdateTestPauseStatusError>> {
        match self.update_test_pause_status_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Pause or start a Synthetic test by changing the status.
    pub async fn update_test_pause_status_with_http_info(
        &self,
        params: UpdateTestPauseStatusParams,
    ) -> Result<ResponseContent<bool>, Error<UpdateTestPauseStatusError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let public_id = params.public_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/synthetics/tests/{public_id}/status",
            local_configuration.base_path,
            public_id = urlencode(public_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        if let Some(ref local_user_agent) = local_configuration.user_agent {
            local_req_builder =
                local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
        }

        // build auth
        if let Some(ref local_apikey) = local_configuration.api_key_auth {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_apikey);
        };
        if let Some(ref local_apikey) = local_configuration.app_key_auth {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_apikey);
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
            let local_entity: Option<bool> = serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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

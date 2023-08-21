// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateGlobalVariableParams is a struct for passing parameters to the method [`CreateGlobalVariable`]
#[derive(Clone, Debug)]
pub struct CreateGlobalVariableParams {
    /* Details of the global variable to create. */
    pub body: SyntheticsGlobalVariable,
}

// CreatePrivateLocationParams is a struct for passing parameters to the method [`CreatePrivateLocation`]
#[derive(Clone, Debug)]
pub struct CreatePrivateLocationParams {
    /* Details of the private location to create. */
    pub body: SyntheticsPrivateLocation,
}

// CreateSyntheticsAPITestParams is a struct for passing parameters to the method [`CreateSyntheticsAPITest`]
#[derive(Clone, Debug)]
pub struct CreateSyntheticsAPITestParams {
    /* Details of the test to create. */
    pub body: SyntheticsAPITest,
}

// CreateSyntheticsBrowserTestParams is a struct for passing parameters to the method [`CreateSyntheticsBrowserTest`]
#[derive(Clone, Debug)]
pub struct CreateSyntheticsBrowserTestParams {
    /* Details of the test to create. */
    pub body: SyntheticsBrowserTest,
}

// DeleteGlobalVariableParams is a struct for passing parameters to the method [`DeleteGlobalVariable`]
#[derive(Clone, Debug)]
pub struct DeleteGlobalVariableParams {
    /* The ID of the global variable. */
    pub variable_id: String,
}

// DeletePrivateLocationParams is a struct for passing parameters to the method [`DeletePrivateLocation`]
#[derive(Clone, Debug)]
pub struct DeletePrivateLocationParams {
    /* The ID of the private location. */
    pub location_id: String,
}

// DeleteTestsParams is a struct for passing parameters to the method [`DeleteTests`]
#[derive(Clone, Debug)]
pub struct DeleteTestsParams {
    /* Public ID list of the Synthetic tests to be deleted. */
    pub body: SyntheticsDeleteTestsPayload,
}

// EditGlobalVariableParams is a struct for passing parameters to the method [`EditGlobalVariable`]
#[derive(Clone, Debug)]
pub struct EditGlobalVariableParams {
    /* The ID of the global variable. */
    pub variable_id: String,
    /* Details of the global variable to update. */
    pub body: SyntheticsGlobalVariable,
}

// GetAPITestParams is a struct for passing parameters to the method [`GetAPITest`]
#[derive(Clone, Debug)]
pub struct GetAPITestParams {
    /* The public ID of the test to get details from. */
    pub public_id: String,
}

// GetAPITestLatestResultsParams is a struct for passing parameters to the method [`GetAPITestLatestResults`]
#[derive(Clone, Debug)]
pub struct GetAPITestLatestResultsParams {
    /* The public ID of the test for which to search results for. */
    pub public_id: String,
    /* Timestamp in milliseconds from which to start querying results. */
    pub from_ts: i64,
    /* Timestamp in milliseconds up to which to query results. */
    pub to_ts: i64,
    /* Locations for which to query results. */
    pub probe_dc: Vec<String>,
}

// GetAPITestResultParams is a struct for passing parameters to the method [`GetAPITestResult`]
#[derive(Clone, Debug)]
pub struct GetAPITestResultParams {
    /* The public ID of the API test to which the target result belongs. */
    pub public_id: String,
    /* The ID of the result to get. */
    pub result_id: String,
}

// GetBrowserTestParams is a struct for passing parameters to the method [`GetBrowserTest`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestParams {
    /* The public ID of the test to get details from. */
    pub public_id: String,
}

// GetBrowserTestLatestResultsParams is a struct for passing parameters to the method [`GetBrowserTestLatestResults`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestLatestResultsParams {
    /* The public ID of the browser test for which to search results
for. */
    pub public_id: String,
    /* Timestamp in milliseconds from which to start querying results. */
    pub from_ts: i64,
    /* Timestamp in milliseconds up to which to query results. */
    pub to_ts: i64,
    /* Locations for which to query results. */
    pub probe_dc: Vec<String>,
}

// GetBrowserTestResultParams is a struct for passing parameters to the method [`GetBrowserTestResult`]
#[derive(Clone, Debug)]
pub struct GetBrowserTestResultParams {
    /* The public ID of the browser test to which the target result
belongs. */
    pub public_id: String,
    /* The ID of the result to get. */
    pub result_id: String,
}

// GetGlobalVariableParams is a struct for passing parameters to the method [`GetGlobalVariable`]
#[derive(Clone, Debug)]
pub struct GetGlobalVariableParams {
    /* The ID of the global variable. */
    pub variable_id: String,
}

// GetPrivateLocationParams is a struct for passing parameters to the method [`GetPrivateLocation`]
#[derive(Clone, Debug)]
pub struct GetPrivateLocationParams {
    /* The ID of the private location. */
    pub location_id: String,
}

// GetSyntheticsCIBatchParams is a struct for passing parameters to the method [`GetSyntheticsCIBatch`]
#[derive(Clone, Debug)]
pub struct GetSyntheticsCIBatchParams {
    /* The ID of the batch. */
    pub batch_id: String,
}

// GetTestParams is a struct for passing parameters to the method [`GetTest`]
#[derive(Clone, Debug)]
pub struct GetTestParams {
    /* The public ID of the test to get details from. */
    pub public_id: String,
}

// ListTestsParams is a struct for passing parameters to the method [`ListTests`]
#[derive(Clone, Debug)]
pub struct ListTestsParams {
    /* Used for pagination. The number of tests returned in the page. */
    pub page_size: String,
    /* Used for pagination. Which page you want to retrieve. Starts at zero. */
    pub page_number: String,
}

// TriggerCITestsParams is a struct for passing parameters to the method [`TriggerCITests`]
#[derive(Clone, Debug)]
pub struct TriggerCITestsParams {
    /* Details of the test to trigger. */
    pub body: SyntheticsCITestBody,
}

// TriggerTestsParams is a struct for passing parameters to the method [`TriggerTests`]
#[derive(Clone, Debug)]
pub struct TriggerTestsParams {
    /* The identifiers of the tests to trigger. */
    pub body: SyntheticsTriggerBody,
}

// UpdateAPITestParams is a struct for passing parameters to the method [`UpdateAPITest`]
#[derive(Clone, Debug)]
pub struct UpdateAPITestParams {
    /* The public ID of the test to get details from. */
    pub public_id: String,
    /* New test details to be saved. */
    pub body: SyntheticsAPITest,
}

// UpdateBrowserTestParams is a struct for passing parameters to the method [`UpdateBrowserTest`]
#[derive(Clone, Debug)]
pub struct UpdateBrowserTestParams {
    /* The public ID of the test to get details from. */
    pub public_id: String,
    /* New test details to be saved. */
    pub body: SyntheticsBrowserTest,
}

// UpdatePrivateLocationParams is a struct for passing parameters to the method [`UpdatePrivateLocation`]
#[derive(Clone, Debug)]
pub struct UpdatePrivateLocationParams {
    /* The ID of the private location. */
    pub location_id: String,
    /* Details of the private location to be updated. */
    pub body: SyntheticsPrivateLocation,
}

// UpdateTestPauseStatusParams is a struct for passing parameters to the method [`UpdateTestPauseStatus`]
#[derive(Clone, Debug)]
pub struct UpdateTestPauseStatusParams {
    /* The public ID of the Synthetic test to update. */
    pub public_id: String,
    /* Status to set the given Synthetic test to. */
    pub body: SyntheticsUpdateTestPauseStatusPayload,
}




/// CreateGlobalVariableError is a struct for typed errors of method [`CreateGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGlobalVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreatePrivateLocationError is a struct for typed errors of method [`CreatePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePrivateLocationError {
    Status402(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsAPITestError is a struct for typed errors of method [`CreateSyntheticsAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsAPITestError {
    Status400(APIErrorResponse),
    Status402(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// CreateSyntheticsBrowserTestError is a struct for typed errors of method [`CreateSyntheticsBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSyntheticsBrowserTestError {
    Status400(APIErrorResponse),
    Status402(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteGlobalVariableError is a struct for typed errors of method [`DeleteGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGlobalVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeletePrivateLocationError is a struct for typed errors of method [`DeletePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePrivateLocationError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteTestsError is a struct for typed errors of method [`DeleteTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTestsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// EditGlobalVariableError is a struct for typed errors of method [`EditGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditGlobalVariableError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAPITestError is a struct for typed errors of method [`GetAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAPITestLatestResultsError is a struct for typed errors of method [`GetAPITestLatestResults`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestLatestResultsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAPITestResultError is a struct for typed errors of method [`GetAPITestResult`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAPITestResultError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestError is a struct for typed errors of method [`GetBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestLatestResultsError is a struct for typed errors of method [`GetBrowserTestLatestResults`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestLatestResultsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetBrowserTestResultError is a struct for typed errors of method [`GetBrowserTestResult`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBrowserTestResultError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetGlobalVariableError is a struct for typed errors of method [`GetGlobalVariable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGlobalVariableError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetPrivateLocationError is a struct for typed errors of method [`GetPrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrivateLocationError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsCIBatchError is a struct for typed errors of method [`GetSyntheticsCIBatch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsCIBatchError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSyntheticsDefaultLocationsError is a struct for typed errors of method [`GetSyntheticsDefaultLocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyntheticsDefaultLocationsError {
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetTestError is a struct for typed errors of method [`GetTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTestError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListGlobalVariablesError is a struct for typed errors of method [`ListGlobalVariables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGlobalVariablesError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListLocationsError is a struct for typed errors of method [`ListLocations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLocationsError {
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListTestsError is a struct for typed errors of method [`ListTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTestsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// TriggerCITestsError is a struct for typed errors of method [`TriggerCITests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerCITestsError {
    Status400(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// TriggerTestsError is a struct for typed errors of method [`TriggerTests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerTestsError {
    Status400(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAPITestError is a struct for typed errors of method [`UpdateAPITest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAPITestError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateBrowserTestError is a struct for typed errors of method [`UpdateBrowserTest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBrowserTestError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdatePrivateLocationError is a struct for typed errors of method [`UpdatePrivateLocation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePrivateLocationError {
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateTestPauseStatusError is a struct for typed errors of method [`UpdateTestPauseStatus`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTestPauseStatusError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
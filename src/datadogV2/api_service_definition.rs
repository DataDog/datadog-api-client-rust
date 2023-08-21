// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateOrUpdateServiceDefinitionsParams is a struct for passing parameters to the method [`CreateOrUpdateServiceDefinitions`]
#[derive(Clone, Debug)]
pub struct CreateOrUpdateServiceDefinitionsParams {
    /* Service Definition YAML/JSON. */
    pub body: ServiceDefinitionsCreateRequest,
}

// DeleteServiceDefinitionParams is a struct for passing parameters to the method [`DeleteServiceDefinition`]
#[derive(Clone, Debug)]
pub struct DeleteServiceDefinitionParams {
    /* The name of the service. */
    pub service_name: String,
}

// GetServiceDefinitionParams is a struct for passing parameters to the method [`GetServiceDefinition`]
#[derive(Clone, Debug)]
pub struct GetServiceDefinitionParams {
    /* The name of the service. */
    pub service_name: String,
    /* The schema version desired in the response. */
    pub schema_version: ServiceDefinitionSchemaVersions,
}

// ListServiceDefinitionsParams is a struct for passing parameters to the method [`ListServiceDefinitions`]
#[derive(Clone, Debug)]
pub struct ListServiceDefinitionsParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* The schema version desired in the response. */
    pub schema_version: ServiceDefinitionSchemaVersions,
}




/// CreateOrUpdateServiceDefinitionsError is a struct for typed errors of method [`CreateOrUpdateServiceDefinitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdateServiceDefinitionsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteServiceDefinitionError is a struct for typed errors of method [`DeleteServiceDefinition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceDefinitionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetServiceDefinitionError is a struct for typed errors of method [`GetServiceDefinition`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceDefinitionError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListServiceDefinitionsError is a struct for typed errors of method [`ListServiceDefinitions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListServiceDefinitionsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
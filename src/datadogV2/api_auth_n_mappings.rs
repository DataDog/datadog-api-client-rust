// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateAuthNMappingParams is a struct for passing parameters to the method [`CreateAuthNMapping`]
#[derive(Clone, Debug)]
pub struct CreateAuthNMappingParams {
    pub body: AuthNMappingCreateRequest,
}

// DeleteAuthNMappingParams is a struct for passing parameters to the method [`DeleteAuthNMapping`]
#[derive(Clone, Debug)]
pub struct DeleteAuthNMappingParams {
    /* The UUID of the AuthN Mapping. */
    pub authn_mapping_id: String,
}

// GetAuthNMappingParams is a struct for passing parameters to the method [`GetAuthNMapping`]
#[derive(Clone, Debug)]
pub struct GetAuthNMappingParams {
    /* The UUID of the AuthN Mapping. */
    pub authn_mapping_id: String,
}

// ListAuthNMappingsParams is a struct for passing parameters to the method [`ListAuthNMappings`]
#[derive(Clone, Debug)]
pub struct ListAuthNMappingsParams {
    /* Size for a given page. The maximum allowed value is 100. */
    pub page_size: i64,
    /* Specific page number to return. */
    pub page_number: i64,
    /* Sort AuthN Mappings depending on the given field. */
    pub sort: AuthNMappingsSort,
    /* Filter all mappings by the given string. */
    pub filter: String,
}

// UpdateAuthNMappingParams is a struct for passing parameters to the method [`UpdateAuthNMapping`]
#[derive(Clone, Debug)]
pub struct UpdateAuthNMappingParams {
    /* The UUID of the AuthN Mapping. */
    pub authn_mapping_id: String,
    pub body: AuthNMappingUpdateRequest,
}




/// CreateAuthNMappingError is a struct for typed errors of method [`CreateAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAuthNMappingError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteAuthNMappingError is a struct for typed errors of method [`DeleteAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAuthNMappingError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetAuthNMappingError is a struct for typed errors of method [`GetAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAuthNMappingError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListAuthNMappingsError is a struct for typed errors of method [`ListAuthNMappings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAuthNMappingsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateAuthNMappingError is a struct for typed errors of method [`UpdateAuthNMapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateAuthNMappingError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status422(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
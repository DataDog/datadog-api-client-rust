// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateHostTagsParams is a struct for passing parameters to the method [`CreateHostTags`]
#[derive(Clone, Debug)]
pub struct CreateHostTagsParams {
    /* This endpoint allows you to add new tags to a host, optionally specifying where the tags came from. */
    pub host_name: String,
    /* Update host tags request body. */
    pub body: HostTags,
    /* The source of the tags.
[Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value). */
    pub source: String,
}

// DeleteHostTagsParams is a struct for passing parameters to the method [`DeleteHostTags`]
#[derive(Clone, Debug)]
pub struct DeleteHostTagsParams {
    /* This endpoint allows you to remove all user-assigned tags for a single host. */
    pub host_name: String,
    /* The source of the tags (for example chef, puppet).
[Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value). */
    pub source: String,
}

// GetHostTagsParams is a struct for passing parameters to the method [`GetHostTags`]
#[derive(Clone, Debug)]
pub struct GetHostTagsParams {
    /* When specified, filters list of tags to those tags with the specified source. */
    pub host_name: String,
    /* Source to filter. */
    pub source: String,
}

// ListHostTagsParams is a struct for passing parameters to the method [`ListHostTags`]
#[derive(Clone, Debug)]
pub struct ListHostTagsParams {
    /* When specified, filters host list to those tags with the specified source. */
    pub source: String,
}

// UpdateHostTagsParams is a struct for passing parameters to the method [`UpdateHostTags`]
#[derive(Clone, Debug)]
pub struct UpdateHostTagsParams {
    /* This endpoint allows you to update/replace all in an integration source with those supplied in the request. */
    pub host_name: String,
    /* Add tags to host */
    pub body: HostTags,
    /* The source of the tags (for example chef, puppet).
[Complete list of source attribute values](https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value) */
    pub source: String,
}




/// CreateHostTagsError is a struct for typed errors of method [`CreateHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateHostTagsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteHostTagsError is a struct for typed errors of method [`DeleteHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteHostTagsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetHostTagsError is a struct for typed errors of method [`GetHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTagsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListHostTagsError is a struct for typed errors of method [`ListHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostTagsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateHostTagsError is a struct for typed errors of method [`UpdateHostTags`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateHostTagsError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
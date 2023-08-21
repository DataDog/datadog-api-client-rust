// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateChildOrgParams is a struct for passing parameters to the method [`CreateChildOrg`]
#[derive(Clone, Debug)]
pub struct CreateChildOrgParams {
    /* Organization object that needs to be created */
    pub body: OrganizationCreateBody,
}

// DowngradeOrgParams is a struct for passing parameters to the method [`DowngradeOrg`]
#[derive(Clone, Debug)]
pub struct DowngradeOrgParams {
    /* The `public_id` of the organization you are operating within. */
    pub public_id: String,
}

// GetOrgParams is a struct for passing parameters to the method [`GetOrg`]
#[derive(Clone, Debug)]
pub struct GetOrgParams {
    /* The `public_id` of the organization you are operating within. */
    pub public_id: String,
}

// UpdateOrgParams is a struct for passing parameters to the method [`UpdateOrg`]
#[derive(Clone, Debug)]
pub struct UpdateOrgParams {
    /* The `public_id` of the organization you are operating within. */
    pub public_id: String,
    pub body: Organization,
}

// UploadIdPForOrgParams is a struct for passing parameters to the method [`UploadIdPForOrg`]
#[derive(Clone, Debug)]
pub struct UploadIdPForOrgParams {
    /* The `public_id` of the organization you are operating with */
    pub public_id: String,
    /* The path to the XML metadata file you wish to upload. */
    pub idp_file: *os.File,
}




/// CreateChildOrgError is a struct for typed errors of method [`CreateChildOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChildOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DowngradeOrgError is a struct for typed errors of method [`DowngradeOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DowngradeOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetOrgError is a struct for typed errors of method [`GetOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListOrgsError is a struct for typed errors of method [`ListOrgs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOrgsError {
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateOrgError is a struct for typed errors of method [`UpdateOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UploadIdPForOrgError is a struct for typed errors of method [`UploadIdPForOrg`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadIdPForOrgError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status415(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
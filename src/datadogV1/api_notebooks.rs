// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateNotebookParams is a struct for passing parameters to the method [`CreateNotebook`]
#[derive(Clone, Debug)]
pub struct CreateNotebookParams {
    /* The JSON description of the notebook you want to create. */
    pub body: NotebookCreateRequest,
}

// DeleteNotebookParams is a struct for passing parameters to the method [`DeleteNotebook`]
#[derive(Clone, Debug)]
pub struct DeleteNotebookParams {
    /* Unique ID, assigned when you create the notebook. */
    pub notebook_id: i64,
}

// GetNotebookParams is a struct for passing parameters to the method [`GetNotebook`]
#[derive(Clone, Debug)]
pub struct GetNotebookParams {
    /* Unique ID, assigned when you create the notebook. */
    pub notebook_id: i64,
}

// ListNotebooksParams is a struct for passing parameters to the method [`ListNotebooks`]
#[derive(Clone, Debug)]
pub struct ListNotebooksParams {
    /* Return notebooks created by the given `author_handle`. */
    pub author_handle: String,
    /* Return notebooks not created by the given `author_handle`. */
    pub exclude_author_handle: String,
    /* The index of the first notebook you want returned. */
    pub start: i64,
    /* The number of notebooks to be returned. */
    pub count: i64,
    /* Sort by field `modified`, `name`, or `created`. */
    pub sort_field: String,
    /* Sort by direction `asc` or `desc`. */
    pub sort_dir: String,
    /* Return only notebooks with `query` string in notebook name or author handle. */
    pub query: String,
    /* Value of `false` excludes the `cells` and global `time` for each notebook. */
    pub include_cells: bool,
    /* True value returns only template notebooks. Default is false (returns only non-template notebooks). */
    pub is_template: bool,
    /* If type is provided, returns only notebooks with that metadata type. Default does not have type filtering. */
    pub type_: String,
}

// UpdateNotebookParams is a struct for passing parameters to the method [`UpdateNotebook`]
#[derive(Clone, Debug)]
pub struct UpdateNotebookParams {
    /* Unique ID, assigned when you create the notebook. */
    pub notebook_id: i64,
    /* Update notebook request body. */
    pub body: NotebookUpdateRequest,
}




/// CreateNotebookError is a struct for typed errors of method [`CreateNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNotebookError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteNotebookError is a struct for typed errors of method [`DeleteNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNotebookError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetNotebookError is a struct for typed errors of method [`GetNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotebookError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListNotebooksError is a struct for typed errors of method [`ListNotebooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNotebooksError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateNotebookError is a struct for typed errors of method [`UpdateNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNotebookError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status409(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateNotebookParams is a struct for passing parameters to the method [`CreateNotebook`]
#[derive(Clone, Debug)]
pub struct CreateNotebookParams {
    /// The JSON description of the notebook you want to create.
    pub body: crate::datadogV1::model::NotebookCreateRequest,
}

/// DeleteNotebookParams is a struct for passing parameters to the method [`DeleteNotebook`]
#[derive(Clone, Debug)]
pub struct DeleteNotebookParams {
    /// Unique ID, assigned when you create the notebook.
    pub notebook_id: i64,
}

/// GetNotebookParams is a struct for passing parameters to the method [`GetNotebook`]
#[derive(Clone, Debug)]
pub struct GetNotebookParams {
    /// Unique ID, assigned when you create the notebook.
    pub notebook_id: i64,
}

/// ListNotebooksParams is a struct for passing parameters to the method [`ListNotebooks`]
#[derive(Clone, Debug)]
pub struct ListNotebooksParams {
    /// Return notebooks created by the given `author_handle`.
    pub author_handle: Option<String>,
    /// Return notebooks not created by the given `author_handle`.
    pub exclude_author_handle: Option<String>,
    /// The index of the first notebook you want returned.
    pub start: Option<i64>,
    /// The number of notebooks to be returned.
    pub count: Option<i64>,
    /// Sort by field `modified`, `name`, or `created`.
    pub sort_field: Option<String>,
    /// Sort by direction `asc` or `desc`.
    pub sort_dir: Option<String>,
    /// Return only notebooks with `query` string in notebook name or author handle.
    pub query: Option<String>,
    /// Value of `false` excludes the `cells` and global `time` for each notebook.
    pub include_cells: Option<bool>,
    /// True value returns only template notebooks. Default is false (returns only non-template notebooks).
    pub is_template: Option<bool>,
    /// If type is provided, returns only notebooks with that metadata type. Default does not have type filtering.
    pub type_: Option<String>,
}

/// UpdateNotebookParams is a struct for passing parameters to the method [`UpdateNotebook`]
#[derive(Clone, Debug)]
pub struct UpdateNotebookParams {
    /// Unique ID, assigned when you create the notebook.
    pub notebook_id: i64,
    /// Update notebook request body.
    pub body: crate::datadogV1::model::NotebookUpdateRequest,
}

/// CreateNotebookError is a struct for typed errors of method [`CreateNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteNotebookError is a struct for typed errors of method [`DeleteNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetNotebookError is a struct for typed errors of method [`GetNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListNotebooksError is a struct for typed errors of method [`ListNotebooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNotebooksError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateNotebookError is a struct for typed errors of method [`UpdateNotebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status409(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct NotebooksAPI {
    config: configuration::Configuration,
}

impl Default for NotebooksAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl NotebooksAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a notebook using the specified options.
    pub async fn create_notebook(
        &self,
        params: CreateNotebookParams,
    ) -> Result<Option<crate::datadogV1::model::NotebookResponse>, Error<CreateNotebookError>> {
        match self.create_notebook_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a notebook using the specified options.
    pub async fn create_notebook_with_http_info(
        &self,
        params: CreateNotebookParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebookResponse>,
        Error<CreateNotebookError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/notebooks", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV1::model::NotebookResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateNotebookError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a notebook using the specified ID.
    pub async fn delete_notebook(
        &self,
        params: DeleteNotebookParams,
    ) -> Result<Option<()>, Error<DeleteNotebookError>> {
        match self.delete_notebook_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a notebook using the specified ID.
    pub async fn delete_notebook_with_http_info(
        &self,
        params: DeleteNotebookParams,
    ) -> Result<ResponseContent<()>, Error<DeleteNotebookError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let notebook_id = params.notebook_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.base_path,
            notebook_id = notebook_id
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
            let local_entity: Option<DeleteNotebookError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a notebook using the specified notebook ID.
    pub async fn get_notebook(
        &self,
        params: GetNotebookParams,
    ) -> Result<Option<crate::datadogV1::model::NotebookResponse>, Error<GetNotebookError>> {
        match self.get_notebook_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a notebook using the specified notebook ID.
    pub async fn get_notebook_with_http_info(
        &self,
        params: GetNotebookParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::NotebookResponse>, Error<GetNotebookError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let notebook_id = params.notebook_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.base_path,
            notebook_id = notebook_id
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
            let local_entity: Option<crate::datadogV1::model::NotebookResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetNotebookError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all notebooks. This can also be used to search for notebooks with a particular `query` in the notebook
    /// `name` or author `handle`.
    pub async fn list_notebooks(
        &self,
        params: ListNotebooksParams,
    ) -> Result<Option<crate::datadogV1::model::NotebooksResponse>, Error<ListNotebooksError>> {
        match self.list_notebooks_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all notebooks. This can also be used to search for notebooks with a particular `query` in the notebook
    /// `name` or author `handle`.
    pub async fn list_notebooks_with_http_info(
        &self,
        params: ListNotebooksParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebooksResponse>,
        Error<ListNotebooksError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let author_handle = params.author_handle;
        let exclude_author_handle = params.exclude_author_handle;
        let start = params.start;
        let count = params.count;
        let sort_field = params.sort_field;
        let sort_dir = params.sort_dir;
        let query = params.query;
        let include_cells = params.include_cells;
        let is_template = params.is_template;
        let type_ = params.type_;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/notebooks", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = author_handle {
            local_req_builder =
                local_req_builder.query(&[("author_handle", &local_str.to_string())]);
        };
        if let Some(ref local_str) = exclude_author_handle {
            local_req_builder =
                local_req_builder.query(&[("exclude_author_handle", &local_str.to_string())]);
        };
        if let Some(ref local_str) = start {
            local_req_builder = local_req_builder.query(&[("start", &local_str.to_string())]);
        };
        if let Some(ref local_str) = count {
            local_req_builder = local_req_builder.query(&[("count", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_field {
            local_req_builder = local_req_builder.query(&[("sort_field", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort_dir {
            local_req_builder = local_req_builder.query(&[("sort_dir", &local_str.to_string())]);
        };
        if let Some(ref local_str) = query {
            local_req_builder = local_req_builder.query(&[("query", &local_str.to_string())]);
        };
        if let Some(ref local_str) = include_cells {
            local_req_builder =
                local_req_builder.query(&[("include_cells", &local_str.to_string())]);
        };
        if let Some(ref local_str) = is_template {
            local_req_builder = local_req_builder.query(&[("is_template", &local_str.to_string())]);
        };
        if let Some(ref local_str) = type_ {
            local_req_builder = local_req_builder.query(&[("type", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::NotebooksResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListNotebooksError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a notebook using the specified ID.
    pub async fn update_notebook(
        &self,
        params: UpdateNotebookParams,
    ) -> Result<Option<crate::datadogV1::model::NotebookResponse>, Error<UpdateNotebookError>> {
        match self.update_notebook_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a notebook using the specified ID.
    pub async fn update_notebook_with_http_info(
        &self,
        params: UpdateNotebookParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebookResponse>,
        Error<UpdateNotebookError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let notebook_id = params.notebook_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.base_path,
            notebook_id = notebook_id
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
            let local_entity: Option<crate::datadogV1::model::NotebookResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateNotebookError> =
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
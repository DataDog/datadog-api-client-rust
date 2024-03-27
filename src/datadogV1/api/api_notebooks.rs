// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListNotebooksOptionalParams is a struct for passing parameters to the method [`NotebooksAPI::list_notebooks`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListNotebooksOptionalParams {
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

impl ListNotebooksOptionalParams {
    /// Return notebooks created by the given `author_handle`.
    pub fn author_handle(mut self, value: String) -> Self {
        self.author_handle = Some(value);
        self
    }
    /// Return notebooks not created by the given `author_handle`.
    pub fn exclude_author_handle(mut self, value: String) -> Self {
        self.exclude_author_handle = Some(value);
        self
    }
    /// The index of the first notebook you want returned.
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }
    /// The number of notebooks to be returned.
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }
    /// Sort by field `modified`, `name`, or `created`.
    pub fn sort_field(mut self, value: String) -> Self {
        self.sort_field = Some(value);
        self
    }
    /// Sort by direction `asc` or `desc`.
    pub fn sort_dir(mut self, value: String) -> Self {
        self.sort_dir = Some(value);
        self
    }
    /// Return only notebooks with `query` string in notebook name or author handle.
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    /// Value of `false` excludes the `cells` and global `time` for each notebook.
    pub fn include_cells(mut self, value: bool) -> Self {
        self.include_cells = Some(value);
        self
    }
    /// True value returns only template notebooks. Default is false (returns only non-template notebooks).
    pub fn is_template(mut self, value: bool) -> Self {
        self.is_template = Some(value);
        self
    }
    /// If type is provided, returns only notebooks with that metadata type. Default does not have type filtering.
    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

/// CreateNotebookError is a struct for typed errors of method [`NotebooksAPI::create_notebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteNotebookError is a struct for typed errors of method [`NotebooksAPI::delete_notebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetNotebookError is a struct for typed errors of method [`NotebooksAPI::get_notebook`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNotebookError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListNotebooksError is a struct for typed errors of method [`NotebooksAPI::list_notebooks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNotebooksError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateNotebookError is a struct for typed errors of method [`NotebooksAPI::update_notebook`]
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for NotebooksAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        }
    }
}

impl NotebooksAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
        let client = middleware_client_builder.build();
        Self { config, client }
    }

    pub fn with_client_and_config(
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a notebook using the specified options.
    pub async fn create_notebook(
        &self,
        body: crate::datadogV1::model::NotebookCreateRequest,
    ) -> Result<crate::datadogV1::model::NotebookResponse, Error<CreateNotebookError>> {
        match self.create_notebook_with_http_info(body).await {
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

    /// Create a notebook using the specified options.
    pub async fn create_notebook_with_http_info(
        &self,
        body: crate::datadogV1::model::NotebookCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebookResponse>,
        Error<CreateNotebookError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_notebook";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::NotebookResponse>(&local_content)
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
        notebook_id: i64,
    ) -> Result<(), Error<DeleteNotebookError>> {
        match self.delete_notebook_with_http_info(notebook_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a notebook using the specified ID.
    pub async fn delete_notebook_with_http_info(
        &self,
        notebook_id: i64,
    ) -> Result<ResponseContent<()>, Error<DeleteNotebookError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_notebook";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.get_operation_host(operation_id),
            notebook_id = notebook_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
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
        notebook_id: i64,
    ) -> Result<crate::datadogV1::model::NotebookResponse, Error<GetNotebookError>> {
        match self.get_notebook_with_http_info(notebook_id).await {
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

    /// Get a notebook using the specified notebook ID.
    pub async fn get_notebook_with_http_info(
        &self,
        notebook_id: i64,
    ) -> Result<ResponseContent<crate::datadogV1::model::NotebookResponse>, Error<GetNotebookError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.get_notebook";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.get_operation_host(operation_id),
            notebook_id = notebook_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::NotebookResponse>(&local_content)
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
        params: ListNotebooksOptionalParams,
    ) -> Result<crate::datadogV1::model::NotebooksResponse, Error<ListNotebooksError>> {
        match self.list_notebooks_with_http_info(params).await {
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

    pub fn list_notebooks_with_pagination(
        &self,
        mut params: ListNotebooksOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV1::model::NotebooksResponseData, Error<ListNotebooksError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.count.is_none() {
                params.count = Some(page_size);
            } else {
                page_size = params.count.unwrap().clone();
            }
            loop {
                let resp = self.list_notebooks(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.start.is_none() {
                    params.start = Some(page_size.clone());
                } else {
                    params.start = Some(params.start.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all notebooks. This can also be used to search for notebooks with a particular `query` in the notebook
    /// `name` or author `handle`.
    pub async fn list_notebooks_with_http_info(
        &self,
        params: ListNotebooksOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebooksResponse>,
        Error<ListNotebooksError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_notebooks";

        // unbox and build optional parameters
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

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = author_handle {
            local_req_builder =
                local_req_builder.query(&[("author_handle", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = exclude_author_handle {
            local_req_builder = local_req_builder
                .query(&[("exclude_author_handle", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = start {
            local_req_builder =
                local_req_builder.query(&[("start", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = count {
            local_req_builder =
                local_req_builder.query(&[("count", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_field {
            local_req_builder =
                local_req_builder.query(&[("sort_field", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_dir {
            local_req_builder =
                local_req_builder.query(&[("sort_dir", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = query {
            local_req_builder =
                local_req_builder.query(&[("query", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include_cells {
            local_req_builder =
                local_req_builder.query(&[("include_cells", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = is_template {
            local_req_builder =
                local_req_builder.query(&[("is_template", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = type_ {
            local_req_builder =
                local_req_builder.query(&[("type", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::NotebooksResponse>(&local_content)
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
        notebook_id: i64,
        body: crate::datadogV1::model::NotebookUpdateRequest,
    ) -> Result<crate::datadogV1::model::NotebookResponse, Error<UpdateNotebookError>> {
        match self.update_notebook_with_http_info(notebook_id, body).await {
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

    /// Update a notebook using the specified ID.
    pub async fn update_notebook_with_http_info(
        &self,
        notebook_id: i64,
        body: crate::datadogV1::model::NotebookUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::NotebookResponse>,
        Error<UpdateNotebookError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_notebook";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/notebooks/{notebook_id}",
            local_configuration.get_operation_host(operation_id),
            notebook_id = notebook_id
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.clone().as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                local_key
                    .key
                    .clone()
                    .parse()
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::NotebookResponse>(&local_content)
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

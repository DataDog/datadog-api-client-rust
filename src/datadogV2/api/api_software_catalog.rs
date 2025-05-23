// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListCatalogEntityOptionalParams is a struct for passing parameters to the method [`SoftwareCatalogAPI::list_catalog_entity`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCatalogEntityOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of entities in the response.
    pub page_limit: Option<i64>,
    /// Filter entities by UUID.
    pub filter_id: Option<String>,
    /// Filter entities by reference
    pub filter_ref: Option<String>,
    /// Filter entities by name.
    pub filter_name: Option<String>,
    /// Filter entities by kind.
    pub filter_kind: Option<String>,
    /// Filter entities by owner.
    pub filter_owner: Option<String>,
    /// Filter entities by relation type.
    pub filter_relation_type: Option<crate::datadogV2::model::RelationType>,
    /// Filter entities by excluding snapshotted entities.
    pub filter_exclude_snapshot: Option<String>,
    /// Include relationship data.
    pub include: Option<crate::datadogV2::model::IncludeType>,
}

impl ListCatalogEntityOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of entities in the response.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Filter entities by UUID.
    pub fn filter_id(mut self, value: String) -> Self {
        self.filter_id = Some(value);
        self
    }
    /// Filter entities by reference
    pub fn filter_ref(mut self, value: String) -> Self {
        self.filter_ref = Some(value);
        self
    }
    /// Filter entities by name.
    pub fn filter_name(mut self, value: String) -> Self {
        self.filter_name = Some(value);
        self
    }
    /// Filter entities by kind.
    pub fn filter_kind(mut self, value: String) -> Self {
        self.filter_kind = Some(value);
        self
    }
    /// Filter entities by owner.
    pub fn filter_owner(mut self, value: String) -> Self {
        self.filter_owner = Some(value);
        self
    }
    /// Filter entities by relation type.
    pub fn filter_relation_type(mut self, value: crate::datadogV2::model::RelationType) -> Self {
        self.filter_relation_type = Some(value);
        self
    }
    /// Filter entities by excluding snapshotted entities.
    pub fn filter_exclude_snapshot(mut self, value: String) -> Self {
        self.filter_exclude_snapshot = Some(value);
        self
    }
    /// Include relationship data.
    pub fn include(mut self, value: crate::datadogV2::model::IncludeType) -> Self {
        self.include = Some(value);
        self
    }
}

/// ListCatalogRelationOptionalParams is a struct for passing parameters to the method [`SoftwareCatalogAPI::list_catalog_relation`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListCatalogRelationOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// Maximum number of relations in the response.
    pub page_limit: Option<i64>,
    /// Filter relations by type.
    pub filter_type: Option<crate::datadogV2::model::RelationType>,
    /// Filter relations by the reference of the first entity in the relation.
    pub filter_from_ref: Option<String>,
    /// Filter relations by the reference of the second entity in the relation.
    pub filter_to_ref: Option<String>,
    /// Include relationship data.
    pub include: Option<crate::datadogV2::model::RelationIncludeType>,
}

impl ListCatalogRelationOptionalParams {
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(mut self, value: i64) -> Self {
        self.page_offset = Some(value);
        self
    }
    /// Maximum number of relations in the response.
    pub fn page_limit(mut self, value: i64) -> Self {
        self.page_limit = Some(value);
        self
    }
    /// Filter relations by type.
    pub fn filter_type(mut self, value: crate::datadogV2::model::RelationType) -> Self {
        self.filter_type = Some(value);
        self
    }
    /// Filter relations by the reference of the first entity in the relation.
    pub fn filter_from_ref(mut self, value: String) -> Self {
        self.filter_from_ref = Some(value);
        self
    }
    /// Filter relations by the reference of the second entity in the relation.
    pub fn filter_to_ref(mut self, value: String) -> Self {
        self.filter_to_ref = Some(value);
        self
    }
    /// Include relationship data.
    pub fn include(mut self, value: crate::datadogV2::model::RelationIncludeType) -> Self {
        self.include = Some(value);
        self
    }
}

/// DeleteCatalogEntityError is a struct for typed errors of method [`SoftwareCatalogAPI::delete_catalog_entity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCatalogEntityError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCatalogEntityError is a struct for typed errors of method [`SoftwareCatalogAPI::list_catalog_entity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCatalogEntityError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListCatalogRelationError is a struct for typed errors of method [`SoftwareCatalogAPI::list_catalog_relation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCatalogRelationError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpsertCatalogEntityError is a struct for typed errors of method [`SoftwareCatalogAPI::upsert_catalog_entity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpsertCatalogEntityError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// API to create, update, retrieve, and delete Software Catalog entities.
#[derive(Debug, Clone)]
pub struct SoftwareCatalogAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for SoftwareCatalogAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl SoftwareCatalogAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let mut reqwest_client_builder = reqwest::Client::builder();

        if let Some(proxy_url) = &config.proxy_url {
            let proxy = reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL");
            reqwest_client_builder = reqwest_client_builder.proxy(proxy);
        }

        let mut middleware_client_builder =
            reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());

        if config.enable_retry {
            struct RetryableStatus;
            impl reqwest_retry::RetryableStrategy for RetryableStatus {
                fn handle(
                    &self,
                    res: &Result<reqwest::Response, reqwest_middleware::Error>,
                ) -> Option<reqwest_retry::Retryable> {
                    match res {
                        Ok(success) => reqwest_retry::default_on_request_success(success),
                        Err(_) => None,
                    }
                }
            }
            let backoff_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                .build_with_max_retries(config.max_retries);

            let retry_middleware =
                reqwest_retry::RetryTransientMiddleware::new_with_policy_and_strategy(
                    backoff_policy,
                    RetryableStatus,
                );

            middleware_client_builder = middleware_client_builder.with(retry_middleware);
        }

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Delete a single entity in Software Catalog.
    pub async fn delete_catalog_entity(
        &self,
        entity_id: String,
    ) -> Result<(), datadog::Error<DeleteCatalogEntityError>> {
        match self.delete_catalog_entity_with_http_info(entity_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a single entity in Software Catalog.
    pub async fn delete_catalog_entity_with_http_info(
        &self,
        entity_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteCatalogEntityError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_catalog_entity";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/catalog/entity/{entity_id}",
            local_configuration.get_operation_host(operation_id),
            entity_id = datadog::urlencode(entity_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteCatalogEntityError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of entities from Software Catalog.
    pub async fn list_catalog_entity(
        &self,
        params: ListCatalogEntityOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListEntityCatalogResponse,
        datadog::Error<ListCatalogEntityError>,
    > {
        match self.list_catalog_entity_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_catalog_entity_with_pagination(
        &self,
        mut params: ListCatalogEntityOptionalParams,
    ) -> impl Stream<
        Item = Result<crate::datadogV2::model::EntityData, datadog::Error<ListCatalogEntityError>>,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_catalog_entity(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get a list of entities from Software Catalog.
    pub async fn list_catalog_entity_with_http_info(
        &self,
        params: ListCatalogEntityOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListEntityCatalogResponse>,
        datadog::Error<ListCatalogEntityError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_catalog_entity";

        // unbox and build optional parameters
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;
        let filter_id = params.filter_id;
        let filter_ref = params.filter_ref;
        let filter_name = params.filter_name;
        let filter_kind = params.filter_kind;
        let filter_owner = params.filter_owner;
        let filter_relation_type = params.filter_relation_type;
        let filter_exclude_snapshot = params.filter_exclude_snapshot;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/catalog/entity",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_id {
            local_req_builder =
                local_req_builder.query(&[("filter[id]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_ref {
            local_req_builder =
                local_req_builder.query(&[("filter[ref]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_name {
            local_req_builder =
                local_req_builder.query(&[("filter[name]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_kind {
            local_req_builder =
                local_req_builder.query(&[("filter[kind]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_owner {
            local_req_builder =
                local_req_builder.query(&[("filter[owner]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_relation_type {
            local_req_builder = local_req_builder
                .query(&[("filter[relation][type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_exclude_snapshot {
            local_req_builder = local_req_builder
                .query(&[("filter[exclude_snapshot]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListEntityCatalogResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListCatalogEntityError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get a list of entity relations from Software Catalog.
    pub async fn list_catalog_relation(
        &self,
        params: ListCatalogRelationOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ListRelationCatalogResponse,
        datadog::Error<ListCatalogRelationError>,
    > {
        match self.list_catalog_relation_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_catalog_relation_with_pagination(
        &self,
        mut params: ListCatalogRelationOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV2::model::RelationResponse,
            datadog::Error<ListCatalogRelationError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 100;
            if params.page_limit.is_none() {
                params.page_limit = Some(page_size);
            } else {
                page_size = params.page_limit.unwrap().clone();
            }
            loop {
                let resp = self.list_catalog_relation(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.page_offset.is_none() {
                    params.page_offset = Some(page_size.clone());
                } else {
                    params.page_offset = Some(params.page_offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get a list of entity relations from Software Catalog.
    pub async fn list_catalog_relation_with_http_info(
        &self,
        params: ListCatalogRelationOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ListRelationCatalogResponse>,
        datadog::Error<ListCatalogRelationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_catalog_relation";

        // unbox and build optional parameters
        let page_offset = params.page_offset;
        let page_limit = params.page_limit;
        let filter_type = params.filter_type;
        let filter_from_ref = params.filter_from_ref;
        let filter_to_ref = params.filter_to_ref;
        let include = params.include;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/catalog/relation",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_limit {
            local_req_builder =
                local_req_builder.query(&[("page[limit]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_type {
            local_req_builder =
                local_req_builder.query(&[("filter[type]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_from_ref {
            local_req_builder =
                local_req_builder.query(&[("filter[from_ref]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_to_ref {
            local_req_builder =
                local_req_builder.query(&[("filter[to_ref]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::ListRelationCatalogResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListCatalogRelationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Create or update entities in Software Catalog.
    pub async fn upsert_catalog_entity(
        &self,
        body: crate::datadogV2::model::UpsertCatalogEntityRequest,
    ) -> Result<
        crate::datadogV2::model::UpsertCatalogEntityResponse,
        datadog::Error<UpsertCatalogEntityError>,
    > {
        match self.upsert_catalog_entity_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create or update entities in Software Catalog.
    pub async fn upsert_catalog_entity_with_http_info(
        &self,
        body: crate::datadogV2::model::UpsertCatalogEntityRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::UpsertCatalogEntityResponse>,
        datadog::Error<UpsertCatalogEntityError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.upsert_catalog_entity";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/catalog/entity",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
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
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
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
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV2::model::UpsertCatalogEntityResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpsertCatalogEntityError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}

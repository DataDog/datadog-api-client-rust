// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateApmRetentionFilterError is a struct for typed errors of method [`APMRetentionFiltersAPI::create_apm_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApmRetentionFilterError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteApmRetentionFilterError is a struct for typed errors of method [`APMRetentionFiltersAPI::delete_apm_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApmRetentionFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetApmRetentionFilterError is a struct for typed errors of method [`APMRetentionFiltersAPI::get_apm_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApmRetentionFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListApmRetentionFiltersError is a struct for typed errors of method [`APMRetentionFiltersAPI::list_apm_retention_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApmRetentionFiltersError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ReorderApmRetentionFiltersError is a struct for typed errors of method [`APMRetentionFiltersAPI::reorder_apm_retention_filters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderApmRetentionFiltersError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateApmRetentionFilterError is a struct for typed errors of method [`APMRetentionFiltersAPI::update_apm_retention_filter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApmRetentionFilterError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct APMRetentionFiltersAPI {
    config: configuration::Configuration,
}

impl Default for APMRetentionFiltersAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl APMRetentionFiltersAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a retention filter to index spans in your organization.
    /// Returns the retention filter definition when the request is successful.
    pub async fn create_apm_retention_filter(
        &self,
        body: crate::datadogV2::model::RetentionFilterCreateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<CreateApmRetentionFilterError>,
    > {
        match self.create_apm_retention_filter_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a retention filter to index spans in your organization.
    /// Returns the retention filter definition when the request is successful.
    pub async fn create_apm_retention_filter_with_http_info(
        &self,
        body: crate::datadogV2::model::RetentionFilterCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<CreateApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_apm_retention_filter";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            let local_entity: Option<crate::datadogV2::model::RetentionFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateApmRetentionFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a specific retention filter from your organization.
    pub async fn delete_apm_retention_filter(
        &self,
        filter_id: String,
    ) -> Result<Option<()>, Error<DeleteApmRetentionFilterError>> {
        match self
            .delete_apm_retention_filter_with_http_info(filter_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific retention filter from your organization.
    pub async fn delete_apm_retention_filter_with_http_info(
        &self,
        filter_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteApmRetentionFilterError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_apm_retention_filter";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.get_operation_host(operation_id),
            filter_id = urlencode(filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            let local_entity: Option<DeleteApmRetentionFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an APM retention filter.
    pub async fn get_apm_retention_filter(
        &self,
        filter_id: String,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<GetApmRetentionFilterError>,
    > {
        match self
            .get_apm_retention_filter_with_http_info(filter_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an APM retention filter.
    pub async fn get_apm_retention_filter_with_http_info(
        &self,
        filter_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<GetApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_apm_retention_filter";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.get_operation_host(operation_id),
            filter_id = urlencode(filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::RetentionFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetApmRetentionFilterError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of APM retention filters.
    pub async fn list_apm_retention_filters(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFiltersResponse>,
        Error<ListApmRetentionFiltersError>,
    > {
        match self.list_apm_retention_filters_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of APM retention filters.
    pub async fn list_apm_retention_filters_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFiltersResponse>,
        Error<ListApmRetentionFiltersError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_apm_retention_filters";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
        };

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV2::model::RetentionFiltersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListApmRetentionFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Re-order the execution order of retention filters.
    pub async fn reorder_apm_retention_filters(
        &self,
        body: crate::datadogV2::model::ReorderRetentionFiltersRequest,
    ) -> Result<Option<()>, Error<ReorderApmRetentionFiltersError>> {
        match self
            .reorder_apm_retention_filters_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Re-order the execution order of retention filters.
    pub async fn reorder_apm_retention_filters_with_http_info(
        &self,
        body: crate::datadogV2::model::ReorderRetentionFiltersRequest,
    ) -> Result<ResponseContent<()>, Error<ReorderApmRetentionFiltersError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.reorder_apm_retention_filters";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters-execution-order",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<ReorderApmRetentionFiltersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a retention filter from your organization.
    pub async fn update_apm_retention_filter(
        &self,
        filter_id: String,
        body: crate::datadogV2::model::RetentionFilterUpdateRequest,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<UpdateApmRetentionFilterError>,
    > {
        match self
            .update_apm_retention_filter_with_http_info(filter_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a retention filter from your organization.
    pub async fn update_apm_retention_filter_with_http_info(
        &self,
        filter_id: String,
        body: crate::datadogV2::model::RetentionFilterUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<UpdateApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_apm_retention_filter";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.get_operation_host(operation_id),
            filter_id = urlencode(filter_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PUT, local_uri_str.as_str());

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            local_req_builder = local_req_builder.header("DD-API-KEY", &local_key.key);
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", &local_key.key);
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
            let local_entity: Option<crate::datadogV2::model::RetentionFilterResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateApmRetentionFilterError> =
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

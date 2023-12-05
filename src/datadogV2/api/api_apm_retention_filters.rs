// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateApmRetentionFilterParams is a struct for passing parameters to the method [`CreateApmRetentionFilter`]
#[derive(Clone, Debug, Default)]
pub struct CreateApmRetentionFilterParams {
    /// The definition of the new retention filter.
    pub body: crate::datadogV2::model::RetentionFilterCreateRequest,
}

/// DeleteApmRetentionFilterParams is a struct for passing parameters to the method [`DeleteApmRetentionFilter`]
#[derive(Clone, Debug, Default)]
pub struct DeleteApmRetentionFilterParams {
    /// The ID of the retention filter.
    pub filter_id: String,
}

/// GetApmRetentionFilterParams is a struct for passing parameters to the method [`GetApmRetentionFilter`]
#[derive(Clone, Debug, Default)]
pub struct GetApmRetentionFilterParams {
    /// The ID of the retention filter.
    pub filter_id: String,
}

/// ReorderApmRetentionFiltersParams is a struct for passing parameters to the method [`ReorderApmRetentionFilters`]
#[derive(Clone, Debug, Default)]
pub struct ReorderApmRetentionFiltersParams {
    /// The list of retention filters in the new order.
    pub body: crate::datadogV2::model::ReorderRetentionFiltersRequest,
}

/// UpdateApmRetentionFilterParams is a struct for passing parameters to the method [`UpdateApmRetentionFilter`]
#[derive(Clone, Debug, Default)]
pub struct UpdateApmRetentionFilterParams {
    /// The ID of the retention filter.
    pub filter_id: String,
    /// The updated definition of the retention filter.
    pub body: crate::datadogV2::model::RetentionFilterUpdateRequest,
}

/// CreateApmRetentionFilterError is a struct for typed errors of method [`CreateApmRetentionFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApmRetentionFilterError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteApmRetentionFilterError is a struct for typed errors of method [`DeleteApmRetentionFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApmRetentionFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetApmRetentionFilterError is a struct for typed errors of method [`GetApmRetentionFilter`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApmRetentionFilterError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListApmRetentionFiltersError is a struct for typed errors of method [`ListApmRetentionFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListApmRetentionFiltersError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ReorderApmRetentionFiltersError is a struct for typed errors of method [`ReorderApmRetentionFilters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReorderApmRetentionFiltersError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateApmRetentionFilterError is a struct for typed errors of method [`UpdateApmRetentionFilter`]
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
pub struct ApmRetentionFiltersAPI {
    config: configuration::Configuration,
}

impl Default for ApmRetentionFiltersAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ApmRetentionFiltersAPI {
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
        params: CreateApmRetentionFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<CreateApmRetentionFilterError>,
    > {
        match self
            .create_apm_retention_filter_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a retention filter to index spans in your organization.
    /// Returns the retention filter definition when the request is successful.
    pub async fn create_apm_retention_filter_with_http_info(
        &self,
        params: CreateApmRetentionFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<CreateApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters",
            local_configuration.base_path
        );
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
        params: DeleteApmRetentionFilterParams,
    ) -> Result<Option<()>, Error<DeleteApmRetentionFilterError>> {
        match self
            .delete_apm_retention_filter_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a specific retention filter from your organization.
    pub async fn delete_apm_retention_filter_with_http_info(
        &self,
        params: DeleteApmRetentionFilterParams,
    ) -> Result<ResponseContent<()>, Error<DeleteApmRetentionFilterError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_id = params.filter_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.base_path,
            filter_id = urlencode(filter_id)
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
        params: GetApmRetentionFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<GetApmRetentionFilterError>,
    > {
        match self.get_apm_retention_filter_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an APM retention filter.
    pub async fn get_apm_retention_filter_with_http_info(
        &self,
        params: GetApmRetentionFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<GetApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_id = params.filter_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.base_path,
            filter_id = urlencode(filter_id)
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

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters",
            local_configuration.base_path
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
        params: ReorderApmRetentionFiltersParams,
    ) -> Result<Option<()>, Error<ReorderApmRetentionFiltersError>> {
        match self
            .reorder_apm_retention_filters_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Re-order the execution order of retention filters.
    pub async fn reorder_apm_retention_filters_with_http_info(
        &self,
        params: ReorderApmRetentionFiltersParams,
    ) -> Result<ResponseContent<()>, Error<ReorderApmRetentionFiltersError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters-execution-order",
            local_configuration.base_path
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
        params: UpdateApmRetentionFilterParams,
    ) -> Result<
        Option<crate::datadogV2::model::RetentionFilterResponse>,
        Error<UpdateApmRetentionFilterError>,
    > {
        match self
            .update_apm_retention_filter_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a retention filter from your organization.
    pub async fn update_apm_retention_filter_with_http_info(
        &self,
        params: UpdateApmRetentionFilterParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RetentionFilterResponse>,
        Error<UpdateApmRetentionFilterError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let filter_id = params.filter_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/apm/config/retention-filters/{filter_id}",
            local_configuration.base_path,
            filter_id = urlencode(filter_id)
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
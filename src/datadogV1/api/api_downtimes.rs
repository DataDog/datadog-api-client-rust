// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CancelDowntimeParams is a struct for passing parameters to the method [`CancelDowntime`]
#[derive(Clone, Debug, Default)]
pub struct CancelDowntimeParams {
    /// ID of the downtime to cancel.
    pub downtime_id: i64,
}

/// CancelDowntimesByScopeParams is a struct for passing parameters to the method [`CancelDowntimesByScope`]
#[derive(Clone, Debug, Default)]
pub struct CancelDowntimesByScopeParams {
    /// Scope to cancel downtimes for.
    pub body: crate::datadogV1::model::CancelDowntimesByScopeRequest,
}

/// CreateDowntimeParams is a struct for passing parameters to the method [`CreateDowntime`]
#[derive(Clone, Debug, Default)]
pub struct CreateDowntimeParams {
    /// Schedule a downtime request body.
    pub body: crate::datadogV1::model::Downtime,
}

/// GetDowntimeParams is a struct for passing parameters to the method [`GetDowntime`]
#[derive(Clone, Debug, Default)]
pub struct GetDowntimeParams {
    /// ID of the downtime to fetch.
    pub downtime_id: i64,
}

/// ListDowntimesParams is a struct for passing parameters to the method [`ListDowntimes`]
#[derive(Clone, Debug, Default)]
pub struct ListDowntimesParams {
    /// Only return downtimes that are active when the request is made.
    pub current_only: Option<bool>,
    /// Return creator information.
    pub with_creator: Option<bool>,
}

/// ListMonitorDowntimesParams is a struct for passing parameters to the method [`ListMonitorDowntimes`]
#[derive(Clone, Debug, Default)]
pub struct ListMonitorDowntimesParams {
    /// The id of the monitor
    pub monitor_id: i64,
}

/// UpdateDowntimeParams is a struct for passing parameters to the method [`UpdateDowntime`]
#[derive(Clone, Debug, Default)]
pub struct UpdateDowntimeParams {
    /// ID of the downtime to update.
    pub downtime_id: i64,
    /// Update a downtime request body.
    pub body: crate::datadogV1::model::Downtime,
}

/// CancelDowntimeError is a struct for typed errors of method [`CancelDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimeError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CancelDowntimesByScopeError is a struct for typed errors of method [`CancelDowntimesByScope`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelDowntimesByScopeError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateDowntimeError is a struct for typed errors of method [`CreateDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateDowntimeError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetDowntimeError is a struct for typed errors of method [`GetDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDowntimeError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListDowntimesError is a struct for typed errors of method [`ListDowntimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListDowntimesError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListMonitorDowntimesError is a struct for typed errors of method [`ListMonitorDowntimes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListMonitorDowntimesError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateDowntimeError is a struct for typed errors of method [`UpdateDowntime`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateDowntimeError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct DowntimesAPI {
    config: configuration::Configuration,
}

impl Default for DowntimesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl DowntimesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime(
        &self,
        params: CancelDowntimeParams,
    ) -> Result<Option<()>, Error<CancelDowntimeError>> {
        match self.cancel_downtime_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Cancel a downtime.
    pub async fn cancel_downtime_with_http_info(
        &self,
        params: CancelDowntimeParams,
    ) -> Result<ResponseContent<()>, Error<CancelDowntimeError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let downtime_id = params.downtime_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/downtime/{downtime_id}",
            local_configuration.base_path,
            downtime_id = downtime_id
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
            let local_entity: Option<CancelDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete all downtimes that match the scope of `X`.
    pub async fn cancel_downtimes_by_scope(
        &self,
        params: CancelDowntimesByScopeParams,
    ) -> Result<
        Option<crate::datadogV1::model::CanceledDowntimesIds>,
        Error<CancelDowntimesByScopeError>,
    > {
        match self.cancel_downtimes_by_scope_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete all downtimes that match the scope of `X`.
    pub async fn cancel_downtimes_by_scope_with_http_info(
        &self,
        params: CancelDowntimesByScopeParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::CanceledDowntimesIds>,
        Error<CancelDowntimesByScopeError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/downtime/cancel/by_scope",
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::CanceledDowntimesIds> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CancelDowntimesByScopeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime(
        &self,
        params: CreateDowntimeParams,
    ) -> Result<Option<crate::datadogV1::model::Downtime>, Error<CreateDowntimeError>> {
        match self.create_downtime_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Schedule a downtime.
    pub async fn create_downtime_with_http_info(
        &self,
        params: CreateDowntimeParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Downtime>, Error<CreateDowntimeError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/downtime", local_configuration.base_path);
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::Downtime> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateDowntimeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime(
        &self,
        params: GetDowntimeParams,
    ) -> Result<Option<crate::datadogV1::model::Downtime>, Error<GetDowntimeError>> {
        match self.get_downtime_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get downtime detail by `downtime_id`.
    pub async fn get_downtime_with_http_info(
        &self,
        params: GetDowntimeParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Downtime>, Error<GetDowntimeError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let downtime_id = params.downtime_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/downtime/{downtime_id}",
            local_configuration.base_path,
            downtime_id = downtime_id
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
            let local_entity: Option<crate::datadogV1::model::Downtime> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetDowntimeError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes(
        &self,
        params: ListDowntimesParams,
    ) -> Result<Option<Vec<crate::datadogV1::model::Downtime>>, Error<ListDowntimesError>> {
        match self.list_downtimes_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all scheduled downtimes.
    pub async fn list_downtimes_with_http_info(
        &self,
        params: ListDowntimesParams,
    ) -> Result<ResponseContent<Vec<crate::datadogV1::model::Downtime>>, Error<ListDowntimesError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let current_only = params.current_only;
        let with_creator = params.with_creator;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/downtime", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = current_only {
            local_req_builder =
                local_req_builder.query(&[("current_only", &local_str.to_string())]);
        };
        if let Some(ref local_str) = with_creator {
            local_req_builder =
                local_req_builder.query(&[("with_creator", &local_str.to_string())]);
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
            let local_entity: Option<Vec<crate::datadogV1::model::Downtime>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes(
        &self,
        params: ListMonitorDowntimesParams,
    ) -> Result<Option<Vec<crate::datadogV1::model::Downtime>>, Error<ListMonitorDowntimesError>>
    {
        match self.list_monitor_downtimes_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all active downtimes for the specified monitor.
    pub async fn list_monitor_downtimes_with_http_info(
        &self,
        params: ListMonitorDowntimesParams,
    ) -> Result<
        ResponseContent<Vec<crate::datadogV1::model::Downtime>>,
        Error<ListMonitorDowntimesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let monitor_id = params.monitor_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/monitor/{monitor_id}/downtimes",
            local_configuration.base_path,
            monitor_id = monitor_id
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
            let local_entity: Option<Vec<crate::datadogV1::model::Downtime>> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListMonitorDowntimesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a single downtime by `downtime_id`.
    pub async fn update_downtime(
        &self,
        params: UpdateDowntimeParams,
    ) -> Result<Option<crate::datadogV1::model::Downtime>, Error<UpdateDowntimeError>> {
        match self.update_downtime_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a single downtime by `downtime_id`.
    pub async fn update_downtime_with_http_info(
        &self,
        params: UpdateDowntimeParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::Downtime>, Error<UpdateDowntimeError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let downtime_id = params.downtime_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/downtime/{downtime_id}",
            local_configuration.base_path,
            downtime_id = downtime_id
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
        local_req_builder = local_req_builder.json(&body);

        let local_req = local_req_builder.build()?;
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            let local_entity: Option<crate::datadogV1::model::Downtime> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateDowntimeError> =
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

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AddReadRoleToArchiveParams is a struct for passing parameters to the method [`AddReadRoleToArchive`]
#[derive(Clone, Debug)]
pub struct AddReadRoleToArchiveParams {
    /// The ID of the archive.
    pub archive_id: String,
    pub body: crate::datadogV2::model::RelationshipToRole,
}

/// CreateLogsArchiveParams is a struct for passing parameters to the method [`CreateLogsArchive`]
#[derive(Clone, Debug)]
pub struct CreateLogsArchiveParams {
    /// The definition of the new archive.
    pub body: crate::datadogV2::model::LogsArchiveCreateRequest,
}

/// DeleteLogsArchiveParams is a struct for passing parameters to the method [`DeleteLogsArchive`]
#[derive(Clone, Debug)]
pub struct DeleteLogsArchiveParams {
    /// The ID of the archive.
    pub archive_id: String,
}

/// GetLogsArchiveParams is a struct for passing parameters to the method [`GetLogsArchive`]
#[derive(Clone, Debug)]
pub struct GetLogsArchiveParams {
    /// The ID of the archive.
    pub archive_id: String,
}

/// ListArchiveReadRolesParams is a struct for passing parameters to the method [`ListArchiveReadRoles`]
#[derive(Clone, Debug)]
pub struct ListArchiveReadRolesParams {
    /// The ID of the archive.
    pub archive_id: String,
}

/// RemoveRoleFromArchiveParams is a struct for passing parameters to the method [`RemoveRoleFromArchive`]
#[derive(Clone, Debug)]
pub struct RemoveRoleFromArchiveParams {
    /// The ID of the archive.
    pub archive_id: String,
    pub body: crate::datadogV2::model::RelationshipToRole,
}

/// UpdateLogsArchiveParams is a struct for passing parameters to the method [`UpdateLogsArchive`]
#[derive(Clone, Debug)]
pub struct UpdateLogsArchiveParams {
    /// The ID of the archive.
    pub archive_id: String,
    /// New definition of the archive.
    pub body: crate::datadogV2::model::LogsArchiveCreateRequest,
}

/// UpdateLogsArchiveOrderParams is a struct for passing parameters to the method [`UpdateLogsArchiveOrder`]
#[derive(Clone, Debug)]
pub struct UpdateLogsArchiveOrderParams {
    /// An object containing the new ordered list of archive IDs.
    pub body: crate::datadogV2::model::LogsArchiveOrder,
}

/// AddReadRoleToArchiveError is a struct for typed errors of method [`AddReadRoleToArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddReadRoleToArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateLogsArchiveError is a struct for typed errors of method [`CreateLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsArchiveError is a struct for typed errors of method [`DeleteLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveError is a struct for typed errors of method [`GetLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveOrderError is a struct for typed errors of method [`GetLogsArchiveOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveOrderError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListArchiveReadRolesError is a struct for typed errors of method [`ListArchiveReadRoles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchiveReadRolesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsArchivesError is a struct for typed errors of method [`ListLogsArchives`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsArchivesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemoveRoleFromArchiveError is a struct for typed errors of method [`RemoveRoleFromArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRoleFromArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveError is a struct for typed errors of method [`UpdateLogsArchive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveOrderError is a struct for typed errors of method [`UpdateLogsArchiveOrder`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsArchiveOrderError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct LogsArchivesAPI {
    config: configuration::Configuration,
}

impl Default for LogsArchivesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl LogsArchivesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Adds a read role to an archive. ([Roles API](https://docs.datadoghq.com/api/v2/roles/))
    pub async fn add_read_role_to_archive(
        &self,
        params: AddReadRoleToArchiveParams,
    ) -> Result<Option<()>, Error<AddReadRoleToArchiveError>> {
        match self.add_read_role_to_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Adds a read role to an archive. ([Roles API](https://docs.datadoghq.com/api/v2/roles/))
    pub async fn add_read_role_to_archive_with_http_info(
        &self,
        params: AddReadRoleToArchiveParams,
    ) -> Result<ResponseContent<()>, Error<AddReadRoleToArchiveError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<AddReadRoleToArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create an archive in your organization.
    pub async fn create_logs_archive(
        &self,
        params: CreateLogsArchiveParams,
    ) -> Result<Option<crate::datadogV2::model::LogsArchive>, Error<CreateLogsArchiveError>> {
        match self.create_logs_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an archive in your organization.
    pub async fn create_logs_archive_with_http_info(
        &self,
        params: CreateLogsArchiveParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<CreateLogsArchiveError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives",
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
            let local_entity: Option<crate::datadogV2::model::LogsArchive> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateLogsArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Delete a given archive from your organization.
    pub async fn delete_logs_archive(
        &self,
        params: DeleteLogsArchiveParams,
    ) -> Result<Option<()>, Error<DeleteLogsArchiveError>> {
        match self.delete_logs_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Delete a given archive from your organization.
    pub async fn delete_logs_archive_with_http_info(
        &self,
        params: DeleteLogsArchiveParams,
    ) -> Result<ResponseContent<()>, Error<DeleteLogsArchiveError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            let local_entity: Option<DeleteLogsArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a specific archive from your organization.
    pub async fn get_logs_archive(
        &self,
        params: GetLogsArchiveParams,
    ) -> Result<Option<crate::datadogV2::model::LogsArchive>, Error<GetLogsArchiveError>> {
        match self.get_logs_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a specific archive from your organization.
    pub async fn get_logs_archive_with_http_info(
        &self,
        params: GetLogsArchiveParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<GetLogsArchiveError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            let local_entity: Option<crate::datadogV2::model::LogsArchive> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the current order of your archives.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_archive_order(
        &self,
    ) -> Result<Option<crate::datadogV2::model::LogsArchiveOrder>, Error<GetLogsArchiveOrderError>>
    {
        match self.get_logs_archive_order_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the current order of your archives.
    /// This endpoint takes no JSON arguments.
    pub async fn get_logs_archive_order_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsArchiveOrder>,
        Error<GetLogsArchiveOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archive-order",
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
            let local_entity: Option<crate::datadogV2::model::LogsArchiveOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetLogsArchiveOrderError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns all read roles a given archive is restricted to.
    pub async fn list_archive_read_roles(
        &self,
        params: ListArchiveReadRolesParams,
    ) -> Result<Option<crate::datadogV2::model::RolesResponse>, Error<ListArchiveReadRolesError>>
    {
        match self.list_archive_read_roles_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns all read roles a given archive is restricted to.
    pub async fn list_archive_read_roles_with_http_info(
        &self,
        params: ListArchiveReadRolesParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RolesResponse>,
        Error<ListArchiveReadRolesError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            let local_entity: Option<crate::datadogV2::model::RolesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListArchiveReadRolesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of configured logs archives with their definitions.
    pub async fn list_logs_archives(
        &self,
    ) -> Result<Option<crate::datadogV2::model::LogsArchives>, Error<ListLogsArchivesError>> {
        match self.list_logs_archives_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured logs archives with their definitions.
    pub async fn list_logs_archives_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchives>, Error<ListLogsArchivesError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives",
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
            let local_entity: Option<crate::datadogV2::model::LogsArchives> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListLogsArchivesError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Removes a role from an archive. ([Roles API](https://docs.datadoghq.com/api/v2/roles/))
    pub async fn remove_role_from_archive(
        &self,
        params: RemoveRoleFromArchiveParams,
    ) -> Result<Option<()>, Error<RemoveRoleFromArchiveError>> {
        match self.remove_role_from_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Removes a role from an archive. ([Roles API](https://docs.datadoghq.com/api/v2/roles/))
    pub async fn remove_role_from_archive_with_http_info(
        &self,
        params: RemoveRoleFromArchiveParams,
    ) -> Result<ResponseContent<()>, Error<RemoveRoleFromArchiveError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            let local_entity: Option<RemoveRoleFromArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a given archive configuration.
    ///
    /// **Note**: Using this method updates your archive configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_archive(
        &self,
        params: UpdateLogsArchiveParams,
    ) -> Result<Option<crate::datadogV2::model::LogsArchive>, Error<UpdateLogsArchiveError>> {
        match self.update_logs_archive_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update a given archive configuration.
    ///
    /// **Note**: Using this method updates your archive configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_archive_with_http_info(
        &self,
        params: UpdateLogsArchiveParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<UpdateLogsArchiveError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let archive_id = params.archive_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.base_path,
            archive_id = urlencode(archive_id)
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
            let local_entity: Option<crate::datadogV2::model::LogsArchive> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsArchiveError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the order of your archives. Since logs are processed sequentially, reordering an archive may change
    /// the structure and content of the data processed by other archives.
    ///
    /// **Note**: Using the `PUT` method updates your archive's order by replacing the current order
    /// with the new one.
    pub async fn update_logs_archive_order(
        &self,
        params: UpdateLogsArchiveOrderParams,
    ) -> Result<Option<crate::datadogV2::model::LogsArchiveOrder>, Error<UpdateLogsArchiveOrderError>>
    {
        match self.update_logs_archive_order_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the order of your archives. Since logs are processed sequentially, reordering an archive may change
    /// the structure and content of the data processed by other archives.
    ///
    /// **Note**: Using the `PUT` method updates your archive's order by replacing the current order
    /// with the new one.
    pub async fn update_logs_archive_order_with_http_info(
        &self,
        params: UpdateLogsArchiveOrderParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsArchiveOrder>,
        Error<UpdateLogsArchiveOrderError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archive-order",
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
            let local_entity: Option<crate::datadogV2::model::LogsArchiveOrder> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateLogsArchiveOrderError> =
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

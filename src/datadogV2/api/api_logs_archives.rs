// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AddReadRoleToArchiveError is a struct for typed errors of method [`LogsArchivesAPI::add_read_role_to_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddReadRoleToArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateLogsArchiveError is a struct for typed errors of method [`LogsArchivesAPI::create_logs_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteLogsArchiveError is a struct for typed errors of method [`LogsArchivesAPI::delete_logs_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveError is a struct for typed errors of method [`LogsArchivesAPI::get_logs_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetLogsArchiveOrderError is a struct for typed errors of method [`LogsArchivesAPI::get_logs_archive_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogsArchiveOrderError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListArchiveReadRolesError is a struct for typed errors of method [`LogsArchivesAPI::list_archive_read_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListArchiveReadRolesError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListLogsArchivesError is a struct for typed errors of method [`LogsArchivesAPI::list_logs_archives`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogsArchivesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemoveRoleFromArchiveError is a struct for typed errors of method [`LogsArchivesAPI::remove_role_from_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveRoleFromArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveError is a struct for typed errors of method [`LogsArchivesAPI::update_logs_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogsArchiveError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateLogsArchiveOrderError is a struct for typed errors of method [`LogsArchivesAPI::update_logs_archive_order`]
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

    /// Adds a read role to an archive. ([Roles API](<https://docs.datadoghq.com/api/v2/roles/>))
    pub async fn add_read_role_to_archive(
        &self,
        archive_id: String,
        body: crate::datadogV2::model::RelationshipToRole,
    ) -> Result<(), Error<AddReadRoleToArchiveError>> {
        match self
            .add_read_role_to_archive_with_http_info(archive_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Adds a read role to an archive. ([Roles API](<https://docs.datadoghq.com/api/v2/roles/>))
    pub async fn add_read_role_to_archive_with_http_info(
        &self,
        archive_id: String,
        body: crate::datadogV2::model::RelationshipToRole,
    ) -> Result<ResponseContent<()>, Error<AddReadRoleToArchiveError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.add_read_role_to_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
        body: crate::datadogV2::model::LogsArchiveCreateRequest,
    ) -> Result<crate::datadogV2::model::LogsArchive, Error<CreateLogsArchiveError>> {
        match self.create_logs_archive_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Create an archive in your organization.
    pub async fn create_logs_archive_with_http_info(
        &self,
        body: crate::datadogV2::model::LogsArchiveCreateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<CreateLogsArchiveError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.create_logs_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives",
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchive>(&local_content) {
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
        archive_id: String,
    ) -> Result<(), Error<DeleteLogsArchiveError>> {
        match self.delete_logs_archive_with_http_info(archive_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Delete a given archive from your organization.
    pub async fn delete_logs_archive_with_http_info(
        &self,
        archive_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteLogsArchiveError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_logs_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
        archive_id: String,
    ) -> Result<crate::datadogV2::model::LogsArchive, Error<GetLogsArchiveError>> {
        match self.get_logs_archive_with_http_info(archive_id).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get a specific archive from your organization.
    pub async fn get_logs_archive_with_http_info(
        &self,
        archive_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<GetLogsArchiveError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_logs_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchive>(&local_content) {
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
    ) -> Result<crate::datadogV2::model::LogsArchiveOrder, Error<GetLogsArchiveOrderError>> {
        match self.get_logs_archive_order_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
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
        let operation_id = "v2.get_logs_archive_order";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archive-order",
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchiveOrder>(&local_content)
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
        archive_id: String,
    ) -> Result<crate::datadogV2::model::RolesResponse, Error<ListArchiveReadRolesError>> {
        match self
            .list_archive_read_roles_with_http_info(archive_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Returns all read roles a given archive is restricted to.
    pub async fn list_archive_read_roles_with_http_info(
        &self,
        archive_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::RolesResponse>,
        Error<ListArchiveReadRolesError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_archive_read_roles";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
            match serde_json::from_str::<crate::datadogV2::model::RolesResponse>(&local_content) {
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
    ) -> Result<crate::datadogV2::model::LogsArchives, Error<ListLogsArchivesError>> {
        match self.list_logs_archives_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Get the list of configured logs archives with their definitions.
    pub async fn list_logs_archives_with_http_info(
        &self,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchives>, Error<ListLogsArchivesError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.list_logs_archives";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives",
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchives>(&local_content) {
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

    /// Removes a role from an archive. ([Roles API](<https://docs.datadoghq.com/api/v2/roles/>))
    pub async fn remove_role_from_archive(
        &self,
        archive_id: String,
        body: crate::datadogV2::model::RelationshipToRole,
    ) -> Result<(), Error<RemoveRoleFromArchiveError>> {
        match self
            .remove_role_from_archive_with_http_info(archive_id, body)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Removes a role from an archive. ([Roles API](<https://docs.datadoghq.com/api/v2/roles/>))
    pub async fn remove_role_from_archive_with_http_info(
        &self,
        archive_id: String,
        body: crate::datadogV2::model::RelationshipToRole,
    ) -> Result<ResponseContent<()>, Error<RemoveRoleFromArchiveError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.remove_role_from_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}/readers",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
        archive_id: String,
        body: crate::datadogV2::model::LogsArchiveCreateRequest,
    ) -> Result<crate::datadogV2::model::LogsArchive, Error<UpdateLogsArchiveError>> {
        match self
            .update_logs_archive_with_http_info(archive_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Update a given archive configuration.
    ///
    /// **Note**: Using this method updates your archive configuration by **replacing**
    /// your current configuration with the new one sent to your Datadog organization.
    pub async fn update_logs_archive_with_http_info(
        &self,
        archive_id: String,
        body: crate::datadogV2::model::LogsArchiveCreateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::LogsArchive>, Error<UpdateLogsArchiveError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.update_logs_archive";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archives/{archive_id}",
            local_configuration.get_operation_host(operation_id),
            archive_id = urlencode(archive_id)
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchive>(&local_content) {
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
        body: crate::datadogV2::model::LogsArchiveOrder,
    ) -> Result<crate::datadogV2::model::LogsArchiveOrder, Error<UpdateLogsArchiveOrderError>> {
        match self.update_logs_archive_order_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity.unwrap()),
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
        body: crate::datadogV2::model::LogsArchiveOrder,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::LogsArchiveOrder>,
        Error<UpdateLogsArchiveOrderError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_logs_archive_order";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/logs/config/archive-order",
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
            match serde_json::from_str::<crate::datadogV2::model::LogsArchiveOrder>(&local_content)
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

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AddPermissionToRoleParams is a struct for passing parameters to the method [`RolesAPI::add_permission_to_role`]
#[derive(Clone, Debug)]
pub struct AddPermissionToRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RelationshipToPermission,
}

/// AddUserToRoleParams is a struct for passing parameters to the method [`RolesAPI::add_user_to_role`]
#[derive(Clone, Debug)]
pub struct AddUserToRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RelationshipToUser,
}

/// CloneRoleParams is a struct for passing parameters to the method [`RolesAPI::clone_role`]
#[derive(Clone, Debug)]
pub struct CloneRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RoleCloneRequest,
}

/// CreateRoleParams is a struct for passing parameters to the method [`RolesAPI::create_role`]
#[derive(Clone, Debug)]
pub struct CreateRoleParams {
    pub body: crate::datadogV2::model::RoleCreateRequest,
}

/// DeleteRoleParams is a struct for passing parameters to the method [`RolesAPI::delete_role`]
#[derive(Clone, Debug)]
pub struct DeleteRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
}

/// GetRoleParams is a struct for passing parameters to the method [`RolesAPI::get_role`]
#[derive(Clone, Debug)]
pub struct GetRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
}

/// ListRolePermissionsParams is a struct for passing parameters to the method [`RolesAPI::list_role_permissions`]
#[derive(Clone, Debug)]
pub struct ListRolePermissionsParams {
    /// The unique identifier of the role.
    pub role_id: String,
}

/// ListRoleUsersParams is a struct for passing parameters to the method [`RolesAPI::list_role_users`]
#[derive(Clone, Debug)]
pub struct ListRoleUsersParams {
    /// The unique identifier of the role.
    pub role_id: String,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// User attribute to order results by. Sort order is **ascending** by default.
    /// Sort order is **descending** if the field is prefixed by a negative sign,
    /// for example `sort=-name`. Options: `name`, `email`, `status`.
    pub sort: Option<String>,
    /// Filter all users by the given string. Defaults to no filtering.
    pub filter: Option<String>,
}

/// ListRolesParams is a struct for passing parameters to the method [`RolesAPI::list_roles`]
#[derive(Clone, Debug)]
pub struct ListRolesParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Sort roles depending on the given field. Sort order is **ascending** by default.
    /// Sort order is **descending** if the field is prefixed by a negative sign, for example:
    /// `sort=-name`.
    pub sort: Option<crate::datadogV2::model::RolesSort>,
    /// Filter all roles by the given string.
    pub filter: Option<String>,
    /// Filter all roles by the given list of role IDs.
    pub filter_id: Option<String>,
}

/// RemovePermissionFromRoleParams is a struct for passing parameters to the method [`RolesAPI::remove_permission_from_role`]
#[derive(Clone, Debug)]
pub struct RemovePermissionFromRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RelationshipToPermission,
}

/// RemoveUserFromRoleParams is a struct for passing parameters to the method [`RolesAPI::remove_user_from_role`]
#[derive(Clone, Debug)]
pub struct RemoveUserFromRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RelationshipToUser,
}

/// UpdateRoleParams is a struct for passing parameters to the method [`RolesAPI::update_role`]
#[derive(Clone, Debug)]
pub struct UpdateRoleParams {
    /// The unique identifier of the role.
    pub role_id: String,
    pub body: crate::datadogV2::model::RoleUpdateRequest,
}

/// AddPermissionToRoleError is a struct for typed errors of method [`RolesAPI::add_permission_to_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPermissionToRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// AddUserToRoleError is a struct for typed errors of method [`RolesAPI::add_user_to_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddUserToRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CloneRoleError is a struct for typed errors of method [`RolesAPI::clone_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateRoleError is a struct for typed errors of method [`RolesAPI::create_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteRoleError is a struct for typed errors of method [`RolesAPI::delete_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRoleError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetRoleError is a struct for typed errors of method [`RolesAPI::get_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListPermissionsError is a struct for typed errors of method [`RolesAPI::list_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPermissionsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListRolePermissionsError is a struct for typed errors of method [`RolesAPI::list_role_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRolePermissionsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListRoleUsersError is a struct for typed errors of method [`RolesAPI::list_role_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRoleUsersError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListRolesError is a struct for typed errors of method [`RolesAPI::list_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListRolesError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemovePermissionFromRoleError is a struct for typed errors of method [`RolesAPI::remove_permission_from_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemovePermissionFromRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// RemoveUserFromRoleError is a struct for typed errors of method [`RolesAPI::remove_user_from_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveUserFromRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateRoleError is a struct for typed errors of method [`RolesAPI::update_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRoleError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct RolesAPI {
    config: configuration::Configuration,
}

impl Default for RolesAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl RolesAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Adds a permission to a role.
    pub async fn add_permission_to_role(
        &self,
        params: AddPermissionToRoleParams,
    ) -> Result<Option<crate::datadogV2::model::PermissionsResponse>, Error<AddPermissionToRoleError>>
    {
        match self.add_permission_to_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Adds a permission to a role.
    pub async fn add_permission_to_role_with_http_info(
        &self,
        params: AddPermissionToRoleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<AddPermissionToRoleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/permissions",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::PermissionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AddPermissionToRoleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Adds a user to a role.
    pub async fn add_user_to_role(
        &self,
        params: AddUserToRoleParams,
    ) -> Result<Option<crate::datadogV2::model::UsersResponse>, Error<AddUserToRoleError>> {
        match self.add_user_to_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Adds a user to a role.
    pub async fn add_user_to_role_with_http_info(
        &self,
        params: AddUserToRoleParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UsersResponse>, Error<AddUserToRoleError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/users",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::UsersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AddUserToRoleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Clone an existing role
    pub async fn clone_role(
        &self,
        params: CloneRoleParams,
    ) -> Result<Option<crate::datadogV2::model::RoleResponse>, Error<CloneRoleError>> {
        match self.clone_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Clone an existing role
    pub async fn clone_role_with_http_info(
        &self,
        params: CloneRoleParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::RoleResponse>, Error<CloneRoleError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/clone",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::RoleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CloneRoleError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Create a new role for your organization.
    pub async fn create_role(
        &self,
        params: CreateRoleParams,
    ) -> Result<Option<crate::datadogV2::model::RoleCreateResponse>, Error<CreateRoleError>> {
        match self.create_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a new role for your organization.
    pub async fn create_role_with_http_info(
        &self,
        params: CreateRoleParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::RoleCreateResponse>, Error<CreateRoleError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/roles", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::RoleCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateRoleError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Disables a role.
    pub async fn delete_role(
        &self,
        params: DeleteRoleParams,
    ) -> Result<Option<()>, Error<DeleteRoleError>> {
        match self.delete_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Disables a role.
    pub async fn delete_role_with_http_info(
        &self,
        params: DeleteRoleParams,
    ) -> Result<ResponseContent<()>, Error<DeleteRoleError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<DeleteRoleError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a role in the organization specified by the role’s `role_id`.
    pub async fn get_role(
        &self,
        params: GetRoleParams,
    ) -> Result<Option<crate::datadogV2::model::RoleResponse>, Error<GetRoleError>> {
        match self.get_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a role in the organization specified by the role’s `role_id`.
    pub async fn get_role_with_http_info(
        &self,
        params: GetRoleParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::RoleResponse>, Error<GetRoleError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::RoleResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetRoleError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns a list of all permissions, including name, description, and ID.
    pub async fn list_permissions(
        &self,
    ) -> Result<Option<crate::datadogV2::model::PermissionsResponse>, Error<ListPermissionsError>>
    {
        match self.list_permissions_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a list of all permissions, including name, description, and ID.
    pub async fn list_permissions_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<ListPermissionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/permissions", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::PermissionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListPermissionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns a list of all permissions for a single role.
    pub async fn list_role_permissions(
        &self,
        params: ListRolePermissionsParams,
    ) -> Result<Option<crate::datadogV2::model::PermissionsResponse>, Error<ListRolePermissionsError>>
    {
        match self.list_role_permissions_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a list of all permissions for a single role.
    pub async fn list_role_permissions_with_http_info(
        &self,
        params: ListRolePermissionsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<ListRolePermissionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/permissions",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::PermissionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListRolePermissionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Gets all users of a role.
    pub async fn list_role_users(
        &self,
        params: ListRoleUsersParams,
    ) -> Result<Option<crate::datadogV2::model::UsersResponse>, Error<ListRoleUsersError>> {
        match self.list_role_users_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Gets all users of a role.
    pub async fn list_role_users_with_http_info(
        &self,
        params: ListRoleUsersParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UsersResponse>, Error<ListRoleUsersError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/users",
            local_configuration.base_path,
            role_id = urlencode(role_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter {
            local_req_builder = local_req_builder.query(&[("filter", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::UsersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListRoleUsersError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns all roles, including their names and their unique identifiers.
    pub async fn list_roles(
        &self,
        params: ListRolesParams,
    ) -> Result<Option<crate::datadogV2::model::RolesResponse>, Error<ListRolesError>> {
        match self.list_roles_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns all roles, including their names and their unique identifiers.
    pub async fn list_roles_with_http_info(
        &self,
        params: ListRolesParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::RolesResponse>, Error<ListRolesError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter = params.filter;
        let filter_id = params.filter_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/roles", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = sort {
            local_req_builder = local_req_builder.query(&[("sort", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter {
            local_req_builder = local_req_builder.query(&[("filter", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_id {
            local_req_builder = local_req_builder.query(&[("filter[id]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::RolesResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListRolesError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Removes a permission from a role.
    pub async fn remove_permission_from_role(
        &self,
        params: RemovePermissionFromRoleParams,
    ) -> Result<
        Option<crate::datadogV2::model::PermissionsResponse>,
        Error<RemovePermissionFromRoleError>,
    > {
        match self
            .remove_permission_from_role_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Removes a permission from a role.
    pub async fn remove_permission_from_role_with_http_info(
        &self,
        params: RemovePermissionFromRoleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<RemovePermissionFromRoleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/permissions",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::PermissionsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<RemovePermissionFromRoleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Removes a user from a role.
    pub async fn remove_user_from_role(
        &self,
        params: RemoveUserFromRoleParams,
    ) -> Result<Option<crate::datadogV2::model::UsersResponse>, Error<RemoveUserFromRoleError>>
    {
        match self.remove_user_from_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Removes a user from a role.
    pub async fn remove_user_from_role_with_http_info(
        &self,
        params: RemoveUserFromRoleParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UsersResponse>,
        Error<RemoveUserFromRoleError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}/users",
            local_configuration.base_path,
            role_id = urlencode(role_id)
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
            let local_entity: Option<crate::datadogV2::model::UsersResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<RemoveUserFromRoleError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit a role. Can only be used with application keys belonging to administrators.
    pub async fn update_role(
        &self,
        params: UpdateRoleParams,
    ) -> Result<Option<crate::datadogV2::model::RoleUpdateResponse>, Error<UpdateRoleError>> {
        match self.update_role_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit a role. Can only be used with application keys belonging to administrators.
    pub async fn update_role_with_http_info(
        &self,
        params: UpdateRoleParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::RoleUpdateResponse>, Error<UpdateRoleError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let role_id = params.role_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/roles/{role_id}",
            local_configuration.base_path,
            role_id = urlencode(role_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            let local_entity: Option<crate::datadogV2::model::RoleUpdateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateRoleError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}

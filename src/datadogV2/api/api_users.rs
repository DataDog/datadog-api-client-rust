// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateUserParams is a struct for passing parameters to the method [`CreateUser`]
#[derive(Clone, Debug)]
pub struct CreateUserParams {
    pub body: crate::datadogV2::model::UserCreateRequest,
}

/// DisableUserParams is a struct for passing parameters to the method [`DisableUser`]
#[derive(Clone, Debug)]
pub struct DisableUserParams {
    /// The ID of the user.
    pub user_id: String,
}

/// GetInvitationParams is a struct for passing parameters to the method [`GetInvitation`]
#[derive(Clone, Debug)]
pub struct GetInvitationParams {
    /// The UUID of the user invitation.
    pub user_invitation_uuid: String,
}

/// GetUserParams is a struct for passing parameters to the method [`GetUser`]
#[derive(Clone, Debug)]
pub struct GetUserParams {
    /// The ID of the user.
    pub user_id: String,
}

/// ListUserOrganizationsParams is a struct for passing parameters to the method [`ListUserOrganizations`]
#[derive(Clone, Debug)]
pub struct ListUserOrganizationsParams {
    /// The ID of the user.
    pub user_id: String,
}

/// ListUserPermissionsParams is a struct for passing parameters to the method [`ListUserPermissions`]
#[derive(Clone, Debug)]
pub struct ListUserPermissionsParams {
    /// The ID of the user.
    pub user_id: String,
}

/// ListUsersParams is a struct for passing parameters to the method [`ListUsers`]
#[derive(Clone, Debug)]
pub struct ListUsersParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// User attribute to order results by. Sort order is ascending by default.
    /// Sort order is descending if the field
    /// is prefixed by a negative sign, for example `sort=-name`. Options: `name`,
    /// `modified_at`, `user_count`.
    pub sort: Option<String>,
    /// Direction of sort. Options: `asc`, `desc`.
    pub sort_dir: Option<crate::datadogV2::model::QuerySortOrder>,
    /// Filter all users by the given string. Defaults to no filtering.
    pub filter: Option<String>,
    /// Filter on status attribute.
    /// Comma separated list, with possible values `Active`, `Pending`, and `Disabled`.
    /// Defaults to no filtering.
    pub filter_status: Option<String>,
}

/// SendInvitationsParams is a struct for passing parameters to the method [`SendInvitations`]
#[derive(Clone, Debug)]
pub struct SendInvitationsParams {
    pub body: crate::datadogV2::model::UserInvitationsRequest,
}

/// UpdateUserParams is a struct for passing parameters to the method [`UpdateUser`]
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /// The ID of the user.
    pub user_id: String,
    pub body: crate::datadogV2::model::UserUpdateRequest,
}

/// CreateUserError is a struct for typed errors of method [`CreateUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DisableUserError is a struct for typed errors of method [`DisableUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableUserError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetInvitationError is a struct for typed errors of method [`GetInvitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvitationError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUserError is a struct for typed errors of method [`GetUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUserOrganizationsError is a struct for typed errors of method [`ListUserOrganizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserOrganizationsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUserPermissionsError is a struct for typed errors of method [`ListUserPermissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserPermissionsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUsersError is a struct for typed errors of method [`ListUsers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SendInvitationsError is a struct for typed errors of method [`SendInvitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInvitationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateUserError is a struct for typed errors of method [`UpdateUser`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct UsersAPI {
    config: configuration::Configuration,
}

impl Default for UsersAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl UsersAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a user for your organization.
    pub async fn create_user(
        &self,
        params: CreateUserParams,
    ) -> Result<Option<crate::datadogV2::model::UserResponse>, Error<CreateUserError>> {
        match self.create_user_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create a user for your organization.
    pub async fn create_user_with_http_info(
        &self,
        params: CreateUserParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<CreateUserError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/users", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::UserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateUserError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Disable a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn disable_user(
        &self,
        params: DisableUserParams,
    ) -> Result<Option<()>, Error<DisableUserError>> {
        match self.disable_user_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Disable a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn disable_user_with_http_info(
        &self,
        params: DisableUserParams,
    ) -> Result<ResponseContent<()>, Error<DisableUserError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_id = params.user_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.base_path,
            user_id = urlencode(user_id)
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
            let local_entity: Option<DisableUserError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Returns a single user invitation by its UUID.
    pub async fn get_invitation(
        &self,
        params: GetInvitationParams,
    ) -> Result<Option<crate::datadogV2::model::UserInvitationResponse>, Error<GetInvitationError>>
    {
        match self.get_invitation_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Returns a single user invitation by its UUID.
    pub async fn get_invitation_with_http_info(
        &self,
        params: GetInvitationParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserInvitationResponse>,
        Error<GetInvitationError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_invitation_uuid = params.user_invitation_uuid;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/user_invitations/{user_invitation_uuid}",
            local_configuration.base_path,
            user_invitation_uuid = urlencode(user_invitation_uuid)
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
            let local_entity: Option<crate::datadogV2::model::UserInvitationResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetInvitationError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a user in the organization specified by the user’s `user_id`.
    pub async fn get_user(
        &self,
        params: GetUserParams,
    ) -> Result<Option<crate::datadogV2::model::UserResponse>, Error<GetUserError>> {
        match self.get_user_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a user in the organization specified by the user’s `user_id`.
    pub async fn get_user_with_http_info(
        &self,
        params: GetUserParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<GetUserError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_id = params.user_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.base_path,
            user_id = urlencode(user_id)
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
            let local_entity: Option<crate::datadogV2::model::UserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetUserError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a user organization. Returns the user information and all organizations
    /// joined by this user.
    pub async fn list_user_organizations(
        &self,
        params: ListUserOrganizationsParams,
    ) -> Result<Option<crate::datadogV2::model::UserResponse>, Error<ListUserOrganizationsError>>
    {
        match self.list_user_organizations_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a user organization. Returns the user information and all organizations
    /// joined by this user.
    pub async fn list_user_organizations_with_http_info(
        &self,
        params: ListUserOrganizationsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserResponse>,
        Error<ListUserOrganizationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_id = params.user_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}/orgs",
            local_configuration.base_path,
            user_id = urlencode(user_id)
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
            let local_entity: Option<crate::datadogV2::model::UserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListUserOrganizationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a user permission set. Returns a list of the user’s permissions
    /// granted by the associated user's roles.
    pub async fn list_user_permissions(
        &self,
        params: ListUserPermissionsParams,
    ) -> Result<Option<crate::datadogV2::model::PermissionsResponse>, Error<ListUserPermissionsError>>
    {
        match self.list_user_permissions_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get a user permission set. Returns a list of the user’s permissions
    /// granted by the associated user's roles.
    pub async fn list_user_permissions_with_http_info(
        &self,
        params: ListUserPermissionsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<ListUserPermissionsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_id = params.user_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}/permissions",
            local_configuration.base_path,
            user_id = urlencode(user_id)
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
            let local_entity: Option<ListUserPermissionsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get the list of all users in the organization. This list includes
    /// all users even if they are deactivated or unverified.
    pub async fn list_users(
        &self,
        params: ListUsersParams,
    ) -> Result<Option<crate::datadogV2::model::UsersResponse>, Error<ListUsersError>> {
        match self.list_users_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the list of all users in the organization. This list includes
    /// all users even if they are deactivated or unverified.
    pub async fn list_users_with_http_info(
        &self,
        params: ListUsersParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UsersResponse>, Error<ListUsersError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let sort_dir = params.sort_dir;
        let filter = params.filter;
        let filter_status = params.filter_status;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/users", local_configuration.base_path);
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
        if let Some(ref local_str) = sort_dir {
            local_req_builder = local_req_builder.query(&[("sort_dir", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter {
            local_req_builder = local_req_builder.query(&[("filter", &local_str.to_string())]);
        };
        if let Some(ref local_str) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_str.to_string())]);
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
            let local_entity: Option<ListUsersError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Sends emails to one or more users inviting them to join the organization.
    pub async fn send_invitations(
        &self,
        params: SendInvitationsParams,
    ) -> Result<Option<crate::datadogV2::model::UserInvitationsResponse>, Error<SendInvitationsError>>
    {
        match self.send_invitations_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Sends emails to one or more users inviting them to join the organization.
    pub async fn send_invitations_with_http_info(
        &self,
        params: SendInvitationsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserInvitationsResponse>,
        Error<SendInvitationsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/user_invitations", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::UserInvitationsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SendInvitationsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Edit a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn update_user(
        &self,
        params: UpdateUserParams,
    ) -> Result<Option<crate::datadogV2::model::UserResponse>, Error<UpdateUserError>> {
        match self.update_user_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Edit a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn update_user_with_http_info(
        &self,
        params: UpdateUserParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<UpdateUserError>>
    {
        let local_configuration = &self.config;

        // unbox and build parameters
        let user_id = params.user_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.base_path,
            user_id = urlencode(user_id)
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
            let local_entity: Option<crate::datadogV2::model::UserResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateUserError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}

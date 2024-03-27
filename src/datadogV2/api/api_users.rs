// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use async_stream::try_stream;
use futures_core::stream::Stream;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListUsersOptionalParams is a struct for passing parameters to the method [`UsersAPI::list_users`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListUsersOptionalParams {
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

impl ListUsersOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(mut self, value: i64) -> Self {
        self.page_number = Some(value);
        self
    }
    /// User attribute to order results by. Sort order is ascending by default.
    /// Sort order is descending if the field
    /// is prefixed by a negative sign, for example `sort=-name`. Options: `name`,
    /// `modified_at`, `user_count`.
    pub fn sort(mut self, value: String) -> Self {
        self.sort = Some(value);
        self
    }
    /// Direction of sort. Options: `asc`, `desc`.
    pub fn sort_dir(mut self, value: crate::datadogV2::model::QuerySortOrder) -> Self {
        self.sort_dir = Some(value);
        self
    }
    /// Filter all users by the given string. Defaults to no filtering.
    pub fn filter(mut self, value: String) -> Self {
        self.filter = Some(value);
        self
    }
    /// Filter on status attribute.
    /// Comma separated list, with possible values `Active`, `Pending`, and `Disabled`.
    /// Defaults to no filtering.
    pub fn filter_status(mut self, value: String) -> Self {
        self.filter_status = Some(value);
        self
    }
}

/// CreateUserError is a struct for typed errors of method [`UsersAPI::create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DisableUserError is a struct for typed errors of method [`UsersAPI::disable_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableUserError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetInvitationError is a struct for typed errors of method [`UsersAPI::get_invitation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInvitationError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUserError is a struct for typed errors of method [`UsersAPI::get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUserOrganizationsError is a struct for typed errors of method [`UsersAPI::list_user_organizations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserOrganizationsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUserPermissionsError is a struct for typed errors of method [`UsersAPI::list_user_permissions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUserPermissionsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListUsersError is a struct for typed errors of method [`UsersAPI::list_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsersError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SendInvitationsError is a struct for typed errors of method [`UsersAPI::send_invitations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendInvitationsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateUserError is a struct for typed errors of method [`UsersAPI::update_user`]
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
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for UsersAPI {
    fn default() -> Self {
        Self::with_config(configuration::Configuration::default())
    }
}

impl UsersAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
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
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create a user for your organization.
    pub async fn create_user(
        &self,
        body: crate::datadogV2::model::UserCreateRequest,
    ) -> Result<crate::datadogV2::model::UserResponse, Error<CreateUserError>> {
        match self.create_user_with_http_info(body).await {
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

    /// Create a user for your organization.
    pub async fn create_user_with_http_info(
        &self,
        body: crate::datadogV2::model::UserCreateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<CreateUserError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.create_user";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users",
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
            match serde_json::from_str::<crate::datadogV2::model::UserResponse>(&local_content) {
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
    pub async fn disable_user(&self, user_id: String) -> Result<(), Error<DisableUserError>> {
        match self.disable_user_with_http_info(user_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Disable a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn disable_user_with_http_info(
        &self,
        user_id: String,
    ) -> Result<ResponseContent<()>, Error<DisableUserError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.disable_user";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.get_operation_host(operation_id),
            user_id = urlencode(user_id)
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
        user_invitation_uuid: String,
    ) -> Result<crate::datadogV2::model::UserInvitationResponse, Error<GetInvitationError>> {
        match self
            .get_invitation_with_http_info(user_invitation_uuid)
            .await
        {
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

    /// Returns a single user invitation by its UUID.
    pub async fn get_invitation_with_http_info(
        &self,
        user_invitation_uuid: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserInvitationResponse>,
        Error<GetInvitationError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_invitation";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_invitations/{user_invitation_uuid}",
            local_configuration.get_operation_host(operation_id),
            user_invitation_uuid = urlencode(user_invitation_uuid)
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
            match serde_json::from_str::<crate::datadogV2::model::UserInvitationResponse>(
                &local_content,
            ) {
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
        user_id: String,
    ) -> Result<crate::datadogV2::model::UserResponse, Error<GetUserError>> {
        match self.get_user_with_http_info(user_id).await {
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

    /// Get a user in the organization specified by the user’s `user_id`.
    pub async fn get_user_with_http_info(
        &self,
        user_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<GetUserError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.get_user";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.get_operation_host(operation_id),
            user_id = urlencode(user_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UserResponse>(&local_content) {
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
        user_id: String,
    ) -> Result<crate::datadogV2::model::UserResponse, Error<ListUserOrganizationsError>> {
        match self.list_user_organizations_with_http_info(user_id).await {
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

    /// Get a user organization. Returns the user information and all organizations
    /// joined by this user.
    pub async fn list_user_organizations_with_http_info(
        &self,
        user_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserResponse>,
        Error<ListUserOrganizationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_user_organizations";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}/orgs",
            local_configuration.get_operation_host(operation_id),
            user_id = urlencode(user_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UserResponse>(&local_content) {
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
        user_id: String,
    ) -> Result<crate::datadogV2::model::PermissionsResponse, Error<ListUserPermissionsError>> {
        match self.list_user_permissions_with_http_info(user_id).await {
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

    /// Get a user permission set. Returns a list of the user’s permissions
    /// granted by the associated user's roles.
    pub async fn list_user_permissions_with_http_info(
        &self,
        user_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::PermissionsResponse>,
        Error<ListUserPermissionsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_user_permissions";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}/permissions",
            local_configuration.get_operation_host(operation_id),
            user_id = urlencode(user_id)
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
            match serde_json::from_str::<crate::datadogV2::model::PermissionsResponse>(
                &local_content,
            ) {
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
        params: ListUsersOptionalParams,
    ) -> Result<crate::datadogV2::model::UsersResponse, Error<ListUsersError>> {
        match self.list_users_with_http_info(params).await {
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

    pub fn list_users_with_pagination(
        &self,
        mut params: ListUsersOptionalParams,
    ) -> impl Stream<Item = Result<crate::datadogV2::model::User, Error<ListUsersError>>> + '_ {
        try_stream! {
            let mut page_size: i64 = 10;
            if params.page_size.is_none() {
                params.page_size = Some(page_size);
            } else {
                page_size = params.page_size.unwrap().clone();
            }
            if params.page_number.is_none() {
                params.page_number = Some(0);
            }
            loop {
                let resp = self.list_users(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                params.page_number = Some(params.page_number.unwrap() + 1);
            }
        }
    }

    /// Get the list of all users in the organization. This list includes
    /// all users even if they are deactivated or unverified.
    pub async fn list_users_with_http_info(
        &self,
        params: ListUsersOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::UsersResponse>, Error<ListUsersError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.list_users";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let sort_dir = params.sort_dir;
        let filter = params.filter;
        let filter_status = params.filter_status;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort_dir {
            local_req_builder =
                local_req_builder.query(&[("sort_dir", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_status {
            local_req_builder =
                local_req_builder.query(&[("filter[status]", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::UsersResponse>(&local_content) {
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
        body: crate::datadogV2::model::UserInvitationsRequest,
    ) -> Result<crate::datadogV2::model::UserInvitationsResponse, Error<SendInvitationsError>> {
        match self.send_invitations_with_http_info(body).await {
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

    /// Sends emails to one or more users inviting them to join the organization.
    pub async fn send_invitations_with_http_info(
        &self,
        body: crate::datadogV2::model::UserInvitationsRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserInvitationsResponse>,
        Error<SendInvitationsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.send_invitations";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/user_invitations",
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
            match serde_json::from_str::<crate::datadogV2::model::UserInvitationsResponse>(
                &local_content,
            ) {
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
        user_id: String,
        body: crate::datadogV2::model::UserUpdateRequest,
    ) -> Result<crate::datadogV2::model::UserResponse, Error<UpdateUserError>> {
        match self.update_user_with_http_info(user_id, body).await {
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

    /// Edit a user. Can only be used with an application key belonging
    /// to an administrator user.
    pub async fn update_user_with_http_info(
        &self,
        user_id: String,
        body: crate::datadogV2::model::UserUpdateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::UserResponse>, Error<UpdateUserError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.update_user";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_id}",
            local_configuration.get_operation_host(operation_id),
            user_id = urlencode(user_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

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
            match serde_json::from_str::<crate::datadogV2::model::UserResponse>(&local_content) {
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

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetTeamMembershipsOptionalParams is a struct for passing parameters to the method [`TeamsAPI::get_team_memberships`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetTeamMembershipsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Specifies the order of returned team memberships
    pub sort: Option<crate::datadogV2::model::GetTeamMembershipsSort>,
    /// Search query, can be user email or name
    pub filter_keyword: Option<String>,
}

impl GetTeamMembershipsOptionalParams {
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// Specifies the order of returned team memberships
    pub fn sort(&mut self, value: crate::datadogV2::model::GetTeamMembershipsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Search query, can be user email or name
    pub fn filter_keyword(&mut self, value: String) -> &mut Self {
        self.filter_keyword = Some(value);
        self
    }
}

/// ListTeamsOptionalParams is a struct for passing parameters to the method [`TeamsAPI::list_teams`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListTeamsOptionalParams {
    /// Specific page number to return.
    pub page_number: Option<i64>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specifies the order of the returned teams
    pub sort: Option<crate::datadogV2::model::ListTeamsSort>,
    /// Included related resources optionally requested. Allowed enum values: `team_links, user_team_permissions`
    pub include: Option<Vec<crate::datadogV2::model::ListTeamsInclude>>,
    /// Search query. Can be team name, team handle, or email of team member
    pub filter_keyword: Option<String>,
    /// When true, only returns teams the current user belongs to
    pub filter_me: Option<bool>,
    /// List of fields that need to be fetched.
    pub fields_team: Option<Vec<crate::datadogV2::model::TeamsField>>,
}

impl ListTeamsOptionalParams {
    /// Specific page number to return.
    pub fn page_number(&mut self, value: i64) -> &mut Self {
        self.page_number = Some(value);
        self
    }
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specifies the order of the returned teams
    pub fn sort(&mut self, value: crate::datadogV2::model::ListTeamsSort) -> &mut Self {
        self.sort = Some(value);
        self
    }
    /// Included related resources optionally requested. Allowed enum values: `team_links, user_team_permissions`
    pub fn include(&mut self, value: Vec<crate::datadogV2::model::ListTeamsInclude>) -> &mut Self {
        self.include = Some(value);
        self
    }
    /// Search query. Can be team name, team handle, or email of team member
    pub fn filter_keyword(&mut self, value: String) -> &mut Self {
        self.filter_keyword = Some(value);
        self
    }
    /// When true, only returns teams the current user belongs to
    pub fn filter_me(&mut self, value: bool) -> &mut Self {
        self.filter_me = Some(value);
        self
    }
    /// List of fields that need to be fetched.
    pub fn fields_team(&mut self, value: Vec<crate::datadogV2::model::TeamsField>) -> &mut Self {
        self.fields_team = Some(value);
        self
    }
}

/// CreateTeamError is a struct for typed errors of method [`TeamsAPI::create_team`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateTeamLinkError is a struct for typed errors of method [`TeamsAPI::create_team_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamLinkError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status422(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// CreateTeamMembershipError is a struct for typed errors of method [`TeamsAPI::create_team_membership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTeamMembershipError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamError is a struct for typed errors of method [`TeamsAPI::delete_team`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamLinkError is a struct for typed errors of method [`TeamsAPI::delete_team_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamLinkError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteTeamMembershipError is a struct for typed errors of method [`TeamsAPI::delete_team_membership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTeamMembershipError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTeamError is a struct for typed errors of method [`TeamsAPI::get_team`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTeamLinkError is a struct for typed errors of method [`TeamsAPI::get_team_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamLinkError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTeamLinksError is a struct for typed errors of method [`TeamsAPI::get_team_links`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamLinksError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTeamMembershipsError is a struct for typed errors of method [`TeamsAPI::get_team_memberships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamMembershipsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetTeamPermissionSettingsError is a struct for typed errors of method [`TeamsAPI::get_team_permission_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTeamPermissionSettingsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetUserMembershipsError is a struct for typed errors of method [`TeamsAPI::get_user_memberships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserMembershipsError {
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListTeamsError is a struct for typed errors of method [`TeamsAPI::list_teams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTeamsError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamError is a struct for typed errors of method [`TeamsAPI::update_team`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status409(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamLinkError is a struct for typed errors of method [`TeamsAPI::update_team_link`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamLinkError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamMembershipError is a struct for typed errors of method [`TeamsAPI::update_team_membership`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamMembershipError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateTeamPermissionSettingError is a struct for typed errors of method [`TeamsAPI::update_team_permission_setting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTeamPermissionSettingError {
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct TeamsAPI {
    config: configuration::Configuration,
}

impl Default for TeamsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl TeamsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create a new team.
    /// User IDs passed through the `users` relationship field are added to the team.
    pub async fn create_team(
        &self,
        body: crate::datadogV2::model::TeamCreateRequest,
    ) -> Result<crate::datadogV2::model::TeamResponse, Error<CreateTeamError>> {
        match self.create_team_with_http_info(body).await {
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

    /// Create a new team.
    /// User IDs passed through the `users` relationship field are added to the team.
    pub async fn create_team_with_http_info(
        &self,
        body: crate::datadogV2::model::TeamCreateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamResponse>, Error<CreateTeamError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.create_team";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team",
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
            match serde_json::from_str::<crate::datadogV2::model::TeamResponse>(&local_content) {
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
            let local_entity: Option<CreateTeamError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Add a new link to a team.
    pub async fn create_team_link(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamLinkCreateRequest,
    ) -> Result<crate::datadogV2::model::TeamLinkResponse, Error<CreateTeamLinkError>> {
        match self.create_team_link_with_http_info(team_id, body).await {
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

    /// Add a new link to a team.
    pub async fn create_team_link_with_http_info(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamLinkCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::TeamLinkResponse>,
        Error<CreateTeamLinkError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_team_link";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/links",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamLinkResponse>(&local_content)
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
            let local_entity: Option<CreateTeamLinkError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Add a user to a team.
    pub async fn create_team_membership(
        &self,
        team_id: String,
        body: crate::datadogV2::model::UserTeamRequest,
    ) -> Result<crate::datadogV2::model::UserTeamResponse, Error<CreateTeamMembershipError>> {
        match self
            .create_team_membership_with_http_info(team_id, body)
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

    /// Add a user to a team.
    pub async fn create_team_membership_with_http_info(
        &self,
        team_id: String,
        body: crate::datadogV2::model::UserTeamRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserTeamResponse>,
        Error<CreateTeamMembershipError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_team_membership";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/memberships",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::UserTeamResponse>(&local_content)
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
            let local_entity: Option<CreateTeamMembershipError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Remove a team using the team's `id`.
    pub async fn delete_team(&self, team_id: String) -> Result<(), Error<DeleteTeamError>> {
        match self.delete_team_with_http_info(team_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove a team using the team's `id`.
    pub async fn delete_team_with_http_info(
        &self,
        team_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteTeamError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_team";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            let local_entity: Option<DeleteTeamError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Remove a link from a team.
    pub async fn delete_team_link(
        &self,
        team_id: String,
        link_id: String,
    ) -> Result<(), Error<DeleteTeamLinkError>> {
        match self.delete_team_link_with_http_info(team_id, link_id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove a link from a team.
    pub async fn delete_team_link_with_http_info(
        &self,
        team_id: String,
        link_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteTeamLinkError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_team_link";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/links/{link_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
            link_id = urlencode(link_id)
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
            let local_entity: Option<DeleteTeamLinkError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Remove a user from a team.
    pub async fn delete_team_membership(
        &self,
        team_id: String,
        user_id: String,
    ) -> Result<(), Error<DeleteTeamMembershipError>> {
        match self
            .delete_team_membership_with_http_info(team_id, user_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Remove a user from a team.
    pub async fn delete_team_membership_with_http_info(
        &self,
        team_id: String,
        user_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteTeamMembershipError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_team_membership";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/memberships/{user_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
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
            let local_entity: Option<DeleteTeamMembershipError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a single team using the team's `id`.
    pub async fn get_team(
        &self,
        team_id: String,
    ) -> Result<crate::datadogV2::model::TeamResponse, Error<GetTeamError>> {
        match self.get_team_with_http_info(team_id).await {
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

    /// Get a single team using the team's `id`.
    pub async fn get_team_with_http_info(
        &self,
        team_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamResponse>, Error<GetTeamError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.get_team";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamResponse>(&local_content) {
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
            let local_entity: Option<GetTeamError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a single link for a team.
    pub async fn get_team_link(
        &self,
        team_id: String,
        link_id: String,
    ) -> Result<crate::datadogV2::model::TeamLinkResponse, Error<GetTeamLinkError>> {
        match self.get_team_link_with_http_info(team_id, link_id).await {
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

    /// Get a single link for a team.
    pub async fn get_team_link_with_http_info(
        &self,
        team_id: String,
        link_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamLinkResponse>, Error<GetTeamLinkError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_team_link";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/links/{link_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
            link_id = urlencode(link_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamLinkResponse>(&local_content)
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
            let local_entity: Option<GetTeamLinkError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all links for a given team.
    pub async fn get_team_links(
        &self,
        team_id: String,
    ) -> Result<crate::datadogV2::model::TeamLinksResponse, Error<GetTeamLinksError>> {
        match self.get_team_links_with_http_info(team_id).await {
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

    /// Get all links for a given team.
    pub async fn get_team_links_with_http_info(
        &self,
        team_id: String,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamLinksResponse>, Error<GetTeamLinksError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.get_team_links";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/links",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamLinksResponse>(&local_content)
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
            let local_entity: Option<GetTeamLinksError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a paginated list of members for a team
    pub async fn get_team_memberships(
        &self,
        team_id: String,
        params: GetTeamMembershipsOptionalParams,
    ) -> Result<crate::datadogV2::model::UserTeamsResponse, Error<GetTeamMembershipsError>> {
        match self
            .get_team_memberships_with_http_info(team_id, params)
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

    /// Get a paginated list of members for a team
    pub async fn get_team_memberships_with_http_info(
        &self,
        team_id: String,
        params: GetTeamMembershipsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserTeamsResponse>,
        Error<GetTeamMembershipsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_team_memberships";

        // unbox and build optional parameters
        let page_size = params.page_size;
        let page_number = params.page_number;
        let sort = params.sort;
        let filter_keyword = params.filter_keyword;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/memberships",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
        if let Some(ref local_query_param) = filter_keyword {
            local_req_builder =
                local_req_builder.query(&[("filter[keyword]", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::UserTeamsResponse>(&local_content)
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
            let local_entity: Option<GetTeamMembershipsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all permission settings for a given team.
    pub async fn get_team_permission_settings(
        &self,
        team_id: String,
    ) -> Result<
        crate::datadogV2::model::TeamPermissionSettingsResponse,
        Error<GetTeamPermissionSettingsError>,
    > {
        match self
            .get_team_permission_settings_with_http_info(team_id)
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

    /// Get all permission settings for a given team.
    pub async fn get_team_permission_settings_with_http_info(
        &self,
        team_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::TeamPermissionSettingsResponse>,
        Error<GetTeamPermissionSettingsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_team_permission_settings";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/permission-settings",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamPermissionSettingsResponse>(
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
            let local_entity: Option<GetTeamPermissionSettingsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get a list of memberships for a user
    pub async fn get_user_memberships(
        &self,
        user_uuid: String,
    ) -> Result<crate::datadogV2::model::UserTeamsResponse, Error<GetUserMembershipsError>> {
        match self.get_user_memberships_with_http_info(user_uuid).await {
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

    /// Get a list of memberships for a user
    pub async fn get_user_memberships_with_http_info(
        &self,
        user_uuid: String,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserTeamsResponse>,
        Error<GetUserMembershipsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_user_memberships";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/users/{user_uuid}/memberships",
            local_configuration.get_operation_host(operation_id),
            user_uuid = urlencode(user_uuid)
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
            match serde_json::from_str::<crate::datadogV2::model::UserTeamsResponse>(&local_content)
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
            let local_entity: Option<GetUserMembershipsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all teams.
    /// Can be used to search for teams using the `filter[keyword]` and `filter[me]` query parameters.
    pub async fn list_teams(
        &self,
        params: ListTeamsOptionalParams,
    ) -> Result<crate::datadogV2::model::TeamsResponse, Error<ListTeamsError>> {
        match self.list_teams_with_http_info(params).await {
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

    /// Get all teams.
    /// Can be used to search for teams using the `filter[keyword]` and `filter[me]` query parameters.
    pub async fn list_teams_with_http_info(
        &self,
        params: ListTeamsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamsResponse>, Error<ListTeamsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.list_teams";

        // unbox and build optional parameters
        let page_number = params.page_number;
        let page_size = params.page_size;
        let sort = params.sort;
        let include = params.include;
        let filter_keyword = params.filter_keyword;
        let filter_me = params.filter_me;
        let fields_team = params.fields_team;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = page_number {
            local_req_builder =
                local_req_builder.query(&[("page[number]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sort {
            local_req_builder =
                local_req_builder.query(&[("sort", &local_query_param.to_string())]);
        };
        if let Some(ref local) = include {
            local_req_builder = local_req_builder.query(&[(
                "include",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        };
        if let Some(ref local_query_param) = filter_keyword {
            local_req_builder =
                local_req_builder.query(&[("filter[keyword]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter_me {
            local_req_builder =
                local_req_builder.query(&[("filter[me]", &local_query_param.to_string())]);
        };
        if let Some(ref local) = fields_team {
            local_req_builder = local_req_builder.query(&[(
                "fields[team]",
                &local
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
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
            match serde_json::from_str::<crate::datadogV2::model::TeamsResponse>(&local_content) {
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
            let local_entity: Option<ListTeamsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a team using the team's `id`.
    /// If the `team_links` relationship is present, the associated links are updated to be in the order they appear in the array, and any existing team links not present are removed.
    pub async fn update_team(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamUpdateRequest,
    ) -> Result<crate::datadogV2::model::TeamResponse, Error<UpdateTeamError>> {
        match self.update_team_with_http_info(team_id, body).await {
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

    /// Update a team using the team's `id`.
    /// If the `team_links` relationship is present, the associated links are updated to be in the order they appear in the array, and any existing team links not present are removed.
    pub async fn update_team_with_http_info(
        &self,
        team_id: String,
        body: crate::datadogV2::model::TeamUpdateRequest,
    ) -> Result<ResponseContent<crate::datadogV2::model::TeamResponse>, Error<UpdateTeamError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v2.update_team";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamResponse>(&local_content) {
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
            let local_entity: Option<UpdateTeamError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a team link.
    pub async fn update_team_link(
        &self,
        team_id: String,
        link_id: String,
        body: crate::datadogV2::model::TeamLinkCreateRequest,
    ) -> Result<crate::datadogV2::model::TeamLinkResponse, Error<UpdateTeamLinkError>> {
        match self
            .update_team_link_with_http_info(team_id, link_id, body)
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

    /// Update a team link.
    pub async fn update_team_link_with_http_info(
        &self,
        team_id: String,
        link_id: String,
        body: crate::datadogV2::model::TeamLinkCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::TeamLinkResponse>,
        Error<UpdateTeamLinkError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_team_link";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/links/{link_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
            link_id = urlencode(link_id)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamLinkResponse>(&local_content)
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
            let local_entity: Option<UpdateTeamLinkError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a user's membership attributes on a team.
    pub async fn update_team_membership(
        &self,
        team_id: String,
        user_id: String,
        body: crate::datadogV2::model::UserTeamUpdateRequest,
    ) -> Result<crate::datadogV2::model::UserTeamResponse, Error<UpdateTeamMembershipError>> {
        match self
            .update_team_membership_with_http_info(team_id, user_id, body)
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

    /// Update a user's membership attributes on a team.
    pub async fn update_team_membership_with_http_info(
        &self,
        team_id: String,
        user_id: String,
        body: crate::datadogV2::model::UserTeamUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::UserTeamResponse>,
        Error<UpdateTeamMembershipError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_team_membership";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/memberships/{user_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
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
            match serde_json::from_str::<crate::datadogV2::model::UserTeamResponse>(&local_content)
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
            let local_entity: Option<UpdateTeamMembershipError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update a team permission setting for a given team.
    pub async fn update_team_permission_setting(
        &self,
        team_id: String,
        action: String,
        body: crate::datadogV2::model::TeamPermissionSettingUpdateRequest,
    ) -> Result<
        crate::datadogV2::model::TeamPermissionSettingResponse,
        Error<UpdateTeamPermissionSettingError>,
    > {
        match self
            .update_team_permission_setting_with_http_info(team_id, action, body)
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

    /// Update a team permission setting for a given team.
    pub async fn update_team_permission_setting_with_http_info(
        &self,
        team_id: String,
        action: String,
        body: crate::datadogV2::model::TeamPermissionSettingUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::TeamPermissionSettingResponse>,
        Error<UpdateTeamPermissionSettingError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_team_permission_setting";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/team/{team_id}/permission-settings/{action}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id),
            action = urlencode(action)
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
            match serde_json::from_str::<crate::datadogV2::model::TeamPermissionSettingResponse>(
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
            let local_entity: Option<UpdateTeamPermissionSettingError> =
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

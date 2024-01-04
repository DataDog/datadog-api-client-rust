// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateIncidentTeamParams is a struct for passing parameters to the method [`CreateIncidentTeam`]
#[derive(Clone, Debug)]
pub struct CreateIncidentTeamParams {
    /// Incident Team Payload.
    pub body: crate::datadogV2::model::IncidentTeamCreateRequest,
}

/// DeleteIncidentTeamParams is a struct for passing parameters to the method [`DeleteIncidentTeam`]
#[derive(Clone, Debug)]
pub struct DeleteIncidentTeamParams {
    /// The ID of the incident team.
    pub team_id: String,
}

/// GetIncidentTeamParams is a struct for passing parameters to the method [`GetIncidentTeam`]
#[derive(Clone, Debug)]
pub struct GetIncidentTeamParams {
    /// The ID of the incident team.
    pub team_id: String,
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
}

/// ListIncidentTeamsParams is a struct for passing parameters to the method [`ListIncidentTeams`]
#[derive(Clone, Debug)]
pub struct ListIncidentTeamsParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// A search query that filters teams by name.
    pub filter: Option<String>,
}

/// UpdateIncidentTeamParams is a struct for passing parameters to the method [`UpdateIncidentTeam`]
#[derive(Clone, Debug)]
pub struct UpdateIncidentTeamParams {
    /// The ID of the incident team.
    pub team_id: String,
    /// Incident Team Payload.
    pub body: crate::datadogV2::model::IncidentTeamUpdateRequest,
}

/// CreateIncidentTeamError is a struct for typed errors of method [`CreateIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateIncidentTeamError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteIncidentTeamError is a struct for typed errors of method [`DeleteIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteIncidentTeamError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetIncidentTeamError is a struct for typed errors of method [`GetIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetIncidentTeamError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListIncidentTeamsError is a struct for typed errors of method [`ListIncidentTeams`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListIncidentTeamsError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateIncidentTeamError is a struct for typed errors of method [`UpdateIncidentTeam`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateIncidentTeamError {
    Status400(Option<crate::datadogV2::model::APIErrorResponse>),
    Status401(Option<crate::datadogV2::model::APIErrorResponse>),
    Status403(Option<crate::datadogV2::model::APIErrorResponse>),
    Status404(Option<crate::datadogV2::model::APIErrorResponse>),
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct IncidentTeamsAPI {
    config: configuration::Configuration,
}

impl Default for IncidentTeamsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl IncidentTeamsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Creates a new incident team.
    pub async fn create_incident_team(
        &self,
        params: CreateIncidentTeamParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<CreateIncidentTeamError>>
    {
        match self.create_incident_team_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates a new incident team.
    pub async fn create_incident_team_with_http_info(
        &self,
        params: CreateIncidentTeamParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<CreateIncidentTeamError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/teams", local_configuration.base_path);
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
            let local_entity: Option<crate::datadogV2::model::IncidentTeamResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateIncidentTeamError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Deletes an existing incident team.
    pub async fn delete_incident_team(
        &self,
        params: DeleteIncidentTeamParams,
    ) -> Result<Option<()>, Error<DeleteIncidentTeamError>> {
        match self.delete_incident_team_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing incident team.
    pub async fn delete_incident_team_with_http_info(
        &self,
        params: DeleteIncidentTeamParams,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentTeamError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let team_id = params.team_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
            local_configuration.base_path,
            team_id = urlencode(team_id)
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
            let local_entity: Option<DeleteIncidentTeamError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get details of an incident team. If the `include[users]` query parameter is provided,
    /// the included attribute will contain the users related to these incident teams.
    pub async fn get_incident_team(
        &self,
        params: GetIncidentTeamParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<GetIncidentTeamError>>
    {
        match self.get_incident_team_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get details of an incident team. If the `include[users]` query parameter is provided,
    /// the included attribute will contain the users related to these incident teams.
    pub async fn get_incident_team_with_http_info(
        &self,
        params: GetIncidentTeamParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<GetIncidentTeamError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let team_id = params.team_id;
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
            local_configuration.base_path,
            team_id = urlencode(team_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = include {
            local_req_builder = local_req_builder.query(&[("include", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentTeamResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetIncidentTeamError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all incident teams for the requesting user's organization. If the `include[users]` query parameter is provided, the included attribute will contain the users related to these incident teams.
    pub async fn list_incident_teams(
        &self,
        params: ListIncidentTeamsParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamsResponse>, Error<ListIncidentTeamsError>>
    {
        match self.list_incident_teams_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all incident teams for the requesting user's organization. If the `include[users]` query parameter is provided, the included attribute will contain the users related to these incident teams.
    pub async fn list_incident_teams_with_http_info(
        &self,
        params: ListIncidentTeamsParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamsResponse>,
        Error<ListIncidentTeamsError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let include = params.include;
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let filter = params.filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v2/teams", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = include {
            local_req_builder = local_req_builder.query(&[("include", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_size {
            local_req_builder = local_req_builder.query(&[("page[size]", &local_str.to_string())]);
        };
        if let Some(ref local_str) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_str.to_string())]);
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
            let local_entity: Option<crate::datadogV2::model::IncidentTeamsResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListIncidentTeamsError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Updates an existing incident team. Only provide the attributes which should be updated as this request is a partial update.
    pub async fn update_incident_team(
        &self,
        params: UpdateIncidentTeamParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<UpdateIncidentTeamError>>
    {
        match self.update_incident_team_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates an existing incident team. Only provide the attributes which should be updated as this request is a partial update.
    pub async fn update_incident_team_with_http_info(
        &self,
        params: UpdateIncidentTeamParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<UpdateIncidentTeamError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let team_id = params.team_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
            local_configuration.base_path,
            team_id = urlencode(team_id)
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
            let local_entity: Option<crate::datadogV2::model::IncidentTeamResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateIncidentTeamError> =
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

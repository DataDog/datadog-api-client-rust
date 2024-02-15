// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use log::warn;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetIncidentTeamOptionalParams is a struct for passing parameters to the method [`IncidentTeamsAPI::get_incident_team`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct GetIncidentTeamOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
}

impl GetIncidentTeamOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(&mut self, value: crate::datadogV2::model::IncidentRelatedObject) -> &mut Self {
        self.include = Some(value);
        self
    }
}

/// ListIncidentTeamsOptionalParams is a struct for passing parameters to the method [`IncidentTeamsAPI::list_incident_teams`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListIncidentTeamsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub include: Option<crate::datadogV2::model::IncidentRelatedObject>,
    /// Size for a given page. The maximum allowed value is 100.
    pub page_size: Option<i64>,
    /// Specific offset to use as the beginning of the returned page.
    pub page_offset: Option<i64>,
    /// A search query that filters teams by name.
    pub filter: Option<String>,
}

impl ListIncidentTeamsOptionalParams {
    /// Specifies which types of related objects should be included in the response.
    pub fn include(&mut self, value: crate::datadogV2::model::IncidentRelatedObject) -> &mut Self {
        self.include = Some(value);
        self
    }
    /// Size for a given page. The maximum allowed value is 100.
    pub fn page_size(&mut self, value: i64) -> &mut Self {
        self.page_size = Some(value);
        self
    }
    /// Specific offset to use as the beginning of the returned page.
    pub fn page_offset(&mut self, value: i64) -> &mut Self {
        self.page_offset = Some(value);
        self
    }
    /// A search query that filters teams by name.
    pub fn filter(&mut self, value: String) -> &mut Self {
        self.filter = Some(value);
        self
    }
}

/// CreateIncidentTeamError is a struct for typed errors of method [`IncidentTeamsAPI::create_incident_team`]
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

/// DeleteIncidentTeamError is a struct for typed errors of method [`IncidentTeamsAPI::delete_incident_team`]
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

/// GetIncidentTeamError is a struct for typed errors of method [`IncidentTeamsAPI::get_incident_team`]
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

/// ListIncidentTeamsError is a struct for typed errors of method [`IncidentTeamsAPI::list_incident_teams`]
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

/// UpdateIncidentTeamError is a struct for typed errors of method [`IncidentTeamsAPI::update_incident_team`]
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
        body: crate::datadogV2::model::IncidentTeamCreateRequest,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<CreateIncidentTeamError>>
    {
        match self.create_incident_team_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Creates a new incident team.
    pub async fn create_incident_team_with_http_info(
        &self,
        body: crate::datadogV2::model::IncidentTeamCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<CreateIncidentTeamError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.create_incident_team";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.create_incident_team' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams",
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
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
        team_id: String,
    ) -> Result<Option<()>, Error<DeleteIncidentTeamError>> {
        match self.delete_incident_team_with_http_info(team_id).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing incident team.
    pub async fn delete_incident_team_with_http_info(
        &self,
        team_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteIncidentTeamError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_incident_team";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.delete_incident_team' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
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
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
        team_id: String,
        params: GetIncidentTeamOptionalParams,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<GetIncidentTeamError>>
    {
        match self.get_incident_team_with_http_info(team_id, params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get details of an incident team. If the `include[users]` query parameter is provided,
    /// the included attribute will contain the users related to these incident teams.
    pub async fn get_incident_team_with_http_info(
        &self,
        team_id: String,
        params: GetIncidentTeamOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<GetIncidentTeamError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_incident_team";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.get_incident_team' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
            local_configuration.get_operation_host(operation_id),
            team_id = urlencode(team_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
        params: ListIncidentTeamsOptionalParams,
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
        params: ListIncidentTeamsOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamsResponse>,
        Error<ListIncidentTeamsError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.list_incident_teams";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.list_incident_teams' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let include = params.include;
        let page_size = params.page_size;
        let page_offset = params.page_offset;
        let filter = params.filter;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = include {
            local_req_builder =
                local_req_builder.query(&[("include", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_size {
            local_req_builder =
                local_req_builder.query(&[("page[size]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page_offset {
            local_req_builder =
                local_req_builder.query(&[("page[offset]", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = filter {
            local_req_builder =
                local_req_builder.query(&[("filter", &local_query_param.to_string())]);
        };

        // build user agent
        local_req_builder = local_req_builder.header(
            reqwest::header::USER_AGENT,
            local_configuration.user_agent.clone(),
        );

        // build auth
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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
        team_id: String,
        body: crate::datadogV2::model::IncidentTeamUpdateRequest,
    ) -> Result<Option<crate::datadogV2::model::IncidentTeamResponse>, Error<UpdateIncidentTeamError>>
    {
        match self
            .update_incident_team_with_http_info(team_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Updates an existing incident team. Only provide the attributes which should be updated as this request is a partial update.
    pub async fn update_incident_team_with_http_info(
        &self,
        team_id: String,
        body: crate::datadogV2::model::IncidentTeamUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::IncidentTeamResponse>,
        Error<UpdateIncidentTeamError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_incident_team";
        if local_configuration.is_unstable_operation_enabled(operation_id) {
            warn!("Using unstable operation {}", operation_id);
        } else {
            let local_error = UnstableOperationDisabledError {
                msg: "Operation 'v2.update_incident_team' is not enabled".to_string(),
            };
            return Err(Error::UnstableOperationDisabledError(local_error));
        }

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/teams/{team_id}",
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
        if let Some(ref local_key) = local_configuration.api_key {
            local_req_builder = local_req_builder.header("DD-API-KEY", local_key);
        };
        if let Some(ref local_key) = local_configuration.app_key {
            local_req_builder = local_req_builder.header("DD-APPLICATION-KEY", local_key);
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

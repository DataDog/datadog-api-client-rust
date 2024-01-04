// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AddSecurityMonitoringSignalToIncidentParams is a struct for passing parameters to the method [`AddSecurityMonitoringSignalToIncident`]
#[derive(Clone, Debug)]
pub struct AddSecurityMonitoringSignalToIncidentParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV1::model::AddSignalToIncidentRequest,
}

/// EditSecurityMonitoringSignalAssigneeParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalAssigneeParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV1::model::SignalAssigneeUpdateRequest,
}

/// EditSecurityMonitoringSignalStateParams is a struct for passing parameters to the method [`EditSecurityMonitoringSignalState`]
#[derive(Clone, Debug)]
pub struct EditSecurityMonitoringSignalStateParams {
    /// The ID of the signal.
    pub signal_id: String,
    /// Attributes describing the signal update.
    pub body: crate::datadogV1::model::SignalStateUpdateRequest,
}

/// AddSecurityMonitoringSignalToIncidentError is a struct for typed errors of method [`AddSecurityMonitoringSignalToIncident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSecurityMonitoringSignalToIncidentError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`EditSecurityMonitoringSignalAssignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`EditSecurityMonitoringSignalState`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalStateError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SecurityMonitoringAPI {
    config: configuration::Configuration,
}

impl Default for SecurityMonitoringAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SecurityMonitoringAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Add a security signal to an incident. This makes it possible to search for signals by incident within the signal explorer and to view the signals on the incident timeline.
    pub async fn add_security_monitoring_signal_to_incident(
        &self,
        params: AddSecurityMonitoringSignalToIncidentParams,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<AddSecurityMonitoringSignalToIncidentError>,
    > {
        match self
            .add_security_monitoring_signal_to_incident_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Add a security signal to an incident. This makes it possible to search for signals by incident within the signal explorer and to view the signals on the incident timeline.
    pub async fn add_security_monitoring_signal_to_incident_with_http_info(
        &self,
        params: AddSecurityMonitoringSignalToIncidentParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<AddSecurityMonitoringSignalToIncidentError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/add_to_incident",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
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
            let local_entity: Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<AddSecurityMonitoringSignalToIncidentError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee(
        &self,
        params: EditSecurityMonitoringSignalAssigneeParams,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        match self
            .edit_security_monitoring_signal_assignee_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee_with_http_info(
        &self,
        params: EditSecurityMonitoringSignalAssigneeParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/assignee",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
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
            let local_entity: Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalAssigneeError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state(
        &self,
        params: EditSecurityMonitoringSignalStateParams,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        match self
            .edit_security_monitoring_signal_state_with_http_info(params)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state_with_http_info(
        &self,
        params: EditSecurityMonitoringSignalStateParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let signal_id = params.signal_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/state",
            local_configuration.base_path,
            signal_id = urlencode(signal_id)
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
            let local_entity: Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<EditSecurityMonitoringSignalStateError> =
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

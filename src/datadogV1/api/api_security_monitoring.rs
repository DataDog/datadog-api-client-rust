// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// AddSecurityMonitoringSignalToIncidentError is a struct for typed errors of method [`SecurityMonitoringAPI::add_security_monitoring_signal_to_incident`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSecurityMonitoringSignalToIncidentError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalAssigneeError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_assignee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EditSecurityMonitoringSignalAssigneeError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// EditSecurityMonitoringSignalStateError is a struct for typed errors of method [`SecurityMonitoringAPI::edit_security_monitoring_signal_state`]
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
        signal_id: String,
        body: crate::datadogV1::model::AddSignalToIncidentRequest,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<AddSecurityMonitoringSignalToIncidentError>,
    > {
        match self
            .add_security_monitoring_signal_to_incident_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Add a security signal to an incident. This makes it possible to search for signals by incident within the signal explorer and to view the signals on the incident timeline.
    pub async fn add_security_monitoring_signal_to_incident_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV1::model::AddSignalToIncidentRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<AddSecurityMonitoringSignalToIncidentError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.add_security_monitoring_signal_to_incident";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/add_to_incident",
            local_configuration.get_operation_host(operation_id),
            signal_id = urlencode(signal_id)
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
        signal_id: String,
        body: crate::datadogV1::model::SignalAssigneeUpdateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        match self
            .edit_security_monitoring_signal_assignee_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Modify the triage assignee of a security signal.
    pub async fn edit_security_monitoring_signal_assignee_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV1::model::SignalAssigneeUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalAssigneeError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.edit_security_monitoring_signal_assignee";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/assignee",
            local_configuration.get_operation_host(operation_id),
            signal_id = urlencode(signal_id)
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
        signal_id: String,
        body: crate::datadogV1::model::SignalStateUpdateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        match self
            .edit_security_monitoring_signal_state_with_http_info(signal_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Change the triage state of a security signal.
    pub async fn edit_security_monitoring_signal_state_with_http_info(
        &self,
        signal_id: String,
        body: crate::datadogV1::model::SignalStateUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SuccessfulSignalUpdateResponse>,
        Error<EditSecurityMonitoringSignalStateError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.edit_security_monitoring_signal_state";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/security_analytics/signals/{signal_id}/state",
            local_configuration.get_operation_host(operation_id),
            signal_id = urlencode(signal_id)
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

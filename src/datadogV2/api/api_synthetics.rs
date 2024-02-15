// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// GetOnDemandConcurrencyCapError is a struct for typed errors of method [`SyntheticsAPI::get_on_demand_concurrency_cap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOnDemandConcurrencyCapError {
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// SetOnDemandConcurrencyCapError is a struct for typed errors of method [`SyntheticsAPI::set_on_demand_concurrency_cap`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetOnDemandConcurrencyCapError {
    Status429(Option<crate::datadogV2::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct SyntheticsAPI {
    config: configuration::Configuration,
}

impl Default for SyntheticsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl SyntheticsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Get the on-demand concurrency cap.
    pub async fn get_on_demand_concurrency_cap(
        &self,
    ) -> Result<
        Option<crate::datadogV2::model::OnDemandConcurrencyCapResponse>,
        Error<GetOnDemandConcurrencyCapError>,
    > {
        match self.get_on_demand_concurrency_cap_with_http_info().await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get the on-demand concurrency cap.
    pub async fn get_on_demand_concurrency_cap_with_http_info(
        &self,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OnDemandConcurrencyCapResponse>,
        Error<GetOnDemandConcurrencyCapError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_on_demand_concurrency_cap";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/synthetics/settings/on_demand_concurrency_cap",
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
            let local_entity: Option<crate::datadogV2::model::OnDemandConcurrencyCapResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetOnDemandConcurrencyCapError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Save new value for on-demand concurrency cap.
    pub async fn set_on_demand_concurrency_cap(
        &self,
        body: crate::datadogV2::model::OnDemandConcurrencyCapAttributes,
    ) -> Result<
        Option<crate::datadogV2::model::OnDemandConcurrencyCapResponse>,
        Error<SetOnDemandConcurrencyCapError>,
    > {
        match self
            .set_on_demand_concurrency_cap_with_http_info(body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Save new value for on-demand concurrency cap.
    pub async fn set_on_demand_concurrency_cap_with_http_info(
        &self,
        body: crate::datadogV2::model::OnDemandConcurrencyCapAttributes,
    ) -> Result<
        ResponseContent<crate::datadogV2::model::OnDemandConcurrencyCapResponse>,
        Error<SetOnDemandConcurrencyCapError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.set_on_demand_concurrency_cap";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v2/synthetics/settings/on_demand_concurrency_cap",
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
            let local_entity: Option<crate::datadogV2::model::OnDemandConcurrencyCapResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<SetOnDemandConcurrencyCapError> =
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

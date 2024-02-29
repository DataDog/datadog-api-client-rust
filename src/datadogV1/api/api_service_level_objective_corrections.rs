// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// ListSLOCorrectionOptionalParams is a struct for passing parameters to the method [`ServiceLevelObjectiveCorrectionsAPI::list_slo_correction`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListSLOCorrectionOptionalParams {
    /// The specific offset to use as the beginning of the returned response.
    pub offset: Option<i64>,
    /// The number of SLO corrections to return in the response. Default is 25.
    pub limit: Option<i64>,
}

impl ListSLOCorrectionOptionalParams {
    /// The specific offset to use as the beginning of the returned response.
    pub fn offset(&mut self, value: i64) -> &mut Self {
        self.offset = Some(value);
        self
    }
    /// The number of SLO corrections to return in the response. Default is 25.
    pub fn limit(&mut self, value: i64) -> &mut Self {
        self.limit = Some(value);
        self
    }
}

/// CreateSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::create_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOCorrectionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::delete_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOCorrectionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::get_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::list_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOCorrectionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::update_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSLOCorrectionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct ServiceLevelObjectiveCorrectionsAPI {
    config: configuration::Configuration,
}

impl Default for ServiceLevelObjectiveCorrectionsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl ServiceLevelObjectiveCorrectionsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// Create an SLO Correction.
    pub async fn create_slo_correction(
        &self,
        body: crate::datadogV1::model::SLOCorrectionCreateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<CreateSLOCorrectionError>,
    > {
        match self.create_slo_correction_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an SLO Correction.
    pub async fn create_slo_correction_with_http_info(
        &self,
        body: crate::datadogV1::model::SLOCorrectionCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<CreateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_slo_correction";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction",
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
            let local_entity: Option<crate::datadogV1::model::SLOCorrectionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<CreateSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Permanently delete the specified SLO correction object.
    pub async fn delete_slo_correction(
        &self,
        slo_correction_id: String,
    ) -> Result<Option<()>, Error<DeleteSLOCorrectionError>> {
        match self
            .delete_slo_correction_with_http_info(slo_correction_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Permanently delete the specified SLO correction object.
    pub async fn delete_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
    ) -> Result<ResponseContent<()>, Error<DeleteSLOCorrectionError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_slo_correction";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = urlencode(slo_correction_id)
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
            let local_entity: Option<DeleteSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get an SLO correction.
    pub async fn get_slo_correction(
        &self,
        slo_correction_id: String,
    ) -> Result<Option<crate::datadogV1::model::SLOCorrectionResponse>, Error<GetSLOCorrectionError>>
    {
        match self
            .get_slo_correction_with_http_info(slo_correction_id)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an SLO correction.
    pub async fn get_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<GetSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slo_correction";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = urlencode(slo_correction_id)
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
            let local_entity: Option<crate::datadogV1::model::SLOCorrectionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<GetSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Get all Service Level Objective corrections.
    pub async fn list_slo_correction(
        &self,
        params: ListSLOCorrectionOptionalParams,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<ListSLOCorrectionError>,
    > {
        match self.list_slo_correction_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get all Service Level Objective corrections.
    pub async fn list_slo_correction_with_http_info(
        &self,
        params: ListSLOCorrectionOptionalParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<ListSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_slo_correction";

        // unbox and build optional parameters
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_query_param) = offset {
            local_req_builder =
                local_req_builder.query(&[("offset", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = limit {
            local_req_builder =
                local_req_builder.query(&[("limit", &local_query_param.to_string())]);
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
            let local_entity: Option<crate::datadogV1::model::SLOCorrectionListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<ListSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// Update the specified SLO correction object.
    pub async fn update_slo_correction(
        &self,
        slo_correction_id: String,
        body: crate::datadogV1::model::SLOCorrectionUpdateRequest,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<UpdateSLOCorrectionError>,
    > {
        match self
            .update_slo_correction_with_http_info(slo_correction_id, body)
            .await
        {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the specified SLO correction object.
    pub async fn update_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
        body: crate::datadogV1::model::SLOCorrectionUpdateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<UpdateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_slo_correction";

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = urlencode(slo_correction_id)
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
            let local_entity: Option<crate::datadogV1::model::SLOCorrectionResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
        } else {
            let local_entity: Option<UpdateSLOCorrectionError> =
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

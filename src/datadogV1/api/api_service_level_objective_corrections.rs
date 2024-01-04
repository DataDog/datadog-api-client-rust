// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

/// CreateSLOCorrectionParams is a struct for passing parameters to the method [`CreateSLOCorrection`]
#[derive(Clone, Debug)]
pub struct CreateSLOCorrectionParams {
    /// Create an SLO Correction
    pub body: crate::datadogV1::model::SLOCorrectionCreateRequest,
}

/// DeleteSLOCorrectionParams is a struct for passing parameters to the method [`DeleteSLOCorrection`]
#[derive(Clone, Debug)]
pub struct DeleteSLOCorrectionParams {
    /// The ID of the SLO correction object.
    pub slo_correction_id: String,
}

/// GetSLOCorrectionParams is a struct for passing parameters to the method [`GetSLOCorrection`]
#[derive(Clone, Debug)]
pub struct GetSLOCorrectionParams {
    /// The ID of the SLO correction object.
    pub slo_correction_id: String,
}

/// ListSLOCorrectionParams is a struct for passing parameters to the method [`ListSLOCorrection`]
#[derive(Clone, Debug)]
pub struct ListSLOCorrectionParams {
    /// The specific offset to use as the beginning of the returned response.
    pub offset: Option<i64>,
    /// The number of SLO corrections to return in the response. Default is 25.
    pub limit: Option<i64>,
}

/// UpdateSLOCorrectionParams is a struct for passing parameters to the method [`UpdateSLOCorrection`]
#[derive(Clone, Debug)]
pub struct UpdateSLOCorrectionParams {
    /// The ID of the SLO correction object.
    pub slo_correction_id: String,
    /// The edited SLO correction object.
    pub body: crate::datadogV1::model::SLOCorrectionUpdateRequest,
}

/// CreateSLOCorrectionError is a struct for typed errors of method [`CreateSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOCorrectionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOCorrectionError is a struct for typed errors of method [`DeleteSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOCorrectionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionError is a struct for typed errors of method [`GetSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListSLOCorrectionError is a struct for typed errors of method [`ListSLOCorrection`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOCorrectionError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOCorrectionError is a struct for typed errors of method [`UpdateSLOCorrection`]
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
        params: CreateSLOCorrectionParams,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<CreateSLOCorrectionError>,
    > {
        match self.create_slo_correction_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Create an SLO Correction.
    pub async fn create_slo_correction_with_http_info(
        &self,
        params: CreateSLOCorrectionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<CreateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo/correction", local_configuration.base_path);
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
        params: DeleteSLOCorrectionParams,
    ) -> Result<Option<()>, Error<DeleteSLOCorrectionError>> {
        match self.delete_slo_correction_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Permanently delete the specified SLO correction object.
    pub async fn delete_slo_correction_with_http_info(
        &self,
        params: DeleteSLOCorrectionParams,
    ) -> Result<ResponseContent<()>, Error<DeleteSLOCorrectionError>> {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_correction_id = params.slo_correction_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.base_path,
            slo_correction_id = urlencode(slo_correction_id)
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
        params: GetSLOCorrectionParams,
    ) -> Result<Option<crate::datadogV1::model::SLOCorrectionResponse>, Error<GetSLOCorrectionError>>
    {
        match self.get_slo_correction_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Get an SLO correction.
    pub async fn get_slo_correction_with_http_info(
        &self,
        params: GetSLOCorrectionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<GetSLOCorrectionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_correction_id = params.slo_correction_id;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.base_path,
            slo_correction_id = urlencode(slo_correction_id)
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
        params: ListSLOCorrectionParams,
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
        params: ListSLOCorrectionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionListResponse>,
        Error<ListSLOCorrectionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &local_configuration.client;

        let local_uri_str = format!("{}/api/v1/slo/correction", local_configuration.base_path);
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        if let Some(ref local_str) = offset {
            local_req_builder = local_req_builder.query(&[("offset", &local_str.to_string())]);
        };
        if let Some(ref local_str) = limit {
            local_req_builder = local_req_builder.query(&[("limit", &local_str.to_string())]);
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
        params: UpdateSLOCorrectionParams,
    ) -> Result<
        Option<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<UpdateSLOCorrectionError>,
    > {
        match self.update_slo_correction_with_http_info(params).await {
            Ok(response_content) => Ok(response_content.entity),
            Err(err) => Err(err),
        }
    }

    /// Update the specified SLO correction object.
    pub async fn update_slo_correction_with_http_info(
        &self,
        params: UpdateSLOCorrectionParams,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        Error<UpdateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;

        // unbox and build parameters
        let slo_correction_id = params.slo_correction_id;
        let body = params.body;

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.base_path,
            slo_correction_id = urlencode(slo_correction_id)
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

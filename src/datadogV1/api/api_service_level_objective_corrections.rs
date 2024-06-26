// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use async_stream::try_stream;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use futures_core::stream::Stream;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

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
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }
    /// The number of SLO corrections to return in the response. Default is 25.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
}

/// CreateSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::create_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSLOCorrectionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// DeleteSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::delete_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSLOCorrectionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::get_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSLOCorrectionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::list_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSLOCorrectionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSLOCorrectionError is a struct for typed errors of method [`ServiceLevelObjectiveCorrectionsAPI::update_slo_correction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSLOCorrectionError {
    APIErrorResponse(crate::datadogV1::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// SLO Status Corrections allow you to prevent specific time periods from negatively impacting
/// your SLO’s status and error budget. You can use Status Corrections for various purposes, such
/// as removing planned maintenance windows, non-business hours, or other time periods that do
/// not correspond to genuine issues. See [SLO status corrections](<https://docs.datadoghq.com/service_management/service_level_objectives/#slo-status-corrections>) for more information.
#[derive(Debug, Clone)]
pub struct ServiceLevelObjectiveCorrectionsAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ServiceLevelObjectiveCorrectionsAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ServiceLevelObjectiveCorrectionsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
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
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Create an SLO Correction.
    pub async fn create_slo_correction(
        &self,
        body: crate::datadogV1::model::SLOCorrectionCreateRequest,
    ) -> Result<
        crate::datadogV1::model::SLOCorrectionResponse,
        datadog::Error<CreateSLOCorrectionError>,
    > {
        match self.create_slo_correction_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Create an SLO Correction.
    pub async fn create_slo_correction_with_http_info(
        &self,
        body: crate::datadogV1::model::SLOCorrectionCreateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        datadog::Error<CreateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_slo_correction";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SLOCorrectionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Permanently delete the specified SLO correction object.
    pub async fn delete_slo_correction(
        &self,
        slo_correction_id: String,
    ) -> Result<(), datadog::Error<DeleteSLOCorrectionError>> {
        match self
            .delete_slo_correction_with_http_info(slo_correction_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Permanently delete the specified SLO correction object.
    pub async fn delete_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteSLOCorrectionError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.delete_slo_correction";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = datadog::urlencode(slo_correction_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::DELETE, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("*/*"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            Ok(datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: None,
            })
        } else {
            let local_entity: Option<DeleteSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get an SLO correction.
    pub async fn get_slo_correction(
        &self,
        slo_correction_id: String,
    ) -> Result<crate::datadogV1::model::SLOCorrectionResponse, datadog::Error<GetSLOCorrectionError>>
    {
        match self
            .get_slo_correction_with_http_info(slo_correction_id)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Get an SLO correction.
    pub async fn get_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        datadog::Error<GetSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.get_slo_correction";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = datadog::urlencode(slo_correction_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SLOCorrectionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Get all Service Level Objective corrections.
    pub async fn list_slo_correction(
        &self,
        params: ListSLOCorrectionOptionalParams,
    ) -> Result<
        crate::datadogV1::model::SLOCorrectionListResponse,
        datadog::Error<ListSLOCorrectionError>,
    > {
        match self.list_slo_correction_with_http_info(params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn list_slo_correction_with_pagination(
        &self,
        mut params: ListSLOCorrectionOptionalParams,
    ) -> impl Stream<
        Item = Result<
            crate::datadogV1::model::SLOCorrection,
            datadog::Error<ListSLOCorrectionError>,
        >,
    > + '_ {
        try_stream! {
            let mut page_size: i64 = 25;
            if params.limit.is_none() {
                params.limit = Some(page_size);
            } else {
                page_size = params.limit.unwrap().clone();
            }
            loop {
                let resp = self.list_slo_correction(params.clone()).await?;
                let Some(data) = resp.data else { break };

                let r = data;
                let count = r.len();
                for team in r {
                    yield team;
                }

                if count < page_size as usize {
                    break;
                }
                if params.offset.is_none() {
                    params.offset = Some(page_size.clone());
                } else {
                    params.offset = Some(params.offset.unwrap() + page_size.clone());
                }
            }
        }
    }

    /// Get all Service Level Objective corrections.
    pub async fn list_slo_correction_with_http_info(
        &self,
        params: ListSLOCorrectionOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::SLOCorrectionListResponse>,
        datadog::Error<ListSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.list_slo_correction";

        // unbox and build optional parameters
        let offset = params.offset;
        let limit = params.limit;

        let local_client = &self.client;

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

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SLOCorrectionListResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Update the specified SLO correction object.
    pub async fn update_slo_correction(
        &self,
        slo_correction_id: String,
        body: crate::datadogV1::model::SLOCorrectionUpdateRequest,
    ) -> Result<
        crate::datadogV1::model::SLOCorrectionResponse,
        datadog::Error<UpdateSLOCorrectionError>,
    > {
        match self
            .update_slo_correction_with_http_info(slo_correction_id, body)
            .await
        {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(datadog::Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Update the specified SLO correction object.
    pub async fn update_slo_correction_with_http_info(
        &self,
        slo_correction_id: String,
        body: crate::datadogV1::model::SLOCorrectionUpdateRequest,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV1::model::SLOCorrectionResponse>,
        datadog::Error<UpdateSLOCorrectionError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.update_slo_correction";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/slo/correction/{slo_correction_id}",
            local_configuration.get_operation_host(operation_id),
            slo_correction_id = datadog::urlencode(slo_correction_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::PATCH, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        // build user agent
        match HeaderValue::from_str(local_configuration.user_agent.as_str()) {
            Ok(user_agent) => headers.insert(reqwest::header::USER_AGENT, user_agent),
            Err(e) => {
                log::warn!("Failed to parse user agent header: {e}, falling back to default");
                headers.insert(
                    reqwest::header::USER_AGENT,
                    HeaderValue::from_static(datadog::DEFAULT_USER_AGENT.as_str()),
                )
            }
        };

        // build auth
        if let Some(local_key) = local_configuration.auth_keys.get("apiKeyAuth") {
            headers.insert(
                "DD-API-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-API-KEY header"),
            );
        };
        if let Some(local_key) = local_configuration.auth_keys.get("appKeyAuth") {
            headers.insert(
                "DD-APPLICATION-KEY",
                HeaderValue::from_str(local_key.key.as_str())
                    .expect("failed to parse DD-APPLICATION-KEY header"),
            );
        };

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, datadog::DDFormatter);
        if body.serialize(&mut ser).is_ok() {
            if let Some(content_encoding) = headers.get("Content-Encoding") {
                match content_encoding.to_str().unwrap_or_default() {
                    "gzip" => {
                        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(datadog::Error::Io(e)),
                        }
                    }
                    _ => {
                        local_req_builder = local_req_builder.body(ser.into_inner());
                    }
                }
            } else {
                local_req_builder = local_req_builder.body(ser.into_inner());
            }
        }

        local_req_builder = local_req_builder.headers(headers);
        let local_req = local_req_builder.build()?;
        log::debug!("request content: {:?}", local_req.body());
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;
        log::debug!("response content: {}", local_content);

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::SLOCorrectionResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(datadog::ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<UpdateSLOCorrectionError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }
}

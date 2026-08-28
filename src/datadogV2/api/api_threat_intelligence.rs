// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// AddSTIXThreatIntelOptionalParams is a struct for passing parameters to the method [`ThreatIntelligenceAPI::add_stix_threat_intel`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct AddSTIXThreatIntelOptionalParams {
    /// Content encoding for the request body. Use gzip for a compressed STIX bundle.
    pub content_encoding: Option<crate::datadogV2::model::STIXContentEncoding>,
}

impl AddSTIXThreatIntelOptionalParams {
    /// Content encoding for the request body. Use gzip for a compressed STIX bundle.
    pub fn content_encoding(mut self, value: crate::datadogV2::model::STIXContentEncoding) -> Self {
        self.content_encoding = Some(value);
        self
    }
}

/// AddSTIXThreatIntelError is a struct for typed errors of method [`ThreatIntelligenceAPI::add_stix_threat_intel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddSTIXThreatIntelError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Ingest and manage threat intelligence data for security enrichment and investigation.
#[derive(Debug, Clone)]
pub struct ThreatIntelligenceAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ThreatIntelligenceAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ThreatIntelligenceAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: datadog::Configuration) -> Self {
        let reqwest_client_builder = {
            let builder = config.apply_headers(reqwest::Client::builder());
            #[cfg(not(target_arch = "wasm32"))]
            let builder = if let Some(proxy_url) = &config.proxy_url {
                builder.proxy(reqwest::Proxy::all(proxy_url).expect("Failed to parse proxy URL"))
            } else {
                builder
            };
            builder
        };

        let middleware_client_builder = {
            let builder =
                reqwest_middleware::ClientBuilder::new(reqwest_client_builder.build().unwrap());
            #[cfg(feature = "retry")]
            let builder = if config.enable_retry {
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

                builder.with(retry_middleware)
            } else {
                builder
            };
            builder
        };

        let client = middleware_client_builder.build();

        Self { config, client }
    }

    pub fn with_client_and_config(
        config: datadog::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// Ingest a STIX 2.1 bundle containing threat intelligence indicators. Only indicator objects are processed. Supported indicator patterns contain IPv4 addresses, IPv6 addresses, domain names, or SHA-256 file hashes.
    ///
    /// Non-indicator objects are ignored and are not included in the response counters. Indicator objects with unsupported STIX versions or patterns that produce no supported observable values increment the `unsupported` counter. Patterns that cannot be parsed increment the `invalid` counter. Processing is best effort, so valid supported indicators in the same bundle are still added.
    ///
    /// A successful response means ingestion has completed. Reference-table materialization and enrichment happen asynchronously. Requests are limited to 50 MB as received, 100 MB after decompression, and 10 requests per second per API key. Gzip-compressed request bodies are supported.
    pub async fn add_stix_threat_intel(
        &self,
        ti_vendor: String,
        body: crate::datadogV2::model::STIXBundleRequest,
        params: AddSTIXThreatIntelOptionalParams,
    ) -> Result<crate::datadogV2::model::STIXIngestResponse, datadog::Error<AddSTIXThreatIntelError>>
    {
        match self
            .add_stix_threat_intel_with_http_info(ti_vendor, body, params)
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

    /// Ingest a STIX 2.1 bundle containing threat intelligence indicators. Only indicator objects are processed. Supported indicator patterns contain IPv4 addresses, IPv6 addresses, domain names, or SHA-256 file hashes.
    ///
    /// Non-indicator objects are ignored and are not included in the response counters. Indicator objects with unsupported STIX versions or patterns that produce no supported observable values increment the `unsupported` counter. Patterns that cannot be parsed increment the `invalid` counter. Processing is best effort, so valid supported indicators in the same bundle are still added.
    ///
    /// A successful response means ingestion has completed. Reference-table materialization and enrichment happen asynchronously. Requests are limited to 50 MB as received, 100 MB after decompression, and 10 requests per second per API key. Gzip-compressed request bodies are supported.
    pub async fn add_stix_threat_intel_with_http_info(
        &self,
        ti_vendor: String,
        body: crate::datadogV2::model::STIXBundleRequest,
        params: AddSTIXThreatIntelOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::STIXIngestResponse>,
        datadog::Error<AddSTIXThreatIntelError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.add_stix_threat_intel";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.add_stix_threat_intel' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let content_encoding = params.content_encoding;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/security/threat-intel/stix",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        // build headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        headers.insert(
            "ti_vendor",
            ti_vendor
                .to_string()
                .parse()
                .expect("failed to parse ti_vendor header"),
        );
        if let Some(ref local) = content_encoding {
            headers.insert(
                "Content-Encoding",
                local
                    .to_string()
                    .parse()
                    .expect("failed to parse Content-Encoding header"),
            );
        }

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
                    #[cfg(feature = "zstd")]
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
            match serde_json::from_str::<crate::datadogV2::model::STIXIngestResponse>(
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
            let local_entity: Option<AddSTIXThreatIntelError> =
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

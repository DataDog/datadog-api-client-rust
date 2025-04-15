// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// UpdateRestrictionPolicyOptionalParams is a struct for passing parameters to the method [`RestrictionPoliciesAPI::update_restriction_policy`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct UpdateRestrictionPolicyOptionalParams {
    /// Allows admins (users with the `user_access_manage` permission) to remove their own access from the resource if set to `true`. By default, this is set to `false`, preventing admins from locking themselves out.
    pub allow_self_lockout: Option<bool>,
}

impl UpdateRestrictionPolicyOptionalParams {
    /// Allows admins (users with the `user_access_manage` permission) to remove their own access from the resource if set to `true`. By default, this is set to `false`, preventing admins from locking themselves out.
    pub fn allow_self_lockout(mut self, value: bool) -> Self {
        self.allow_self_lockout = Some(value);
        self
    }
}

/// DeleteRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::delete_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRestrictionPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::get_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRestrictionPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateRestrictionPolicyError is a struct for typed errors of method [`RestrictionPoliciesAPI::update_restriction_policy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRestrictionPolicyError {
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// A restriction policy defines the access control rules for a resource, mapping a set of relations
/// (such as editor and viewer) to a set of allowed principals (such as roles, teams, or users).
/// The restriction policy determines who is authorized to perform what actions on the resource.
#[derive(Debug, Clone)]
pub struct RestrictionPoliciesAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for RestrictionPoliciesAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl RestrictionPoliciesAPI {
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

    /// Deletes the restriction policy associated with a specified resource.
    pub async fn delete_restriction_policy(
        &self,
        resource_id: String,
    ) -> Result<(), datadog::Error<DeleteRestrictionPolicyError>> {
        match self
            .delete_restriction_policy_with_http_info(resource_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Deletes the restriction policy associated with a specified resource.
    pub async fn delete_restriction_policy_with_http_info(
        &self,
        resource_id: String,
    ) -> Result<datadog::ResponseContent<()>, datadog::Error<DeleteRestrictionPolicyError>> {
        let local_configuration = &self.config;
        let operation_id = "v2.delete_restriction_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id)
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
            let local_entity: Option<DeleteRestrictionPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Retrieves the restriction policy associated with a specified resource.
    pub async fn get_restriction_policy(
        &self,
        resource_id: String,
    ) -> Result<
        crate::datadogV2::model::RestrictionPolicyResponse,
        datadog::Error<GetRestrictionPolicyError>,
    > {
        match self
            .get_restriction_policy_with_http_info(resource_id)
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

    /// Retrieves the restriction policy associated with a specified resource.
    pub async fn get_restriction_policy_with_http_info(
        &self,
        resource_id: String,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        datadog::Error<GetRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.get_restriction_policy";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id)
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
            match serde_json::from_str::<crate::datadogV2::model::RestrictionPolicyResponse>(
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
            let local_entity: Option<GetRestrictionPolicyError> =
                serde_json::from_str(&local_content).ok();
            let local_error = datadog::ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(datadog::Error::ResponseError(local_error))
        }
    }

    /// Updates the restriction policy associated with a resource.
    ///
    /// #### Supported resources
    /// Restriction policies can be applied to the following resources:
    /// - Dashboards: `dashboard`
    /// - Integration Accounts: `integration-account`
    /// - Integration Services: `integration-service`
    /// - Integration Webhooks: `integration-webhook`
    /// - Notebooks: `notebook`
    /// - Powerpacks: `powerpack`
    /// - Reference Tables: `reference-table`
    /// - Security Rules: `security-rule`
    /// - Service Level Objectives: `slo`
    /// - Synthetic Global Variables: `synthetics-global-variable`
    /// - Synthetic Tests: `synthetics-test`
    /// - Synthetic Private Locations: `synthetics-private-location`
    /// - Monitors: `monitor`
    /// - Workflows: `workflow`
    /// - App Builder Apps: `app-builder-app`
    /// - Connections: `connection`
    /// - Connection Groups: `connection-group`
    ///
    /// #### Supported relations for resources
    /// Resource Type               | Supported Relations
    /// ----------------------------|--------------------------
    /// Dashboards                  | `viewer`, `editor`
    /// Integration Accounts        | `viewer`, `editor`
    /// Integration Services        | `viewer`, `editor`
    /// Integration Webhooks        | `viewer`, `editor`
    /// Notebooks                   | `viewer`, `editor`
    /// Powerpacks                  | `viewer`, `editor`
    /// Security Rules              | `viewer`, `editor`
    /// Service Level Objectives    | `viewer`, `editor`
    /// Synthetic Global Variables  | `viewer`, `editor`
    /// Synthetic Tests             | `viewer`, `editor`
    /// Synthetic Private Locations | `viewer`, `editor`
    /// Monitors                    | `viewer`, `editor`
    /// Reference Tables            | `viewer`, `editor`
    /// Workflows                   | `viewer`, `runner`, `editor`
    /// App Builder Apps            | `viewer`, `editor`
    /// Connections                 | `viewer`, `resolver`, `editor`
    /// Connection Groups           | `viewer`, `editor`
    pub async fn update_restriction_policy(
        &self,
        resource_id: String,
        body: crate::datadogV2::model::RestrictionPolicyUpdateRequest,
        params: UpdateRestrictionPolicyOptionalParams,
    ) -> Result<
        crate::datadogV2::model::RestrictionPolicyResponse,
        datadog::Error<UpdateRestrictionPolicyError>,
    > {
        match self
            .update_restriction_policy_with_http_info(resource_id, body, params)
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

    /// Updates the restriction policy associated with a resource.
    ///
    /// #### Supported resources
    /// Restriction policies can be applied to the following resources:
    /// - Dashboards: `dashboard`
    /// - Integration Accounts: `integration-account`
    /// - Integration Services: `integration-service`
    /// - Integration Webhooks: `integration-webhook`
    /// - Notebooks: `notebook`
    /// - Powerpacks: `powerpack`
    /// - Reference Tables: `reference-table`
    /// - Security Rules: `security-rule`
    /// - Service Level Objectives: `slo`
    /// - Synthetic Global Variables: `synthetics-global-variable`
    /// - Synthetic Tests: `synthetics-test`
    /// - Synthetic Private Locations: `synthetics-private-location`
    /// - Monitors: `monitor`
    /// - Workflows: `workflow`
    /// - App Builder Apps: `app-builder-app`
    /// - Connections: `connection`
    /// - Connection Groups: `connection-group`
    ///
    /// #### Supported relations for resources
    /// Resource Type               | Supported Relations
    /// ----------------------------|--------------------------
    /// Dashboards                  | `viewer`, `editor`
    /// Integration Accounts        | `viewer`, `editor`
    /// Integration Services        | `viewer`, `editor`
    /// Integration Webhooks        | `viewer`, `editor`
    /// Notebooks                   | `viewer`, `editor`
    /// Powerpacks                  | `viewer`, `editor`
    /// Security Rules              | `viewer`, `editor`
    /// Service Level Objectives    | `viewer`, `editor`
    /// Synthetic Global Variables  | `viewer`, `editor`
    /// Synthetic Tests             | `viewer`, `editor`
    /// Synthetic Private Locations | `viewer`, `editor`
    /// Monitors                    | `viewer`, `editor`
    /// Reference Tables            | `viewer`, `editor`
    /// Workflows                   | `viewer`, `runner`, `editor`
    /// App Builder Apps            | `viewer`, `editor`
    /// Connections                 | `viewer`, `resolver`, `editor`
    /// Connection Groups           | `viewer`, `editor`
    pub async fn update_restriction_policy_with_http_info(
        &self,
        resource_id: String,
        body: crate::datadogV2::model::RestrictionPolicyUpdateRequest,
        params: UpdateRestrictionPolicyOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::RestrictionPolicyResponse>,
        datadog::Error<UpdateRestrictionPolicyError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v2.update_restriction_policy";

        // unbox and build optional parameters
        let allow_self_lockout = params.allow_self_lockout;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/restriction_policy/{resource_id}",
            local_configuration.get_operation_host(operation_id),
            resource_id = datadog::urlencode(resource_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::POST, local_uri_str.as_str());

        if let Some(ref local_query_param) = allow_self_lockout {
            local_req_builder =
                local_req_builder.query(&[("allow_self_lockout", &local_query_param.to_string())]);
        };

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
            match serde_json::from_str::<crate::datadogV2::model::RestrictionPolicyResponse>(
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
            let local_entity: Option<UpdateRestrictionPolicyError> =
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

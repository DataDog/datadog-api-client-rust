// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

/// ListProductCatalogSKUsOptionalParams is a struct for passing parameters to the method [`ProductCatalogAPI::list_product_catalog_sk_us`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListProductCatalogSKUsOptionalParams {
    /// The date the returned prices, allotments, and pricing tiers are effective as of, in
    /// `YYYY-MM-DD` format. Defaults to the date of the request, and must not be later
    /// than it.
    pub as_of_date: Option<String>,
}

impl ListProductCatalogSKUsOptionalParams {
    /// The date the returned prices, allotments, and pricing tiers are effective as of, in
    /// `YYYY-MM-DD` format. Defaults to the date of the request, and must not be later
    /// than it.
    pub fn as_of_date(mut self, value: String) -> Self {
        self.as_of_date = Some(value);
        self
    }
}

/// ListProductCatalogSKUsError is a struct for typed errors of method [`ProductCatalogAPI::list_product_catalog_sk_us`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProductCatalogSKUsError {
    JSONAPIErrorResponse(crate::datadogV2::model::JSONAPIErrorResponse),
    APIErrorResponse(crate::datadogV2::model::APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Look up the Datadog SKUs that are generally available, together with the public list
/// prices, allotments, and tiered pricing that apply to them on a given date.
#[derive(Debug, Clone)]
pub struct ProductCatalogAPI {
    config: datadog::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for ProductCatalogAPI {
    fn default() -> Self {
        Self::with_config(datadog::Configuration::default())
    }
}

impl ProductCatalogAPI {
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

    /// Get every generally available Datadog SKU, with the pricing and allotment metadata that
    /// applies to it, for the Datadog site serving the request. A SKU is generally available
    /// when it is billed through a metered commitment or through automatic billing; SKUs in any
    /// other phase are not returned.
    ///
    /// Prices, allotments, and pricing tiers are returned as they were in effect on
    /// `as_of_date`, which defaults to the date of the request. Prices are public list prices:
    /// they do not reflect discounts, commitments, or negotiated rates on an account.
    ///
    /// Each SKU is a separate resource in `data`, identified by its SKU code, and sorted by
    /// that code in ascending order. The whole catalog is returned in a single response, so
    /// this endpoint is not paginated.
    pub async fn list_product_catalog_sk_us(
        &self,
        version: crate::datadogV2::model::ProductCatalogSKUsAPIVersion,
        params: ListProductCatalogSKUsOptionalParams,
    ) -> Result<
        crate::datadogV2::model::ProductCatalogSKUsResponse,
        datadog::Error<ListProductCatalogSKUsError>,
    > {
        match self
            .list_product_catalog_sk_us_with_http_info(version, params)
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

    /// Get every generally available Datadog SKU, with the pricing and allotment metadata that
    /// applies to it, for the Datadog site serving the request. A SKU is generally available
    /// when it is billed through a metered commitment or through automatic billing; SKUs in any
    /// other phase are not returned.
    ///
    /// Prices, allotments, and pricing tiers are returned as they were in effect on
    /// `as_of_date`, which defaults to the date of the request. Prices are public list prices:
    /// they do not reflect discounts, commitments, or negotiated rates on an account.
    ///
    /// Each SKU is a separate resource in `data`, identified by its SKU code, and sorted by
    /// that code in ascending order. The whole catalog is returned in a single response, so
    /// this endpoint is not paginated.
    pub async fn list_product_catalog_sk_us_with_http_info(
        &self,
        version: crate::datadogV2::model::ProductCatalogSKUsAPIVersion,
        params: ListProductCatalogSKUsOptionalParams,
    ) -> Result<
        datadog::ResponseContent<crate::datadogV2::model::ProductCatalogSKUsResponse>,
        datadog::Error<ListProductCatalogSKUsError>,
    > {
        let local_configuration = &self.config;
        let local_operation_id = "v2.list_product_catalog_sk_us";
        if local_configuration.is_unstable_operation_enabled(local_operation_id) {
            warn!("Using unstable operation {local_operation_id}");
        } else {
            let local_error = datadog::UnstableOperationDisabledError {
                msg: "Operation 'v2.list_product_catalog_sk_us' is not enabled".to_string(),
            };
            return Err(datadog::Error::UnstableOperationDisabledError(local_error));
        }

        // unbox and build optional parameters
        let as_of_date = params.as_of_date;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v2/product-catalog/skus",
            local_configuration.get_operation_host(local_operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("version", &version.to_string())]);
        if let Some(ref local_query_param) = as_of_date {
            local_req_builder =
                local_req_builder.query(&[("as_of_date", &local_query_param.to_string())]);
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
            match serde_json::from_str::<crate::datadogV2::model::ProductCatalogSKUsResponse>(
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
            let local_entity: Option<ListProductCatalogSKUsError> =
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

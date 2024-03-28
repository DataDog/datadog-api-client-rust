// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use flate2::{
    write::{GzEncoder, ZlibEncoder},
    Compression,
};
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// ListEventsOptionalParams is a struct for passing parameters to the method [`EventsAPI::list_events`]
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct ListEventsOptionalParams {
    /// Priority of your events, either `low` or `normal`.
    pub priority: Option<crate::datadogV1::model::EventPriority>,
    /// A comma separated string of sources.
    pub sources: Option<String>,
    /// A comma separated list indicating what tags, if any, should be used to filter the list of events.
    pub tags: Option<String>,
    /// Set unaggregated to `true` to return all events within the specified [`start`,`end`] timeframe.
    /// Otherwise if an event is aggregated to a parent event with a timestamp outside of the timeframe,
    /// it won't be available in the output. Aggregated events with `is_aggregate=true` in the response will still be returned unless exclude_aggregate is set to `true.`
    pub unaggregated: Option<bool>,
    /// Set `exclude_aggregate` to `true` to only return unaggregated events where `is_aggregate=false` in the response. If the `exclude_aggregate` parameter is set to `true`,
    /// then the unaggregated parameter is ignored and will be `true` by default.
    pub exclude_aggregate: Option<bool>,
    /// By default 1000 results are returned per request. Set page to the number of the page to return with `0` being the first page. The page parameter can only be used
    /// when either unaggregated or exclude_aggregate is set to `true.`
    pub page: Option<i32>,
}

impl ListEventsOptionalParams {
    /// Priority of your events, either `low` or `normal`.
    pub fn priority(mut self, value: crate::datadogV1::model::EventPriority) -> Self {
        self.priority = Some(value);
        self
    }
    /// A comma separated string of sources.
    pub fn sources(mut self, value: String) -> Self {
        self.sources = Some(value);
        self
    }
    /// A comma separated list indicating what tags, if any, should be used to filter the list of events.
    pub fn tags(mut self, value: String) -> Self {
        self.tags = Some(value);
        self
    }
    /// Set unaggregated to `true` to return all events within the specified [`start`,`end`] timeframe.
    /// Otherwise if an event is aggregated to a parent event with a timestamp outside of the timeframe,
    /// it won't be available in the output. Aggregated events with `is_aggregate=true` in the response will still be returned unless exclude_aggregate is set to `true.`
    pub fn unaggregated(mut self, value: bool) -> Self {
        self.unaggregated = Some(value);
        self
    }
    /// Set `exclude_aggregate` to `true` to only return unaggregated events where `is_aggregate=false` in the response. If the `exclude_aggregate` parameter is set to `true`,
    /// then the unaggregated parameter is ignored and will be `true` by default.
    pub fn exclude_aggregate(mut self, value: bool) -> Self {
        self.exclude_aggregate = Some(value);
        self
    }
    /// By default 1000 results are returned per request. Set page to the number of the page to return with `0` being the first page. The page parameter can only be used
    /// when either unaggregated or exclude_aggregate is set to `true.`
    pub fn page(mut self, value: i32) -> Self {
        self.page = Some(value);
        self
    }
}

/// CreateEventError is a struct for typed errors of method [`EventsAPI::create_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEventError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// GetEventError is a struct for typed errors of method [`EventsAPI::get_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventError {
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status404(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

/// ListEventsError is a struct for typed errors of method [`EventsAPI::list_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventsError {
    Status400(Option<crate::datadogV1::model::APIErrorResponse>),
    Status403(Option<crate::datadogV1::model::APIErrorResponse>),
    Status429(Option<crate::datadogV1::model::APIErrorResponse>),
    UnknownValue(serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct EventsAPI {
    config: configuration::Configuration,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Default for EventsAPI {
    fn default() -> Self {
        Self::with_config(configuration::Configuration::default())
    }
}

impl EventsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
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
        config: configuration::Configuration,
        client: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self { config, client }
    }

    /// This endpoint allows you to post events to the stream.
    /// Tag them, set priority and event aggregate them with other events.
    pub async fn create_event(
        &self,
        body: crate::datadogV1::model::EventCreateRequest,
    ) -> Result<crate::datadogV1::model::EventCreateResponse, Error<CreateEventError>> {
        match self.create_event_with_http_info(body).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to post events to the stream.
    /// Tag them, set priority and event aggregate them with other events.
    pub async fn create_event_with_http_info(
        &self,
        body: crate::datadogV1::model::EventCreateRequest,
    ) -> Result<
        ResponseContent<crate::datadogV1::model::EventCreateResponse>,
        Error<CreateEventError>,
    > {
        let local_configuration = &self.config;
        let operation_id = "v1.create_event";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/events",
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
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
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

        // build body parameters
        let output = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(output, DDFormatter);
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
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "deflate" => {
                        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
                        }
                    }
                    "zstd1" => {
                        let mut enc = zstd::stream::Encoder::new(Vec::new(), 0).unwrap();
                        let _ = enc.write_all(ser.into_inner().as_slice());
                        match enc.finish() {
                            Ok(buf) => {
                                local_req_builder = local_req_builder.body(buf);
                            }
                            Err(e) => return Err(Error::Io(e)),
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::EventCreateResponse>(
                &local_content,
            ) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<CreateEventError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// This endpoint allows you to query for event details.
    ///
    /// **Note**: If the event you’re querying contains markdown formatting of any kind,
    /// you may see characters such as `%`,`\`,`n` in your output.
    pub async fn get_event(
        &self,
        event_id: i64,
    ) -> Result<crate::datadogV1::model::EventResponse, Error<GetEventError>> {
        match self.get_event_with_http_info(event_id).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// This endpoint allows you to query for event details.
    ///
    /// **Note**: If the event you’re querying contains markdown formatting of any kind,
    /// you may see characters such as `%`,`\`,`n` in your output.
    pub async fn get_event_with_http_info(
        &self,
        event_id: i64,
    ) -> Result<ResponseContent<crate::datadogV1::model::EventResponse>, Error<GetEventError>> {
        let local_configuration = &self.config;
        let operation_id = "v1.get_event";

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/events/{event_id}",
            local_configuration.get_operation_host(operation_id),
            event_id = event_id
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
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::EventResponse>(&local_content) {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<GetEventError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }

    /// The event stream can be queried and filtered by time, priority, sources and tags.
    ///
    /// **Notes**:
    /// - If the event you’re querying contains markdown formatting of any kind,
    /// you may see characters such as `%`,`\`,`n` in your output.
    ///
    /// - This endpoint returns a maximum of `1000` most recent results. To return additional results,
    /// identify the last timestamp of the last result and set that as the `end` query time to
    /// paginate the results. You can also use the page parameter to specify which set of `1000` results to return.
    pub async fn list_events(
        &self,
        start: i64,
        end: i64,
        params: ListEventsOptionalParams,
    ) -> Result<crate::datadogV1::model::EventListResponse, Error<ListEventsError>> {
        match self.list_events_with_http_info(start, end, params).await {
            Ok(response_content) => {
                if let Some(e) = response_content.entity {
                    Ok(e)
                } else {
                    Err(Error::Serde(serde::de::Error::custom(
                        "response content was None",
                    )))
                }
            }
            Err(err) => Err(err),
        }
    }

    /// The event stream can be queried and filtered by time, priority, sources and tags.
    ///
    /// **Notes**:
    /// - If the event you’re querying contains markdown formatting of any kind,
    /// you may see characters such as `%`,`\`,`n` in your output.
    ///
    /// - This endpoint returns a maximum of `1000` most recent results. To return additional results,
    /// identify the last timestamp of the last result and set that as the `end` query time to
    /// paginate the results. You can also use the page parameter to specify which set of `1000` results to return.
    pub async fn list_events_with_http_info(
        &self,
        start: i64,
        end: i64,
        params: ListEventsOptionalParams,
    ) -> Result<ResponseContent<crate::datadogV1::model::EventListResponse>, Error<ListEventsError>>
    {
        let local_configuration = &self.config;
        let operation_id = "v1.list_events";

        // unbox and build optional parameters
        let priority = params.priority;
        let sources = params.sources;
        let tags = params.tags;
        let unaggregated = params.unaggregated;
        let exclude_aggregate = params.exclude_aggregate;
        let page = params.page;

        let local_client = &self.client;

        let local_uri_str = format!(
            "{}/api/v1/events",
            local_configuration.get_operation_host(operation_id)
        );
        let mut local_req_builder =
            local_client.request(reqwest::Method::GET, local_uri_str.as_str());

        local_req_builder = local_req_builder.query(&[("start", &start.to_string())]);
        local_req_builder = local_req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref local_query_param) = priority {
            local_req_builder =
                local_req_builder.query(&[("priority", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = sources {
            local_req_builder =
                local_req_builder.query(&[("sources", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = tags {
            local_req_builder =
                local_req_builder.query(&[("tags", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = unaggregated {
            local_req_builder =
                local_req_builder.query(&[("unaggregated", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = exclude_aggregate {
            local_req_builder =
                local_req_builder.query(&[("exclude_aggregate", &local_query_param.to_string())]);
        };
        if let Some(ref local_query_param) = page {
            local_req_builder =
                local_req_builder.query(&[("page", &local_query_param.to_string())]);
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
                    HeaderValue::from_static(configuration::DEFAULT_USER_AGENT.as_str()),
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
        let local_resp = local_client.execute(local_req).await?;

        let local_status = local_resp.status();
        let local_content = local_resp.text().await?;

        if !local_status.is_client_error() && !local_status.is_server_error() {
            match serde_json::from_str::<crate::datadogV1::model::EventListResponse>(&local_content)
            {
                Ok(e) => {
                    return Ok(ResponseContent {
                        status: local_status,
                        content: local_content,
                        entity: Some(e),
                    })
                }
                Err(e) => return Err(crate::datadog::Error::Serde(e)),
            };
        } else {
            let local_entity: Option<ListEventsError> = serde_json::from_str(&local_content).ok();
            let local_error = ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            };
            Err(Error::ResponseError(local_error))
        }
    }
}

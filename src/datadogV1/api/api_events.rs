// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use crate::datadog::*;
use reqwest;
use serde::{Deserialize, Serialize};

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
    pub fn priority(&mut self, value: crate::datadogV1::model::EventPriority) -> &mut Self {
        self.priority = Some(value);
        self
    }
    /// A comma separated string of sources.
    pub fn sources(&mut self, value: String) -> &mut Self {
        self.sources = Some(value);
        self
    }
    /// A comma separated list indicating what tags, if any, should be used to filter the list of events.
    pub fn tags(&mut self, value: String) -> &mut Self {
        self.tags = Some(value);
        self
    }
    /// Set unaggregated to `true` to return all events within the specified [`start`,`end`] timeframe.
    /// Otherwise if an event is aggregated to a parent event with a timestamp outside of the timeframe,
    /// it won't be available in the output. Aggregated events with `is_aggregate=true` in the response will still be returned unless exclude_aggregate is set to `true.`
    pub fn unaggregated(&mut self, value: bool) -> &mut Self {
        self.unaggregated = Some(value);
        self
    }
    /// Set `exclude_aggregate` to `true` to only return unaggregated events where `is_aggregate=false` in the response. If the `exclude_aggregate` parameter is set to `true`,
    /// then the unaggregated parameter is ignored and will be `true` by default.
    pub fn exclude_aggregate(&mut self, value: bool) -> &mut Self {
        self.exclude_aggregate = Some(value);
        self
    }
    /// By default 1000 results are returned per request. Set page to the number of the page to return with `0` being the first page. The page parameter can only be used
    /// when either unaggregated or exclude_aggregate is set to `true.`
    pub fn page(&mut self, value: i32) -> &mut Self {
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
}

impl Default for EventsAPI {
    fn default() -> Self {
        Self {
            config: configuration::Configuration::new(),
        }
    }
}

impl EventsAPI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(config: configuration::Configuration) -> Self {
        Self { config }
    }

    /// This endpoint allows you to post events to the stream.
    /// Tag them, set priority and event aggregate them with other events.
    pub async fn create_event(
        &self,
        body: crate::datadogV1::model::EventCreateRequest,
    ) -> Result<Option<crate::datadogV1::model::EventCreateResponse>, Error<CreateEventError>> {
        match self.create_event_with_http_info(body).await {
            Ok(response_content) => Ok(response_content.entity),
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

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/events",
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
            let local_entity: Option<crate::datadogV1::model::EventCreateResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<crate::datadogV1::model::EventResponse>, Error<GetEventError>> {
        match self.get_event_with_http_info(event_id).await {
            Ok(response_content) => Ok(response_content.entity),
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

        let local_client = &local_configuration.client;

        let local_uri_str = format!(
            "{}/api/v1/events/{event_id}",
            local_configuration.get_operation_host(operation_id),
            event_id = event_id
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
            let local_entity: Option<crate::datadogV1::model::EventResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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
    ) -> Result<Option<crate::datadogV1::model::EventListResponse>, Error<ListEventsError>> {
        match self.list_events_with_http_info(start, end, params).await {
            Ok(response_content) => Ok(response_content.entity),
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

        let local_client = &local_configuration.client;

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
            let local_entity: Option<crate::datadogV1::model::EventListResponse> =
                serde_json::from_str(&local_content).ok();
            Ok(ResponseContent {
                status: local_status,
                content: local_content,
                entity: local_entity,
            })
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

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateEventParams is a struct for passing parameters to the method [`CreateEvent`]
#[derive(Clone, Debug)]
pub struct CreateEventParams {
    /* Event request object */
    pub body: EventCreateRequest,
}

// GetEventParams is a struct for passing parameters to the method [`GetEvent`]
#[derive(Clone, Debug)]
pub struct GetEventParams {
    /* The ID of the event. */
    pub event_id: i64,
}

// ListEventsParams is a struct for passing parameters to the method [`ListEvents`]
#[derive(Clone, Debug)]
pub struct ListEventsParams {
    /* POSIX timestamp. */
    pub start: i64,
    /* POSIX timestamp. */
    pub end: i64,
    /* Priority of your events, either `low` or `normal`. */
    pub priority: EventPriority,
    /* A comma separated string of sources. */
    pub sources: String,
    /* A comma separated list indicating what tags, if any, should be used to filter the list of events. */
    pub tags: String,
    /* Set unaggregated to `true` to return all events within the specified [`start`,`end`] timeframe.
Otherwise if an event is aggregated to a parent event with a timestamp outside of the timeframe,
it won't be available in the output. Aggregated events with `is_aggregate=true` in the response will still be returned unless exclude_aggregate is set to `true.` */
    pub unaggregated: bool,
    /* Set `exclude_aggregate` to `true` to only return unaggregated events where `is_aggregate=false` in the response. If the `exclude_aggregate` parameter is set to `true`,
then the unaggregated parameter is ignored and will be `true` by default. */
    pub exclude_aggregate: bool,
    /* By default 1000 results are returned per request. Set page to the number of the page to return with `0` being the first page. The page parameter can only be used
when either unaggregated or exclude_aggregate is set to `true.` */
    pub page: i32,
}




/// CreateEventError is a struct for typed errors of method [`CreateEvent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEventError {
    Status400(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetEventError is a struct for typed errors of method [`GetEvent`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEventError {
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListEventsError is a struct for typed errors of method [`ListEvents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
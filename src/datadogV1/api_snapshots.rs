// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// GetGraphSnapshotParams is a struct for passing parameters to the method [`GetGraphSnapshot`]
#[derive(Clone, Debug)]
pub struct GetGraphSnapshotParams {
    /* The POSIX timestamp of the start of the query in seconds. */
    pub start: i64,
    /* The POSIX timestamp of the end of the query in seconds. */
    pub end: i64,
    /* The metric query. */
    pub metric_query: String,
    /* A query that adds event bands to the graph. */
    pub event_query: String,
    /* A JSON document defining the graph. `graph_def` can be used instead of `metric_query`.
The JSON document uses the [grammar defined here](https://docs.datadoghq.com/graphing/graphing_json/#grammar)
and should be formatted to a single line then URL encoded. */
    pub graph_def: String,
    /* A title for the graph. If no title is specified, the graph does not have a title. */
    pub title: String,
    /* The height of the graph. If no height is specified, the graph's original height is used. */
    pub height: i64,
    /* The width of the graph. If no width is specified, the graph's original width is used. */
    pub width: i64,
}




/// GetGraphSnapshotError is a struct for typed errors of method [`GetGraphSnapshot`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGraphSnapshotError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
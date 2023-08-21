// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// ListProcessesParams is a struct for passing parameters to the method [`ListProcesses`]
#[derive(Clone, Debug)]
pub struct ListProcessesParams {
    /* String to search processes by. */
    pub search: String,
    /* Comma-separated list of tags to filter processes by. */
    pub tags: String,
    /* Unix timestamp (number of seconds since epoch) of the start of the query window.
If not provided, the start of the query window will be 15 minutes before the `to` timestamp. If neither
`from` nor `to` are provided, the query window will be `[now - 15m, now]`. */
    pub from: i64,
    /* Unix timestamp (number of seconds since epoch) of the end of the query window.
If not provided, the end of the query window will be 15 minutes after the `from` timestamp. If neither
`from` nor `to` are provided, the query window will be `[now - 15m, now]`. */
    pub to: i64,
    /* Maximum number of results returned. */
    pub page_limit: i32,
    /* String to query the next page of results.
This key is provided with each valid response from the API in `meta.page.after`. */
    pub page_cursor: String,
}




/// ListProcessesError is a struct for typed errors of method [`ListProcesses`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListProcessesError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
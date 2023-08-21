// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// GetHostTotalsParams is a struct for passing parameters to the method [`GetHostTotals`]
#[derive(Clone, Debug)]
pub struct GetHostTotalsParams {
    /* Number of seconds from which you want to get total number of active hosts. */
    pub from: i64,
}

// ListHostsParams is a struct for passing parameters to the method [`ListHosts`]
#[derive(Clone, Debug)]
pub struct ListHostsParams {
    /* String to filter search results. */
    pub filter: String,
    /* Sort hosts by this field. */
    pub sort_field: String,
    /* Direction of sort. Options include `asc` and `desc`. */
    pub sort_dir: String,
    /* Host result to start search from. */
    pub start: i64,
    /* Number of hosts to return. Max 1000. */
    pub count: i64,
    /* Number of seconds since UNIX epoch from which you want to search your hosts. */
    pub from: i64,
    /* Include information on the muted status of hosts and when the mute expires. */
    pub include_muted_hosts_data: bool,
    /* Include additional metadata about the hosts (agent_version, machine, platform, processor, etc.). */
    pub include_hosts_metadata: bool,
}

// MuteHostParams is a struct for passing parameters to the method [`MuteHost`]
#[derive(Clone, Debug)]
pub struct MuteHostParams {
    /* Name of the host to mute. */
    pub host_name: String,
    /* Mute a host request body. */
    pub body: HostMuteSettings,
}

// UnmuteHostParams is a struct for passing parameters to the method [`UnmuteHost`]
#[derive(Clone, Debug)]
pub struct UnmuteHostParams {
    /* Name of the host to unmute. */
    pub host_name: String,
}




/// GetHostTotalsError is a struct for typed errors of method [`GetHostTotals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHostTotalsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// ListHostsError is a struct for typed errors of method [`ListHosts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHostsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// MuteHostError is a struct for typed errors of method [`MuteHost`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MuteHostError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UnmuteHostError is a struct for typed errors of method [`UnmuteHost`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnmuteHostError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
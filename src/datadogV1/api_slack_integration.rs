// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

// CreateSlackIntegrationChannelParams is a struct for passing parameters to the method [`CreateSlackIntegrationChannel`]
#[derive(Clone, Debug)]
pub struct CreateSlackIntegrationChannelParams {
    /* Your Slack account name. */
    pub account_name: String,
    /* Payload describing Slack channel to be created */
    pub body: SlackIntegrationChannel,
}

// GetSlackIntegrationChannelParams is a struct for passing parameters to the method [`GetSlackIntegrationChannel`]
#[derive(Clone, Debug)]
pub struct GetSlackIntegrationChannelParams {
    /* Your Slack account name. */
    pub account_name: String,
    /* The name of the Slack channel being operated on. */
    pub channel_name: String,
}

// GetSlackIntegrationChannelsParams is a struct for passing parameters to the method [`GetSlackIntegrationChannels`]
#[derive(Clone, Debug)]
pub struct GetSlackIntegrationChannelsParams {
    /* Your Slack account name. */
    pub account_name: String,
}

// RemoveSlackIntegrationChannelParams is a struct for passing parameters to the method [`RemoveSlackIntegrationChannel`]
#[derive(Clone, Debug)]
pub struct RemoveSlackIntegrationChannelParams {
    /* Your Slack account name. */
    pub account_name: String,
    /* The name of the Slack channel being operated on. */
    pub channel_name: String,
}

// UpdateSlackIntegrationChannelParams is a struct for passing parameters to the method [`UpdateSlackIntegrationChannel`]
#[derive(Clone, Debug)]
pub struct UpdateSlackIntegrationChannelParams {
    /* Your Slack account name. */
    pub account_name: String,
    /* The name of the Slack channel being operated on. */
    pub channel_name: String,
    /* Payload describing fields and values to be updated. */
    pub body: SlackIntegrationChannel,
}




/// CreateSlackIntegrationChannelError is a struct for typed errors of method [`CreateSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSlackIntegrationChannelError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelError is a struct for typed errors of method [`GetSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// GetSlackIntegrationChannelsError is a struct for typed errors of method [`GetSlackIntegrationChannels`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSlackIntegrationChannelsError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// RemoveSlackIntegrationChannelError is a struct for typed errors of method [`RemoveSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveSlackIntegrationChannelError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}

/// UpdateSlackIntegrationChannelError is a struct for typed errors of method [`UpdateSlackIntegrationChannel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSlackIntegrationChannelError {
    Status400(APIErrorResponse),
    Status403(APIErrorResponse),
    Status404(APIErrorResponse),
    Status429(APIErrorResponse),
    UnknownValue(serde_json::Value),
}
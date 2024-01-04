// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The JSON:API data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORADeploymentResponseData {
    /// The ID of the received DORA deployment event.
    #[serde(rename = "id")]
    pub id: String,
    /// JSON:API type for DORA deployment events.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DORADeploymentType>,
}

impl DORADeploymentResponseData {
    pub fn new(id: String) -> DORADeploymentResponseData {
        DORADeploymentResponseData { id, type_: None }
    }
}
// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object description of a pipeline event after being processed and stored by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppPipelineEvent {
    /// JSON object containing all event attributes and their associated values.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::CIAppPipelineEventAttributes>>,
    /// Unique ID of the event.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CIAppPipelineEventTypeName>,
}

impl CIAppPipelineEvent {
    pub fn new() -> CIAppPipelineEvent {
        CIAppPipelineEvent {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}
impl Default for CIAppPipelineEvent {
    fn default() -> Self {
        Self::new()
    }
}

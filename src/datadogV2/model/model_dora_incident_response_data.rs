// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response after receiving a DORA incident event.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORAIncidentResponseData {
    /// The ID of the received DORA incident event.
    #[serde(rename = "id")]
    pub id: String,
    /// JSON:API type for DORA incident events.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DORAIncidentType>,
}

impl DORAIncidentResponseData {
    pub fn new(id: String) -> DORAIncidentResponseData {
        DORAIncidentResponseData { id, type_: None }
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::DORAIncidentType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

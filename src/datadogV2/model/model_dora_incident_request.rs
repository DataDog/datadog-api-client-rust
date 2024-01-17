// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request to create a DORA incident event.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DORAIncidentRequest {
    /// The JSON:API data.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::DORAIncidentRequestData>,
}

impl DORAIncidentRequest {
    pub fn new(data: Box<crate::datadogV2::model::DORAIncidentRequestData>) -> DORAIncidentRequest {
        DORAIncidentRequest { data }
    }
}

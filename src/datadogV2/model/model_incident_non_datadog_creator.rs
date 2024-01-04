// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident's non Datadog creator.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentNonDatadogCreator {
    /// Non Datadog creator `48px` image.
    #[serde(rename = "image_48_px")]
    pub image_48_px: Option<String>,
    /// Non Datadog creator name.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

impl IncidentNonDatadogCreator {
    pub fn new() -> IncidentNonDatadogCreator {
        IncidentNonDatadogCreator {
            image_48_px: None,
            name: None,
        }
    }
}
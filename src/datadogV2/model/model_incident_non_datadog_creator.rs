// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident's non Datadog creator.
#[non_exhaustive]
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

    pub fn image_48_px(&mut self, value: String) -> &mut Self {
        self.image_48_px = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for IncidentNonDatadogCreator {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// List of Synthetic locations.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsLocations {
    /// List of Synthetic locations.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::datadogV1::model::SyntheticsLocation>>,
}

impl SyntheticsLocations {
    pub fn new() -> SyntheticsLocations {
        SyntheticsLocations { locations: None }
    }

    pub fn with_locations(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsLocation>,
    ) -> &mut Self {
        self.locations = Some(value);
        self
    }
}
impl Default for SyntheticsLocations {
    fn default() -> Self {
        Self::new()
    }
}

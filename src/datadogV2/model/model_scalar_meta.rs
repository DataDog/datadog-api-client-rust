// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Metadata for the resulting numerical values.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalarMeta {
    /// Detailed information about the unit.
    /// First element describes the "primary unit" (for example, `bytes` in `bytes per second`).
    /// The second element describes the "per unit" (for example, `second` in `bytes per second`).
    /// If the second element is not present, the API returns null.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option")]
    pub unit: Option<Option<Vec<Option<crate::datadogV2::model::Unit>>>>,
}

impl ScalarMeta {
    pub fn new() -> ScalarMeta {
        ScalarMeta { unit: None }
    }

    pub fn unit(mut self, value: Option<Vec<Option<crate::datadogV2::model::Unit>>>) -> Self {
        self.unit = Some(value);
        self
    }
}

impl Default for ScalarMeta {
    fn default() -> Self {
        Self::new()
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpacks are templated groups of dashboard widgets you can save from an existing dashboard and turn into reusable packs in the widget tray.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Powerpack {
    /// Powerpack data object.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::PowerpackData>,
}

impl Powerpack {
    pub fn new() -> Powerpack {
        Powerpack { data: None }
    }

    pub fn with_data(&mut self, value: crate::datadogV2::model::PowerpackData) -> &mut Self {
        self.data = Some(value);
        self
    }
}
impl Default for Powerpack {
    fn default() -> Self {
        Self::new()
    }
}

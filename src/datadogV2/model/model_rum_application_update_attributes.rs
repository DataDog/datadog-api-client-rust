// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application update attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationUpdateAttributes {
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl RUMApplicationUpdateAttributes {
    pub fn new() -> RUMApplicationUpdateAttributes {
        RUMApplicationUpdateAttributes {
            name: None,
            type_: None,
        }
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: String) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for RUMApplicationUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

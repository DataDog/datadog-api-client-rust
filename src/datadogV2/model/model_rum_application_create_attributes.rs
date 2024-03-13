// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// RUM application creation attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RUMApplicationCreateAttributes {
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: String,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl RUMApplicationCreateAttributes {
    pub fn new(name: String) -> RUMApplicationCreateAttributes {
        RUMApplicationCreateAttributes { name, type_: None }
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

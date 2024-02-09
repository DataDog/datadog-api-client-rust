// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Define an expression alias.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesWidgetExpressionAlias {
    /// Expression alias.
    #[serde(rename = "alias_name")]
    pub alias_name: Option<String>,
    /// Expression name.
    #[serde(rename = "expression")]
    pub expression: String,
}

impl TimeseriesWidgetExpressionAlias {
    pub fn new(expression: String) -> TimeseriesWidgetExpressionAlias {
        TimeseriesWidgetExpressionAlias {
            alias_name: None,
            expression,
        }
    }

    pub fn alias_name(&mut self, value: String) -> &mut Self {
        self.alias_name = Some(value);
        self
    }
}

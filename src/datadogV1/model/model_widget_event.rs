// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Event overlay control options.
///
/// See the dedicated [Events JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/widget_json/#events-schema>)
/// to learn how to build the `<EVENTS_SCHEMA>`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetEvent {
    /// Query definition.
    #[serde(rename = "q")]
    pub q: String,
    /// The execution method for multi-value filters.
    #[serde(rename = "tags_execution")]
    pub tags_execution: Option<String>,
}

impl WidgetEvent {
    pub fn new(q: String) -> WidgetEvent {
        WidgetEvent {
            q,
            tags_execution: None,
        }
    }

    pub fn tags_execution(&mut self, value: String) -> &mut Self {
        self.tags_execution = Some(value);
        self
    }
}

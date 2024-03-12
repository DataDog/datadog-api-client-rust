// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Time setting for the widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetTime {
    /// The available timeframes depend on the widget you are using.
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV1::model::WidgetLiveSpan>,
}

impl WidgetTime {
    pub fn new() -> WidgetTime {
        WidgetTime { live_span: None }
    }

    pub fn live_span(mut self, value: crate::datadogV1::model::WidgetLiveSpan) -> Self {
        self.live_span = Some(value);
        self
    }
}

impl Default for WidgetTime {
    fn default() -> Self {
        Self::new()
    }
}

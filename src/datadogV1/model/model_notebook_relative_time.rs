// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relative timeframe.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotebookRelativeTime {
    /// The available timeframes depend on the widget you are using.
    #[serde(rename = "live_span")]
    pub live_span: crate::datadogV1::model::WidgetLiveSpan,
}

impl NotebookRelativeTime {
    pub fn new(live_span: crate::datadogV1::model::WidgetLiveSpan) -> NotebookRelativeTime {
        NotebookRelativeTime { live_span }
    }
}

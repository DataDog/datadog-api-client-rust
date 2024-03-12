// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Powerpack group widget definition object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerpackGroupWidget {
    /// Powerpack group widget object.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV2::model::PowerpackGroupWidgetDefinition,
    /// Powerpack group widget layout.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV2::model::PowerpackGroupWidgetLayout>,
    /// The available timeframes depend on the widget you are using.
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV2::model::WidgetLiveSpan>,
}

impl PowerpackGroupWidget {
    pub fn new(
        definition: crate::datadogV2::model::PowerpackGroupWidgetDefinition,
    ) -> PowerpackGroupWidget {
        PowerpackGroupWidget {
            definition,
            layout: None,
            live_span: None,
        }
    }

    pub fn layout(mut self, value: crate::datadogV2::model::PowerpackGroupWidgetLayout) -> Self {
        self.layout = Some(value);
        self
    }

    pub fn live_span(mut self, value: crate::datadogV2::model::WidgetLiveSpan) -> Self {
        self.live_span = Some(value);
        self
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Set a timeseries on the widget background.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesBackground {
    /// Timeseries is made using an area or bars.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::TimeseriesBackgroundType,
    /// Axis controls for the widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::WidgetAxis>,
}

impl TimeseriesBackground {
    pub fn new(type_: crate::datadogV1::model::TimeseriesBackgroundType) -> TimeseriesBackground {
        TimeseriesBackground { type_, yaxis: None }
    }

    pub fn yaxis(&mut self, value: crate::datadogV1::model::WidgetAxis) -> &mut Self {
        self.yaxis = Some(value);
        self
    }
}

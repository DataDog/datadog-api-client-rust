// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Top list widget stacked display options.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToplistWidgetStacked {
    /// Top list widget stacked legend behavior.
    #[serde(rename = "legend")]
    pub legend: crate::datadogV1::model::ToplistWidgetLegend,
    /// Top list widget stacked display type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::ToplistWidgetStackedType,
}

impl ToplistWidgetStacked {
    pub fn new(
        legend: crate::datadogV1::model::ToplistWidgetLegend,
        type_: crate::datadogV1::model::ToplistWidgetStackedType,
    ) -> ToplistWidgetStacked {
        ToplistWidgetStacked { legend, type_ }
    }
}

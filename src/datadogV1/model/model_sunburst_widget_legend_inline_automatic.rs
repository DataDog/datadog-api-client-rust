// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration of inline or automatic legends.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SunburstWidgetLegendInlineAutomatic {
    /// Whether to hide the percentages of the groups.
    #[serde(rename = "hide_percent")]
    pub hide_percent: Option<bool>,
    /// Whether to hide the values of the groups.
    #[serde(rename = "hide_value")]
    pub hide_value: Option<bool>,
    /// Whether to show the legend inline or let it be automatically generated.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType,
}

impl SunburstWidgetLegendInlineAutomatic {
    pub fn new(
        type_: crate::datadogV1::model::SunburstWidgetLegendInlineAutomaticType,
    ) -> SunburstWidgetLegendInlineAutomatic {
        SunburstWidgetLegendInlineAutomatic {
            hide_percent: None,
            hide_value: None,
            type_,
        }
    }
}

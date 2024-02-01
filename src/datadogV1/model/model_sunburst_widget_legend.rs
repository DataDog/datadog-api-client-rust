// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Configuration of the legend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SunburstWidgetLegend {
    SunburstWidgetLegendTable(Box<crate::datadogV1::model::SunburstWidgetLegendTable>),
    SunburstWidgetLegendInlineAutomatic(
        Box<crate::datadogV1::model::SunburstWidgetLegendInlineAutomatic>,
    ),
}

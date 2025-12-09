// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Bar chart widget display options.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BarChartWidgetDisplay {
    BarChartWidgetStacked(Box<crate::datadogV1::model::BarChartWidgetStacked>),
    BarChartWidgetFlat(Box<crate::datadogV1::model::BarChartWidgetFlat>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for BarChartWidgetDisplay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::BarChartWidgetStacked>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(BarChartWidgetDisplay::BarChartWidgetStacked(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1::model::BarChartWidgetFlat>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(BarChartWidgetDisplay::BarChartWidgetFlat(_v));
            }
        }

        return Ok(BarChartWidgetDisplay::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Configuration of the legend.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SunburstWidgetLegend {
    SunburstWidgetLegendTable(Box<crate::datadogV1::model::SunburstWidgetLegendTable>),
    SunburstWidgetLegendInlineAutomatic(
        Box<crate::datadogV1::model::SunburstWidgetLegendInlineAutomatic>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SunburstWidgetLegend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SunburstWidgetLegendTable>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SunburstWidgetLegend::SunburstWidgetLegendTable(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::SunburstWidgetLegendInlineAutomatic>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SunburstWidgetLegend::SunburstWidgetLegendInlineAutomatic(
                    _v,
                ));
            }
        }

        return Ok(SunburstWidgetLegend::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

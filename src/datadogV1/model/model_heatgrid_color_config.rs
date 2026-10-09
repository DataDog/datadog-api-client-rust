// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Color configuration for continuous gradients or discrete thresholds.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HeatgridColorConfig {
    HeatgridGradientCustomColor(Box<crate::datadogV1::model::HeatgridGradientCustomColor>),
    HeatgridGradientPresetColor(Box<crate::datadogV1::model::HeatgridGradientPresetColor>),
    HeatgridDiscreteCustomColor(Box<crate::datadogV1::model::HeatgridDiscreteCustomColor>),
    HeatgridDiscretePresetColor(Box<crate::datadogV1::model::HeatgridDiscretePresetColor>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for HeatgridColorConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HeatgridGradientCustomColor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(HeatgridColorConfig::HeatgridGradientCustomColor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HeatgridGradientPresetColor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(HeatgridColorConfig::HeatgridGradientPresetColor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HeatgridDiscreteCustomColor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(HeatgridColorConfig::HeatgridDiscreteCustomColor(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::HeatgridDiscretePresetColor>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(HeatgridColorConfig::HeatgridDiscretePresetColor(_v));
            }
        }

        return Ok(HeatgridColorConfig::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

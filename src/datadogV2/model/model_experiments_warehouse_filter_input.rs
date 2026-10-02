// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A property or measure comparison for a Warehouse numerator or denominator. Set exactly one target ID that is not blank. Measure comparisons require numeric measures and numeric values. BETWEEN bounds must be in ascending order.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExperimentsWarehouseFilterInput {
    ExperimentsPropertyFilterInput(Box<crate::datadogV2::model::ExperimentsPropertyFilterInput>),
    ExperimentsPropertyNullFilterInput(
        Box<crate::datadogV2::model::ExperimentsPropertyNullFilterInput>,
    ),
    ExperimentsMeasureComparisonFilterInput(
        Box<crate::datadogV2::model::ExperimentsMeasureComparisonFilterInput>,
    ),
    ExperimentsMeasureRangeFilterInput(
        Box<crate::datadogV2::model::ExperimentsMeasureRangeFilterInput>,
    ),
    ExperimentsMeasureNullFilterInput(
        Box<crate::datadogV2::model::ExperimentsMeasureNullFilterInput>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ExperimentsWarehouseFilterInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsPropertyFilterInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsWarehouseFilterInput::ExperimentsPropertyFilterInput(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsPropertyNullFilterInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsWarehouseFilterInput::ExperimentsPropertyNullFilterInput(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsMeasureComparisonFilterInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    ExperimentsWarehouseFilterInput::ExperimentsMeasureComparisonFilterInput(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsMeasureRangeFilterInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsWarehouseFilterInput::ExperimentsMeasureRangeFilterInput(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsMeasureNullFilterInput>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsWarehouseFilterInput::ExperimentsMeasureNullFilterInput(_v));
            }
        }

        return Ok(ExperimentsWarehouseFilterInput::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

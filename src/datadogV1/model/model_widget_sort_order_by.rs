// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The item to sort the widget by.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WidgetSortOrderBy {
    WidgetFormulaSort(Box<crate::datadogV1::model::WidgetFormulaSort>),
    WidgetGroupSort(Box<crate::datadogV1::model::WidgetGroupSort>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for WidgetSortOrderBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::WidgetFormulaSort>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetSortOrderBy::WidgetFormulaSort(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV1::model::WidgetGroupSort>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(WidgetSortOrderBy::WidgetGroupSort(_v));
            }
        }

        return Ok(WidgetSortOrderBy::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

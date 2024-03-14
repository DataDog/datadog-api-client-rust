// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A single column in a scalar query response.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ScalarColumn {
    GroupScalarColumn(Box<crate::datadogV2::model::GroupScalarColumn>),
    DataScalarColumn(Box<crate::datadogV2::model::DataScalarColumn>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ScalarColumn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::GroupScalarColumn>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarColumn::GroupScalarColumn(_v));
            }
        }
        if let Ok(_v) =
            serde_json::from_value::<Box<crate::datadogV2::model::DataScalarColumn>>(value.clone())
        {
            if !_v._unparsed {
                return Ok(ScalarColumn::DataScalarColumn(_v));
            }
        }

        return Ok(ScalarColumn::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

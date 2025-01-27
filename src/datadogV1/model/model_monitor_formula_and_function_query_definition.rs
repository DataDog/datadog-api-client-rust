// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A formula and function query.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum MonitorFormulaAndFunctionQueryDefinition {
    MonitorFormulaAndFunctionEventQueryDefinition(
        Box<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinition>,
    ),
    MonitorFormulaAndFunctionCostQueryDefinition(
        Box<crate::datadogV1::model::MonitorFormulaAndFunctionCostQueryDefinition>,
    ),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionQueryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::MonitorFormulaAndFunctionEventQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionEventQueryDefinition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::MonitorFormulaAndFunctionCostQueryDefinition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(MonitorFormulaAndFunctionQueryDefinition::MonitorFormulaAndFunctionCostQueryDefinition(_v));
            }
        }

        return Ok(MonitorFormulaAndFunctionQueryDefinition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

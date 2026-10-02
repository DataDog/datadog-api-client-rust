// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// A saved-filter condition or a complete inline condition. The two forms cannot be combined.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExperimentsTargetingRuleCondition {
    ExperimentsSavedFilterCondition(Box<crate::datadogV2::model::ExperimentsSavedFilterCondition>),
    ExperimentsInlineCondition(Box<crate::datadogV2::model::ExperimentsInlineCondition>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for ExperimentsTargetingRuleCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsSavedFilterCondition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsTargetingRuleCondition::ExperimentsSavedFilterCondition(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::ExperimentsInlineCondition>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(ExperimentsTargetingRuleCondition::ExperimentsInlineCondition(_v));
            }
        }

        return Ok(ExperimentsTargetingRuleCondition::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

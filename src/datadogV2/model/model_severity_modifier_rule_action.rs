// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The action to take when a severity modifier rule matches a finding. This is a discriminated union on `type`: `set` assigns a fixed severity, while `shift` moves the severity up or down by one rank.
///
/// A severity modifier rule's `rule.query` must not filter on `@severity` or on the `@severity_details.user_adjusted.*` namespace; use `@severity_details.adjusted.value` to filter on the Datadog-adjusted severity instead.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SeverityModifierRuleAction {
    SeverityModifierRuleSetAction(Box<crate::datadogV2::model::SeverityModifierRuleSetAction>),
    SeverityModifierRuleShiftAction(Box<crate::datadogV2::model::SeverityModifierRuleShiftAction>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for SeverityModifierRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SeverityModifierRuleSetAction>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SeverityModifierRuleAction::SeverityModifierRuleSetAction(
                    _v,
                ));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV2::model::SeverityModifierRuleShiftAction>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(SeverityModifierRuleAction::SeverityModifierRuleShiftAction(
                    _v,
                ));
            }
        }

        return Ok(SeverityModifierRuleAction::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

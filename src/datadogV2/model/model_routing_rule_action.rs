// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// Defines an action that is executed when a routing rule matches certain criteria.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RoutingRuleAction {
    SendSlackMessageAction(Box<crate::datadogV2::model::SendSlackMessageAction>),
    SendTeamsMessageAction(Box<crate::datadogV2::model::SendTeamsMessageAction>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for RoutingRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SendSlackMessageAction>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(RoutingRuleAction::SendSlackMessageAction(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV2::model::SendTeamsMessageAction>>(
            value.clone(),
        ) {
            if !_v._unparsed {
                return Ok(RoutingRuleAction::SendTeamsMessageAction(_v));
            }
        }

        return Ok(RoutingRuleAction::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}

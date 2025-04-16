// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a single escalation step within an escalation policy creation request. Contains assignment strategy, escalation timeout, and a list of targets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyCreateRequestDataAttributesStepsItems {
    /// Specifies how this escalation step will assign targets (example `default` or `round-robin`).
    #[serde(rename = "assignment")]
    pub assignment: Option<
        crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment,
    >,
    /// Defines how many seconds to wait before escalating to the next step.
    #[serde(rename = "escalate_after_seconds")]
    pub escalate_after_seconds: Option<i64>,
    /// Specifies the collection of escalation targets for this step.
    #[serde(rename = "targets")]
    pub targets: Vec<
        crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyCreateRequestDataAttributesStepsItems {
    pub fn new(
        targets: Vec<crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems>,
    ) -> EscalationPolicyCreateRequestDataAttributesStepsItems {
        EscalationPolicyCreateRequestDataAttributesStepsItems {
            assignment: None,
            escalate_after_seconds: None,
            targets,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn assignment(
        mut self,
        value: crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment,
    ) -> Self {
        self.assignment = Some(value);
        self
    }

    pub fn escalate_after_seconds(mut self, value: i64) -> Self {
        self.escalate_after_seconds = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for EscalationPolicyCreateRequestDataAttributesStepsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyCreateRequestDataAttributesStepsItemsVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyCreateRequestDataAttributesStepsItemsVisitor {
            type Value = EscalationPolicyCreateRequestDataAttributesStepsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut assignment: Option<crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment> = None;
                let mut escalate_after_seconds: Option<i64> = None;
                let mut targets: Option<Vec<crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsTargetsItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "assignment" => {
                            if v.is_null() {
                                continue;
                            }
                            assignment = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _assignment) = assignment {
                                match _assignment {
                                    crate::datadogV2::model::EscalationPolicyCreateRequestDataAttributesStepsItemsAssignment::UnparsedObject(_assignment) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "escalate_after_seconds" => {
                            if v.is_null() {
                                continue;
                            }
                            escalate_after_seconds =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targets" => {
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let targets = targets.ok_or_else(|| M::Error::missing_field("targets"))?;

                let content = EscalationPolicyCreateRequestDataAttributesStepsItems {
                    assignment,
                    escalate_after_seconds,
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyCreateRequestDataAttributesStepsItemsVisitor)
    }
}

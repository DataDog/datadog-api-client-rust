// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The action to take when the due date rule matches a finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DueDateRuleAction {
    /// A list of severity-to-due-date mappings. Each severity may appear at most once.
    #[serde(rename = "due_days_per_severity")]
    pub due_days_per_severity: Vec<crate::datadogV2::model::DueDatePerSeverityItem>,
    /// The reference point from which the due date is calculated. When `fix_available` is selected but not applicable to the finding type, `first_seen` is used instead.
    #[serde(rename = "due_from")]
    pub due_from: crate::datadogV2::model::DueDateFrom,
    /// An optional description providing more context for the due date assignment.
    #[serde(rename = "reason_description")]
    pub reason_description: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DueDateRuleAction {
    pub fn new(
        due_days_per_severity: Vec<crate::datadogV2::model::DueDatePerSeverityItem>,
        due_from: crate::datadogV2::model::DueDateFrom,
    ) -> DueDateRuleAction {
        DueDateRuleAction {
            due_days_per_severity,
            due_from,
            reason_description: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reason_description(mut self, value: String) -> Self {
        self.reason_description = Some(value);
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

impl<'de> Deserialize<'de> for DueDateRuleAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DueDateRuleActionVisitor;
        impl<'a> Visitor<'a> for DueDateRuleActionVisitor {
            type Value = DueDateRuleAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut due_days_per_severity: Option<
                    Vec<crate::datadogV2::model::DueDatePerSeverityItem>,
                > = None;
                let mut due_from: Option<crate::datadogV2::model::DueDateFrom> = None;
                let mut reason_description: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "due_days_per_severity" => {
                            due_days_per_severity =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "due_from" => {
                            due_from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _due_from) = due_from {
                                match _due_from {
                                    crate::datadogV2::model::DueDateFrom::UnparsedObject(
                                        _due_from,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "reason_description" => {
                            if v.is_null() {
                                continue;
                            }
                            reason_description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let due_days_per_severity = due_days_per_severity
                    .ok_or_else(|| M::Error::missing_field("due_days_per_severity"))?;
                let due_from = due_from.ok_or_else(|| M::Error::missing_field("due_from"))?;

                let content = DueDateRuleAction {
                    due_days_per_severity,
                    due_from,
                    reason_description,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DueDateRuleActionVisitor)
    }
}

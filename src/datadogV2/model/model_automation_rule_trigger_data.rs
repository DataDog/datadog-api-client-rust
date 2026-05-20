// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Additional configuration for the trigger, dependent on the trigger type. For `status_transitioned` triggers, specify `from_status_name` and `to_status_name`. For `attribute_value_changed` triggers, specify `field` and `change_type`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AutomationRuleTriggerData {
    /// The approval outcome to match. Used with `case_review_approved` triggers.
    #[serde(rename = "approval_type")]
    pub approval_type: Option<String>,
    /// The kind of attribute change to match. Allowed values: `VALUE_ADDED`, `VALUE_DELETED`, `ANY_CHANGES`. Used with `attribute_value_changed` triggers.
    #[serde(rename = "change_type")]
    pub change_type: Option<String>,
    /// The case attribute field name to monitor for changes. Used with `attribute_value_changed` triggers.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The originating status name. Used with `status_transitioned` triggers to match transitions from this status.
    #[serde(rename = "from_status_name")]
    pub from_status_name: Option<String>,
    /// The destination status name. Used with `status_transitioned` triggers to match transitions to this status.
    #[serde(rename = "to_status_name")]
    pub to_status_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AutomationRuleTriggerData {
    pub fn new() -> AutomationRuleTriggerData {
        AutomationRuleTriggerData {
            approval_type: None,
            change_type: None,
            field: None,
            from_status_name: None,
            to_status_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn approval_type(mut self, value: String) -> Self {
        self.approval_type = Some(value);
        self
    }

    pub fn change_type(mut self, value: String) -> Self {
        self.change_type = Some(value);
        self
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn from_status_name(mut self, value: String) -> Self {
        self.from_status_name = Some(value);
        self
    }

    pub fn to_status_name(mut self, value: String) -> Self {
        self.to_status_name = Some(value);
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

impl Default for AutomationRuleTriggerData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AutomationRuleTriggerData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AutomationRuleTriggerDataVisitor;
        impl<'a> Visitor<'a> for AutomationRuleTriggerDataVisitor {
            type Value = AutomationRuleTriggerData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut approval_type: Option<String> = None;
                let mut change_type: Option<String> = None;
                let mut field: Option<String> = None;
                let mut from_status_name: Option<String> = None;
                let mut to_status_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "approval_type" => {
                            if v.is_null() {
                                continue;
                            }
                            approval_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_type" => {
                            if v.is_null() {
                                continue;
                            }
                            change_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from_status_name" => {
                            if v.is_null() {
                                continue;
                            }
                            from_status_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_status_name" => {
                            if v.is_null() {
                                continue;
                            }
                            to_status_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AutomationRuleTriggerData {
                    approval_type,
                    change_type,
                    field,
                    from_status_name,
                    to_status_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AutomationRuleTriggerDataVisitor)
    }
}

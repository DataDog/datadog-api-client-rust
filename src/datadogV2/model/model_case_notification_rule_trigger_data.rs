// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Trigger data
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseNotificationRuleTriggerData {
    /// Change type (added, removed, changed)
    #[serde(rename = "change_type")]
    pub change_type: Option<String>,
    /// Field name for attribute value changed trigger
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// Status ID to transition from
    #[serde(rename = "from_status")]
    pub from_status: Option<String>,
    /// Status name to transition from
    #[serde(rename = "from_status_name")]
    pub from_status_name: Option<String>,
    /// Status ID to transition to
    #[serde(rename = "to_status")]
    pub to_status: Option<String>,
    /// Status name to transition to
    #[serde(rename = "to_status_name")]
    pub to_status_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseNotificationRuleTriggerData {
    pub fn new() -> CaseNotificationRuleTriggerData {
        CaseNotificationRuleTriggerData {
            change_type: None,
            field: None,
            from_status: None,
            from_status_name: None,
            to_status: None,
            to_status_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn change_type(mut self, value: String) -> Self {
        self.change_type = Some(value);
        self
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn from_status(mut self, value: String) -> Self {
        self.from_status = Some(value);
        self
    }

    pub fn from_status_name(mut self, value: String) -> Self {
        self.from_status_name = Some(value);
        self
    }

    pub fn to_status(mut self, value: String) -> Self {
        self.to_status = Some(value);
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

impl Default for CaseNotificationRuleTriggerData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseNotificationRuleTriggerData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseNotificationRuleTriggerDataVisitor;
        impl<'a> Visitor<'a> for CaseNotificationRuleTriggerDataVisitor {
            type Value = CaseNotificationRuleTriggerData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut change_type: Option<String> = None;
                let mut field: Option<String> = None;
                let mut from_status: Option<String> = None;
                let mut from_status_name: Option<String> = None;
                let mut to_status: Option<String> = None;
                let mut to_status_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                        "from_status" => {
                            if v.is_null() {
                                continue;
                            }
                            from_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from_status_name" => {
                            if v.is_null() {
                                continue;
                            }
                            from_status_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_status" => {
                            if v.is_null() {
                                continue;
                            }
                            to_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = CaseNotificationRuleTriggerData {
                    change_type,
                    field,
                    from_status,
                    from_status_name,
                    to_status,
                    to_status_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseNotificationRuleTriggerDataVisitor)
    }
}

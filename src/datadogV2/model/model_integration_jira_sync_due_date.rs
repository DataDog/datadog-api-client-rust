// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationJiraSyncDueDate {
    #[serde(rename = "jira_field_id")]
    pub jira_field_id: Option<String>,
    #[serde(rename = "sync_type")]
    pub sync_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationJiraSyncDueDate {
    pub fn new() -> IntegrationJiraSyncDueDate {
        IntegrationJiraSyncDueDate {
            jira_field_id: None,
            sync_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn jira_field_id(mut self, value: String) -> Self {
        self.jira_field_id = Some(value);
        self
    }

    pub fn sync_type(mut self, value: String) -> Self {
        self.sync_type = Some(value);
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

impl Default for IntegrationJiraSyncDueDate {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationJiraSyncDueDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationJiraSyncDueDateVisitor;
        impl<'a> Visitor<'a> for IntegrationJiraSyncDueDateVisitor {
            type Value = IntegrationJiraSyncDueDate;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut jira_field_id: Option<String> = None;
                let mut sync_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "jira_field_id" => {
                            if v.is_null() {
                                continue;
                            }
                            jira_field_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_type" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationJiraSyncDueDate {
                    jira_field_id,
                    sync_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationJiraSyncDueDateVisitor)
    }
}

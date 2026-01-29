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
pub struct IntegrationServiceNowSyncConfigPriority {
    #[serde(rename = "impact_mapping")]
    pub impact_mapping: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "sync_type")]
    pub sync_type: Option<String>,
    #[serde(rename = "urgency_mapping")]
    pub urgency_mapping: Option<std::collections::BTreeMap<String, String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationServiceNowSyncConfigPriority {
    pub fn new() -> IntegrationServiceNowSyncConfigPriority {
        IntegrationServiceNowSyncConfigPriority {
            impact_mapping: None,
            sync_type: None,
            urgency_mapping: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn impact_mapping(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.impact_mapping = Some(value);
        self
    }

    pub fn sync_type(mut self, value: String) -> Self {
        self.sync_type = Some(value);
        self
    }

    pub fn urgency_mapping(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.urgency_mapping = Some(value);
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

impl Default for IntegrationServiceNowSyncConfigPriority {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationServiceNowSyncConfigPriority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationServiceNowSyncConfigPriorityVisitor;
        impl<'a> Visitor<'a> for IntegrationServiceNowSyncConfigPriorityVisitor {
            type Value = IntegrationServiceNowSyncConfigPriority;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut impact_mapping: Option<std::collections::BTreeMap<String, String>> = None;
                let mut sync_type: Option<String> = None;
                let mut urgency_mapping: Option<std::collections::BTreeMap<String, String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "impact_mapping" => {
                            if v.is_null() {
                                continue;
                            }
                            impact_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sync_type" => {
                            if v.is_null() {
                                continue;
                            }
                            sync_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "urgency_mapping" => {
                            if v.is_null() {
                                continue;
                            }
                            urgency_mapping =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationServiceNowSyncConfigPriority {
                    impact_mapping,
                    sync_type,
                    urgency_mapping,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationServiceNowSyncConfigPriorityVisitor)
    }
}

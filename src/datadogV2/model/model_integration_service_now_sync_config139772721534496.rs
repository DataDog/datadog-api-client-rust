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
pub struct IntegrationServiceNowSyncConfig139772721534496 {
    /// Sync property configuration
    #[serde(rename = "comments")]
    pub comments: Option<crate::datadogV2::model::SyncProperty>,
    #[serde(rename = "priority")]
    pub priority: Option<crate::datadogV2::model::IntegrationServiceNowSyncConfigPriority>,
    /// Sync property with mapping configuration
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::SyncPropertyWithMapping>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationServiceNowSyncConfig139772721534496 {
    pub fn new() -> IntegrationServiceNowSyncConfig139772721534496 {
        IntegrationServiceNowSyncConfig139772721534496 {
            comments: None,
            priority: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn comments(mut self, value: crate::datadogV2::model::SyncProperty) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn priority(
        mut self,
        value: crate::datadogV2::model::IntegrationServiceNowSyncConfigPriority,
    ) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::SyncPropertyWithMapping) -> Self {
        self.status = Some(value);
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

impl Default for IntegrationServiceNowSyncConfig139772721534496 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationServiceNowSyncConfig139772721534496 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationServiceNowSyncConfig139772721534496Visitor;
        impl<'a> Visitor<'a> for IntegrationServiceNowSyncConfig139772721534496Visitor {
            type Value = IntegrationServiceNowSyncConfig139772721534496;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut comments: Option<crate::datadogV2::model::SyncProperty> = None;
                let mut priority: Option<
                    crate::datadogV2::model::IntegrationServiceNowSyncConfigPriority,
                > = None;
                let mut status: Option<crate::datadogV2::model::SyncPropertyWithMapping> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "comments" => {
                            if v.is_null() {
                                continue;
                            }
                            comments = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "priority" => {
                            if v.is_null() {
                                continue;
                            }
                            priority = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationServiceNowSyncConfig139772721534496 {
                    comments,
                    priority,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationServiceNowSyncConfig139772721534496Visitor)
    }
}

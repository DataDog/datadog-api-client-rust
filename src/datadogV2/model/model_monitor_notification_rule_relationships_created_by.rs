// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The user who created the monitor notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorNotificationRuleRelationshipsCreatedBy {
    /// Data for the user who created the monitor notification rule.
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option")]
    pub data:
        Option<Option<crate::datadogV2::model::MonitorNotificationRuleRelationshipsCreatedByData>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorNotificationRuleRelationshipsCreatedBy {
    pub fn new() -> MonitorNotificationRuleRelationshipsCreatedBy {
        MonitorNotificationRuleRelationshipsCreatedBy {
            data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Option<crate::datadogV2::model::MonitorNotificationRuleRelationshipsCreatedByData>,
    ) -> Self {
        self.data = Some(value);
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

impl Default for MonitorNotificationRuleRelationshipsCreatedBy {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorNotificationRuleRelationshipsCreatedBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorNotificationRuleRelationshipsCreatedByVisitor;
        impl<'a> Visitor<'a> for MonitorNotificationRuleRelationshipsCreatedByVisitor {
            type Value = MonitorNotificationRuleRelationshipsCreatedBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Option<
                        crate::datadogV2::model::MonitorNotificationRuleRelationshipsCreatedByData,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = MonitorNotificationRuleRelationshipsCreatedBy {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorNotificationRuleRelationshipsCreatedByVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines a single member within a layer during an update request, referring
/// to a specific user.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
    /// Identifies the user who is assigned to this member object. Only `id` is required.
    #[serde(rename = "user")]
    pub user: Option<
        crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsUser,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
    pub fn new() -> ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
        ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
            user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn user(
        mut self,
        value: crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsUser,
    ) -> Self {
        self.user = Some(value);
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

impl Default for ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsVisitor;
        impl<'a> Visitor<'a> for ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsVisitor {
            type Value = ScheduleUpdateRequestDataAttributesLayersItemsMembersItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut user: Option<crate::datadogV2::model::ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsUser> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "user" => {
                            if v.is_null() {
                                continue;
                            }
                            user = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScheduleUpdateRequestDataAttributesLayersItemsMembersItems {
                    user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ScheduleUpdateRequestDataAttributesLayersItemsMembersItemsVisitor)
    }
}

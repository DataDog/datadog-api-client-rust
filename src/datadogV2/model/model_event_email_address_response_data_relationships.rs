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
pub struct EventEmailAddressResponseDataRelationships {
    #[serde(rename = "created_by")]
    pub created_by: crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser,
    #[serde(rename = "revoked_by")]
    pub revoked_by: Option<crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EventEmailAddressResponseDataRelationships {
    pub fn new(
        created_by: crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser,
    ) -> EventEmailAddressResponseDataRelationships {
        EventEmailAddressResponseDataRelationships {
            created_by,
            revoked_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn revoked_by(
        mut self,
        value: crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser,
    ) -> Self {
        self.revoked_by = Some(value);
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

impl<'de> Deserialize<'de> for EventEmailAddressResponseDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventEmailAddressResponseDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for EventEmailAddressResponseDataRelationshipsVisitor {
            type Value = EventEmailAddressResponseDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by: Option<
                    crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser,
                > = None;
                let mut revoked_by: Option<
                    crate::datadogV2::model::EventEmailAddressResponseDataRelationshipsUser,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revoked_by" => {
                            if v.is_null() {
                                continue;
                            }
                            revoked_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;

                let content = EventEmailAddressResponseDataRelationships {
                    created_by,
                    revoked_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EventEmailAddressResponseDataRelationshipsVisitor)
    }
}

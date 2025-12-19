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
pub struct AttachmentDataRelationships {
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user:
        Option<crate::datadogV2::model::AttachmentDataRelationshipsLastModifiedByUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AttachmentDataRelationships {
    pub fn new() -> AttachmentDataRelationships {
        AttachmentDataRelationships {
            last_modified_by_user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::AttachmentDataRelationshipsLastModifiedByUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
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

impl Default for AttachmentDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AttachmentDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AttachmentDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for AttachmentDataRelationshipsVisitor {
            type Value = AttachmentDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::AttachmentDataRelationshipsLastModifiedByUser,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AttachmentDataRelationships {
                    last_modified_by_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AttachmentDataRelationshipsVisitor)
    }
}

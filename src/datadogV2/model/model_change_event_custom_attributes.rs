// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing custom event attributes required for Change events. The overall object
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEventCustomAttributes {
    /// JSON Object representing the entity which made the change. Currently it only supports
    /// `user` and `system` author type.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::ChangeEventCustomAttributesAuthor>,
    /// Required JSON Object representing a resource. A resource is defined by `type` and `name`. Currently it only
    /// supports `feature_flag` resource type.
    #[serde(rename = "changed_resource")]
    pub changed_resource: crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeEventCustomAttributes {
    pub fn new(
        changed_resource: crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
    ) -> ChangeEventCustomAttributes {
        ChangeEventCustomAttributes {
            author: None,
            changed_resource,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(
        mut self,
        value: crate::datadogV2::model::ChangeEventCustomAttributesAuthor,
    ) -> Self {
        self.author = Some(value);
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

impl<'de> Deserialize<'de> for ChangeEventCustomAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeEventCustomAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeEventCustomAttributesVisitor {
            type Value = ChangeEventCustomAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::ChangeEventCustomAttributesAuthor> =
                    None;
                let mut changed_resource: Option<
                    crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "changed_resource" => {
                            changed_resource =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let changed_resource =
                    changed_resource.ok_or_else(|| M::Error::missing_field("changed_resource"))?;

                let content = ChangeEventCustomAttributes {
                    author,
                    changed_resource,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeEventCustomAttributesVisitor)
    }
}

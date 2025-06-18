// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Change event attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEventCustomAttributes {
    /// The entity that made the change. Optional, if provided it must include `type` and `name`.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::ChangeEventCustomAttributesAuthor>,
    /// Free form JSON object with information related to the `change` event. Supports up to 100 properties per object and a maximum nesting depth of 10 levels.
    #[serde(rename = "change_metadata")]
    pub change_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A uniquely identified resource.
    #[serde(rename = "changed_resource")]
    pub changed_resource: crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
    /// A list of resources impacted by this change. It is recommended to provide an impacted resource to display
    /// the change event at the correct location. Only resources of type `service` are supported. Maximum of 100 impacted resources allowed.
    #[serde(rename = "impacted_resources")]
    pub impacted_resources:
        Option<Vec<crate::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems>>,
    /// Free form JSON object representing the new state of the changed resource.
    #[serde(rename = "new_value")]
    pub new_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Free form JSON object representing the previous state of the changed resource.
    #[serde(rename = "prev_value")]
    pub prev_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
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
            change_metadata: None,
            changed_resource,
            impacted_resources: None,
            new_value: None,
            prev_value: None,
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

    pub fn change_metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.change_metadata = Some(value);
        self
    }

    pub fn impacted_resources(
        mut self,
        value: Vec<crate::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems>,
    ) -> Self {
        self.impacted_resources = Some(value);
        self
    }

    pub fn new_value(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.new_value = Some(value);
        self
    }

    pub fn prev_value(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.prev_value = Some(value);
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
                let mut change_metadata: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut changed_resource: Option<
                    crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
                > = None;
                let mut impacted_resources: Option<
                    Vec<crate::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems>,
                > = None;
                let mut new_value: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut prev_value: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "change_metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            change_metadata =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "changed_resource" => {
                            changed_resource =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "impacted_resources" => {
                            if v.is_null() {
                                continue;
                            }
                            impacted_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "new_value" => {
                            if v.is_null() {
                                continue;
                            }
                            new_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "prev_value" => {
                            if v.is_null() {
                                continue;
                            }
                            prev_value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let changed_resource =
                    changed_resource.ok_or_else(|| M::Error::missing_field("changed_resource"))?;

                let content = ChangeEventCustomAttributes {
                    author,
                    change_metadata,
                    changed_resource,
                    impacted_resources,
                    new_value,
                    prev_value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeEventCustomAttributesVisitor)
    }
}

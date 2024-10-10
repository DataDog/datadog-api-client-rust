// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object representing custom event attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChangeEventCustomAttributes {
    /// Object representing the entity which made the change. Optional field but if provided should include `type` and `name`.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::ChangeEventCustomAttributesAuthor>,
    /// Free form object with any related information of the `change` event.
    #[serde(rename = "change_metadata")]
    pub change_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Object representing a uniquely identified resource. Only the resource type `feature_flag` is supported.
    #[serde(rename = "changed_resource")]
    pub changed_resource: crate::datadogV2::model::ChangeEventCustomAttributesChangedResource,
    /// A list of resources impacted by this change. At least one resource is required. Only resources
    /// of type `service` are supported.
    #[serde(rename = "impacted_resources")]
    pub impacted_resources:
        Option<Vec<crate::datadogV2::model::ChangeEventCustomAttributesImpactedResourcesItems>>,
    /// Free form object to track new value of the changed resource.
    #[serde(rename = "new_value")]
    pub new_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Free form object to track previous value of the changed resource.
    #[serde(rename = "prev_value")]
    pub prev_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
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
            change_metadata: None,
            changed_resource,
            impacted_resources: None,
            new_value: None,
            prev_value: None,
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
                    change_metadata,
                    changed_resource,
                    impacted_resources,
                    new_value,
                    prev_value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeEventCustomAttributesVisitor)
    }
}

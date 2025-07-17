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
pub struct ChangeEventAttributes {
    /// Aggregation key of the event.
    #[serde(rename = "aggregation_key")]
    pub aggregation_key: Option<String>,
    /// The entity that made the change.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::ChangeEventAttributesAuthor>,
    /// JSON object of change metadata.
    #[serde(rename = "change_metadata")]
    pub change_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// A uniquely identified resource.
    #[serde(rename = "changed_resource")]
    pub changed_resource: Option<crate::datadogV2::model::ChangeEventAttributesChangedResource>,
    /// JSON object of event system attributes.
    #[serde(rename = "evt")]
    pub evt: Option<crate::datadogV2::model::EventSystemAttributes>,
    /// A list of resources impacted by this change.
    #[serde(rename = "impacted_resources")]
    pub impacted_resources:
        Option<Vec<crate::datadogV2::model::ChangeEventAttributesImpactedResourcesItem>>,
    /// The new state of the changed resource.
    #[serde(rename = "new_value")]
    pub new_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The previous state of the changed resource.
    #[serde(rename = "prev_value")]
    pub prev_value: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Service that triggered the event.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// POSIX timestamp of the event.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    /// The title of the event.
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ChangeEventAttributes {
    pub fn new() -> ChangeEventAttributes {
        ChangeEventAttributes {
            aggregation_key: None,
            author: None,
            change_metadata: None,
            changed_resource: None,
            evt: None,
            impacted_resources: None,
            new_value: None,
            prev_value: None,
            service: None,
            timestamp: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aggregation_key(mut self, value: String) -> Self {
        self.aggregation_key = Some(value);
        self
    }

    pub fn author(mut self, value: crate::datadogV2::model::ChangeEventAttributesAuthor) -> Self {
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

    pub fn changed_resource(
        mut self,
        value: crate::datadogV2::model::ChangeEventAttributesChangedResource,
    ) -> Self {
        self.changed_resource = Some(value);
        self
    }

    pub fn evt(mut self, value: crate::datadogV2::model::EventSystemAttributes) -> Self {
        self.evt = Some(value);
        self
    }

    pub fn impacted_resources(
        mut self,
        value: Vec<crate::datadogV2::model::ChangeEventAttributesImpactedResourcesItem>,
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

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
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

impl Default for ChangeEventAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ChangeEventAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChangeEventAttributesVisitor;
        impl<'a> Visitor<'a> for ChangeEventAttributesVisitor {
            type Value = ChangeEventAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation_key: Option<String> = None;
                let mut author: Option<crate::datadogV2::model::ChangeEventAttributesAuthor> = None;
                let mut change_metadata: Option<
                    std::collections::BTreeMap<String, serde_json::Value>,
                > = None;
                let mut changed_resource: Option<
                    crate::datadogV2::model::ChangeEventAttributesChangedResource,
                > = None;
                let mut evt: Option<crate::datadogV2::model::EventSystemAttributes> = None;
                let mut impacted_resources: Option<
                    Vec<crate::datadogV2::model::ChangeEventAttributesImpactedResourcesItem>,
                > = None;
                let mut new_value: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut prev_value: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut service: Option<String> = None;
                let mut timestamp: Option<i64> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation_key" => {
                            if v.is_null() {
                                continue;
                            }
                            aggregation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                            if v.is_null() {
                                continue;
                            }
                            changed_resource =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evt" => {
                            if v.is_null() {
                                continue;
                            }
                            evt = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ChangeEventAttributes {
                    aggregation_key,
                    author,
                    change_metadata,
                    changed_resource,
                    evt,
                    impacted_resources,
                    new_value,
                    prev_value,
                    service,
                    timestamp,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ChangeEventAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents key links tied to a monitor to help users take action on alerts.
/// This feature is in Preview and only available to users with the feature enabled.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorAsset {
    /// Indicates the type of asset this entity represents on a monitor.
    #[serde(rename = "category")]
    pub category: crate::datadogV1::model::MonitorAssetCategory,
    /// Name for the monitor asset
    #[serde(rename = "name")]
    pub name: String,
    /// Represents the identifier of the internal Datadog resource that this asset represents. IDs in this field should be passed in as strings.
    #[serde(rename = "resource_key")]
    pub resource_key: Option<String>,
    /// Type of internal Datadog resource associated with a monitor asset.
    #[serde(rename = "resource_type")]
    pub resource_type: Option<crate::datadogV1::model::MonitorAssetResourceType>,
    /// URL link for the asset. For links with an internal resource type set, this should be the relative path to where the Datadog domain is appended internally. For external links, this should be the full URL path.
    #[serde(rename = "url")]
    pub url: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorAsset {
    pub fn new(
        category: crate::datadogV1::model::MonitorAssetCategory,
        name: String,
        url: String,
    ) -> MonitorAsset {
        MonitorAsset {
            category,
            name,
            resource_key: None,
            resource_type: None,
            url,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn resource_key(mut self, value: String) -> Self {
        self.resource_key = Some(value);
        self
    }

    pub fn resource_type(
        mut self,
        value: crate::datadogV1::model::MonitorAssetResourceType,
    ) -> Self {
        self.resource_type = Some(value);
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

impl<'de> Deserialize<'de> for MonitorAsset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorAssetVisitor;
        impl<'a> Visitor<'a> for MonitorAssetVisitor {
            type Value = MonitorAsset;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<crate::datadogV1::model::MonitorAssetCategory> = None;
                let mut name: Option<String> = None;
                let mut resource_key: Option<String> = None;
                let mut resource_type: Option<crate::datadogV1::model::MonitorAssetResourceType> =
                    None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _category) = category {
                                match _category {
                                    crate::datadogV1::model::MonitorAssetCategory::UnparsedObject(_category) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_key" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource_type) = resource_type {
                                match _resource_type {
                                    crate::datadogV1::model::MonitorAssetResourceType::UnparsedObject(_resource_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;

                let content = MonitorAsset {
                    category,
                    name,
                    resource_key,
                    resource_type,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorAssetVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes associated with the custom destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationResponseAttributes {
    /// Whether logs matching this custom destination should be forwarded or not.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Whether tags from the forwarded logs should be forwarded or not.
    #[serde(rename = "forward_tags")]
    pub forward_tags: Option<bool>,
    /// List of [keys of tags](<https://docs.datadoghq.com/getting_started/tagging/#define-tags>) to be filtered.
    ///
    /// An empty list represents no restriction is in place and either all or no tags will be
    /// forwarded depending on `forward_tags_restriction_list_type` parameter.
    #[serde(rename = "forward_tags_restriction_list")]
    pub forward_tags_restriction_list: Option<Vec<String>>,
    /// How `forward_tags_restriction_list` parameter should be interpreted.
    /// If `ALLOW_LIST`, then only tags whose keys on the forwarded logs match the ones on the restriction list
    /// are forwarded.
    ///
    /// `BLOCK_LIST` works the opposite way. It does not forward the tags matching the ones on the list.
    #[serde(rename = "forward_tags_restriction_list_type")]
    pub forward_tags_restriction_list_type:
        Option<crate::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType>,
    /// A custom destination's location to forward logs.
    #[serde(rename = "forwarder_destination")]
    pub forwarder_destination:
        Option<crate::datadogV2::model::CustomDestinationResponseForwardDestination>,
    /// The custom destination name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The custom destination query filter. Logs matching this query are forwarded to the destination.
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationResponseAttributes {
    pub fn new() -> CustomDestinationResponseAttributes {
        CustomDestinationResponseAttributes {
            enabled: None,
            forward_tags: None,
            forward_tags_restriction_list: None,
            forward_tags_restriction_list_type: None,
            forwarder_destination: None,
            name: None,
            query: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn forward_tags(mut self, value: bool) -> Self {
        self.forward_tags = Some(value);
        self
    }

    pub fn forward_tags_restriction_list(mut self, value: Vec<String>) -> Self {
        self.forward_tags_restriction_list = Some(value);
        self
    }

    pub fn forward_tags_restriction_list_type(
        mut self,
        value: crate::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType,
    ) -> Self {
        self.forward_tags_restriction_list_type = Some(value);
        self
    }

    pub fn forwarder_destination(
        mut self,
        value: crate::datadogV2::model::CustomDestinationResponseForwardDestination,
    ) -> Self {
        self.forwarder_destination = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
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

impl Default for CustomDestinationResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomDestinationResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationResponseAttributesVisitor;
        impl<'a> Visitor<'a> for CustomDestinationResponseAttributesVisitor {
            type Value = CustomDestinationResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut forward_tags: Option<bool> = None;
                let mut forward_tags_restriction_list: Option<Vec<String>> = None;
                let mut forward_tags_restriction_list_type: Option<
                    crate::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType,
                > = None;
                let mut forwarder_destination: Option<
                    crate::datadogV2::model::CustomDestinationResponseForwardDestination,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forward_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            forward_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forward_tags_restriction_list" => {
                            if v.is_null() {
                                continue;
                            }
                            forward_tags_restriction_list =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "forward_tags_restriction_list_type" => {
                            if v.is_null() {
                                continue;
                            }
                            forward_tags_restriction_list_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _forward_tags_restriction_list_type) =
                                forward_tags_restriction_list_type
                            {
                                match _forward_tags_restriction_list_type {
                                    crate::datadogV2::model::CustomDestinationAttributeTagsRestrictionListType::UnparsedObject(_forward_tags_restriction_list_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "forwarder_destination" => {
                            if v.is_null() {
                                continue;
                            }
                            forwarder_destination =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _forwarder_destination) = forwarder_destination {
                                match _forwarder_destination {
                                    crate::datadogV2::model::CustomDestinationResponseForwardDestination::UnparsedObject(_forwarder_destination) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CustomDestinationResponseAttributes {
                    enabled,
                    forward_tags,
                    forward_tags_restriction_list,
                    forward_tags_restriction_list_type,
                    forwarder_destination,
                    name,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationResponseAttributesVisitor)
    }
}

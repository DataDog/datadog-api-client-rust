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
pub struct CustomDestinationUpdateRequestAttributes {
    /// Whether logs matching this custom destination should be forwarded or not.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Whether tags from the forwarded logs should be forwarded or not.
    #[serde(rename = "forward_tags")]
    pub forward_tags: Option<bool>,
    /// List of [keys of tags](<https://docs.datadoghq.com/getting_started/tagging/#define-tags>) to be restricted from being forwarded.
    /// An empty list represents no restriction is in place and either all or no tags will be forwarded depending on `forward_tags_restriction_list_type` parameter.
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
    pub forwarder_destination: Option<crate::datadogV2::model::CustomDestinationForwardDestination>,
    /// The custom destination name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The custom destination query and filter. Logs matching this query are forwarded to the destination.
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationUpdateRequestAttributes {
    pub fn new() -> CustomDestinationUpdateRequestAttributes {
        CustomDestinationUpdateRequestAttributes {
            enabled: None,
            forward_tags: None,
            forward_tags_restriction_list: None,
            forward_tags_restriction_list_type: None,
            forwarder_destination: None,
            name: None,
            query: None,
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
        value: crate::datadogV2::model::CustomDestinationForwardDestination,
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
}

impl Default for CustomDestinationUpdateRequestAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CustomDestinationUpdateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationUpdateRequestAttributesVisitor;
        impl<'a> Visitor<'a> for CustomDestinationUpdateRequestAttributesVisitor {
            type Value = CustomDestinationUpdateRequestAttributes;

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
                    crate::datadogV2::model::CustomDestinationForwardDestination,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
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
                                    crate::datadogV2::model::CustomDestinationForwardDestination::UnparsedObject(_forwarder_destination) => {
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
                        &_ => {}
                    }
                }

                let content = CustomDestinationUpdateRequestAttributes {
                    enabled,
                    forward_tags,
                    forward_tags_restriction_list,
                    forward_tags_restriction_list_type,
                    forwarder_destination,
                    name,
                    query,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationUpdateRequestAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object for creating a single WAF exclusion filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafExclusionFilterCreateData {
    /// Attributes for creating a WAF exclusion filter.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateAttributes,
    /// Type of the resource. The value should always be `exclusion_filter`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafExclusionFilterCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateAttributes,
        type_: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterType,
    ) -> ApplicationSecurityWafExclusionFilterCreateData {
        ApplicationSecurityWafExclusionFilterCreateData {
            attributes,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafExclusionFilterCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafExclusionFilterCreateDataVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafExclusionFilterCreateDataVisitor {
            type Value = ApplicationSecurityWafExclusionFilterCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterCreateAttributes,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ApplicationSecurityWafExclusionFilterCreateData {
                    attributes,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafExclusionFilterCreateDataVisitor)
    }
}

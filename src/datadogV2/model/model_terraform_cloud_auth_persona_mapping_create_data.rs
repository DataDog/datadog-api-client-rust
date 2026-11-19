// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data for creating an HCP Terraform persona mapping.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TerraformCloudAuthPersonaMappingCreateData {
    /// Attributes for creating an HCP Terraform persona mapping.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::TerraformCloudAuthPersonaMappingCreateAttributes,
    /// Type identifier for an HCP Terraform persona mapping.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::TerraformCloudAuthPersonaMappingType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TerraformCloudAuthPersonaMappingCreateData {
    pub fn new(
        attributes: crate::datadogV2::model::TerraformCloudAuthPersonaMappingCreateAttributes,
        type_: crate::datadogV2::model::TerraformCloudAuthPersonaMappingType,
    ) -> TerraformCloudAuthPersonaMappingCreateData {
        TerraformCloudAuthPersonaMappingCreateData {
            attributes,
            type_,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for TerraformCloudAuthPersonaMappingCreateData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TerraformCloudAuthPersonaMappingCreateDataVisitor;
        impl<'a> Visitor<'a> for TerraformCloudAuthPersonaMappingCreateDataVisitor {
            type Value = TerraformCloudAuthPersonaMappingCreateData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::TerraformCloudAuthPersonaMappingCreateAttributes,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::TerraformCloudAuthPersonaMappingType,
                > = None;
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
                                    crate::datadogV2::model::TerraformCloudAuthPersonaMappingType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = TerraformCloudAuthPersonaMappingCreateData {
                    attributes,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TerraformCloudAuthPersonaMappingCreateDataVisitor)
    }
}

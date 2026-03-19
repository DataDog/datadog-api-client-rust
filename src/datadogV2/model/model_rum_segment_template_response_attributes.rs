// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a segment template in a response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentTemplateResponseAttributes {
    /// The category of the template.
    #[serde(rename = "category")]
    pub category: String,
    /// The creation timestamp in RFC 3339 format.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// A description of the template.
    #[serde(rename = "description")]
    pub description: String,
    /// The last modification timestamp in RFC 3339 format.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The name of the template.
    #[serde(rename = "name")]
    pub name: String,
    /// The template parameter definitions.
    #[serde(rename = "parameters")]
    pub parameters:
        std::collections::BTreeMap<String, crate::datadogV2::model::RumSegmentTemplateParameterDef>,
    /// The status of a segment template.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::RumSegmentTemplateStatus,
    /// The version number of the template.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentTemplateResponseAttributes {
    pub fn new(
        category: String,
        created_at: chrono::DateTime<chrono::Utc>,
        description: String,
        modified_at: chrono::DateTime<chrono::Utc>,
        name: String,
        parameters: std::collections::BTreeMap<
            String,
            crate::datadogV2::model::RumSegmentTemplateParameterDef,
        >,
        status: crate::datadogV2::model::RumSegmentTemplateStatus,
        version: i64,
    ) -> RumSegmentTemplateResponseAttributes {
        RumSegmentTemplateResponseAttributes {
            category,
            created_at,
            description,
            modified_at,
            name,
            parameters,
            status,
            version,
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

impl<'de> Deserialize<'de> for RumSegmentTemplateResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentTemplateResponseAttributesVisitor;
        impl<'a> Visitor<'a> for RumSegmentTemplateResponseAttributesVisitor {
            type Value = RumSegmentTemplateResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut category: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut description: Option<String> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut parameters: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::RumSegmentTemplateParameterDef,
                    >,
                > = None;
                let mut status: Option<crate::datadogV2::model::RumSegmentTemplateStatus> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "category" => {
                            category = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "parameters" => {
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::RumSegmentTemplateStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let category = category.ok_or_else(|| M::Error::missing_field("category"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let parameters = parameters.ok_or_else(|| M::Error::missing_field("parameters"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = RumSegmentTemplateResponseAttributes {
                    category,
                    created_at,
                    description,
                    modified_at,
                    name,
                    parameters,
                    status,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentTemplateResponseAttributesVisitor)
    }
}

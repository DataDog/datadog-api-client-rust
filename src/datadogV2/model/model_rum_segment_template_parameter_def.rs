// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A parameter definition for a segment template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentTemplateParameterDef {
    /// The default value for the parameter.
    #[serde(rename = "default")]
    pub default: String,
    /// A description of the parameter.
    #[serde(rename = "description")]
    pub description: String,
    /// Validation rules for the parameter.
    #[serde(rename = "validate")]
    pub validate: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentTemplateParameterDef {
    pub fn new(
        default: String,
        description: String,
        validate: String,
    ) -> RumSegmentTemplateParameterDef {
        RumSegmentTemplateParameterDef {
            default,
            description,
            validate,
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

impl<'de> Deserialize<'de> for RumSegmentTemplateParameterDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentTemplateParameterDefVisitor;
        impl<'a> Visitor<'a> for RumSegmentTemplateParameterDefVisitor {
            type Value = RumSegmentTemplateParameterDef;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default: Option<String> = None;
                let mut description: Option<String> = None;
                let mut validate: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default" => {
                            default = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "validate" => {
                            validate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let default = default.ok_or_else(|| M::Error::missing_field("default"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let validate = validate.ok_or_else(|| M::Error::missing_field("validate"))?;

                let content = RumSegmentTemplateParameterDef {
                    default,
                    description,
                    validate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentTemplateParameterDefVisitor)
    }
}

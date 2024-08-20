// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Composed target for `validatesJSONSchema` operator.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsAssertionJSONSchemaTargetTarget {
    /// The JSON Schema to assert.
    #[serde(rename = "jsonSchema")]
    pub json_schema: Option<String>,
    /// The JSON Schema meta-schema version used in the assertion.
    #[serde(rename = "metaSchema")]
    pub meta_schema: Option<crate::datadogV1::model::SyntheticsAssertionJSONSchemaMetaSchema>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsAssertionJSONSchemaTargetTarget {
    pub fn new() -> SyntheticsAssertionJSONSchemaTargetTarget {
        SyntheticsAssertionJSONSchemaTargetTarget {
            json_schema: None,
            meta_schema: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn json_schema(mut self, value: String) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn meta_schema(
        mut self,
        value: crate::datadogV1::model::SyntheticsAssertionJSONSchemaMetaSchema,
    ) -> Self {
        self.meta_schema = Some(value);
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

impl Default for SyntheticsAssertionJSONSchemaTargetTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsAssertionJSONSchemaTargetTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsAssertionJSONSchemaTargetTargetVisitor;
        impl<'a> Visitor<'a> for SyntheticsAssertionJSONSchemaTargetTargetVisitor {
            type Value = SyntheticsAssertionJSONSchemaTargetTarget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut json_schema: Option<String> = None;
                let mut meta_schema: Option<
                    crate::datadogV1::model::SyntheticsAssertionJSONSchemaMetaSchema,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "jsonSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            json_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metaSchema" => {
                            if v.is_null() {
                                continue;
                            }
                            meta_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _meta_schema) = meta_schema {
                                match _meta_schema {
                                    crate::datadogV1::model::SyntheticsAssertionJSONSchemaMetaSchema::UnparsedObject(_meta_schema) => {
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

                let content = SyntheticsAssertionJSONSchemaTargetTarget {
                    json_schema,
                    meta_schema,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsAssertionJSONSchemaTargetTargetVisitor)
    }
}

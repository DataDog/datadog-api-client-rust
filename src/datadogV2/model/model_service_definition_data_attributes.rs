// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Service definition attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceDefinitionDataAttributes {
    /// Metadata about a service definition.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ServiceDefinitionMeta>,
    /// Service definition schema.
    #[serde(rename = "schema")]
    pub schema: Option<crate::datadogV2::model::ServiceDefinitionSchema>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ServiceDefinitionDataAttributes {
    pub fn new() -> ServiceDefinitionDataAttributes {
        ServiceDefinitionDataAttributes {
            meta: None,
            schema: None,
            _unparsed: false,
        }
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::ServiceDefinitionMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }

    pub fn schema(&mut self, value: crate::datadogV2::model::ServiceDefinitionSchema) -> &mut Self {
        self.schema = Some(value);
        self
    }
}

impl Default for ServiceDefinitionDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ServiceDefinitionDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceDefinitionDataAttributesVisitor;
        impl<'a> Visitor<'a> for ServiceDefinitionDataAttributesVisitor {
            type Value = ServiceDefinitionDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut meta: Option<crate::datadogV2::model::ServiceDefinitionMeta> = None;
                let mut schema: Option<crate::datadogV2::model::ServiceDefinitionSchema> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _schema) = schema {
                                match _schema {
                                    crate::datadogV2::model::ServiceDefinitionSchema::UnparsedObject(_schema) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = ServiceDefinitionDataAttributes {
                    meta,
                    schema,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceDefinitionDataAttributesVisitor)
    }
}

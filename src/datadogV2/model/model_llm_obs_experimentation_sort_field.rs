// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A field and direction to sort results by.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LLMObsExperimentationSortField {
    /// Sort direction.
    #[serde(rename = "direction")]
    pub direction: Option<crate::datadogV2::model::LLMObsExperimentationSortFieldDirection>,
    /// The field name to sort on.
    #[serde(rename = "field")]
    pub field: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LLMObsExperimentationSortField {
    pub fn new(field: String) -> LLMObsExperimentationSortField {
        LLMObsExperimentationSortField {
            direction: None,
            field,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn direction(
        mut self,
        value: crate::datadogV2::model::LLMObsExperimentationSortFieldDirection,
    ) -> Self {
        self.direction = Some(value);
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

impl<'de> Deserialize<'de> for LLMObsExperimentationSortField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LLMObsExperimentationSortFieldVisitor;
        impl<'a> Visitor<'a> for LLMObsExperimentationSortFieldVisitor {
            type Value = LLMObsExperimentationSortField;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut direction: Option<
                    crate::datadogV2::model::LLMObsExperimentationSortFieldDirection,
                > = None;
                let mut field: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "direction" => {
                            if v.is_null() {
                                continue;
                            }
                            direction = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _direction) = direction {
                                match _direction {
                                    crate::datadogV2::model::LLMObsExperimentationSortFieldDirection::UnparsedObject(_direction) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "field" => {
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let field = field.ok_or_else(|| M::Error::missing_field("field"))?;

                let content = LLMObsExperimentationSortField {
                    direction,
                    field,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LLMObsExperimentationSortFieldVisitor)
    }
}

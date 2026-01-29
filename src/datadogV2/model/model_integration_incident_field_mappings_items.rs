// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IntegrationIncidentFieldMappingsItems {
    #[serde(rename = "case_field")]
    pub case_field: Option<String>,
    #[serde(rename = "incident_user_defined_field_id")]
    pub incident_user_defined_field_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IntegrationIncidentFieldMappingsItems {
    pub fn new() -> IntegrationIncidentFieldMappingsItems {
        IntegrationIncidentFieldMappingsItems {
            case_field: None,
            incident_user_defined_field_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn case_field(mut self, value: String) -> Self {
        self.case_field = Some(value);
        self
    }

    pub fn incident_user_defined_field_id(mut self, value: String) -> Self {
        self.incident_user_defined_field_id = Some(value);
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

impl Default for IntegrationIncidentFieldMappingsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IntegrationIncidentFieldMappingsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntegrationIncidentFieldMappingsItemsVisitor;
        impl<'a> Visitor<'a> for IntegrationIncidentFieldMappingsItemsVisitor {
            type Value = IntegrationIncidentFieldMappingsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case_field: Option<String> = None;
                let mut incident_user_defined_field_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "case_field" => {
                            if v.is_null() {
                                continue;
                            }
                            case_field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "incident_user_defined_field_id" => {
                            if v.is_null() {
                                continue;
                            }
                            incident_user_defined_field_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IntegrationIncidentFieldMappingsItems {
                    case_field,
                    incident_user_defined_field_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IntegrationIncidentFieldMappingsItemsVisitor)
    }
}

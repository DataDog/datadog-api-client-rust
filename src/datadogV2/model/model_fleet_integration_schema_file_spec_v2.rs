// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A configuration file specification for an integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FleetIntegrationSchemaFileSpecV2 {
    /// The name of the example configuration file.
    #[serde(rename = "example_name")]
    pub example_name: String,
    /// The name of the configuration file.
    #[serde(rename = "name")]
    pub name: String,
    /// The configuration options declared in the file.
    #[serde(rename = "options")]
    pub options: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FleetIntegrationSchemaFileSpecV2 {
    pub fn new(
        example_name: String,
        name: String,
        options: Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>,
    ) -> FleetIntegrationSchemaFileSpecV2 {
        FleetIntegrationSchemaFileSpecV2 {
            example_name,
            name,
            options,
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

impl<'de> Deserialize<'de> for FleetIntegrationSchemaFileSpecV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FleetIntegrationSchemaFileSpecV2Visitor;
        impl<'a> Visitor<'a> for FleetIntegrationSchemaFileSpecV2Visitor {
            type Value = FleetIntegrationSchemaFileSpecV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut example_name: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<
                    Vec<crate::datadogV2::model::FleetIntegrationSchemaSpecOptionV2>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "example_name" => {
                            example_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let example_name =
                    example_name.ok_or_else(|| M::Error::missing_field("example_name"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let options = options.ok_or_else(|| M::Error::missing_field("options"))?;

                let content = FleetIntegrationSchemaFileSpecV2 {
                    example_name,
                    name,
                    options,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FleetIntegrationSchemaFileSpecV2Visitor)
    }
}

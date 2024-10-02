// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Initial application arguments for a mobile test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsMobileTestInitialApplicationArguments {
    /// Name of the property.
    #[serde(rename = "propertyNames")]
    pub property_names: Option<
        crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArgumentsPropertyNames,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsMobileTestInitialApplicationArguments {
    pub fn new() -> SyntheticsMobileTestInitialApplicationArguments {
        SyntheticsMobileTestInitialApplicationArguments {
            property_names: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn property_names(
        mut self,
        value: crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArgumentsPropertyNames,
    ) -> Self {
        self.property_names = Some(value);
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

impl Default for SyntheticsMobileTestInitialApplicationArguments {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsMobileTestInitialApplicationArguments {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsMobileTestInitialApplicationArgumentsVisitor;
        impl<'a> Visitor<'a> for SyntheticsMobileTestInitialApplicationArgumentsVisitor {
            type Value = SyntheticsMobileTestInitialApplicationArguments;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut property_names: Option<crate::datadogV1::model::SyntheticsMobileTestInitialApplicationArgumentsPropertyNames> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "propertyNames" => {
                            if v.is_null() {
                                continue;
                            }
                            property_names =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsMobileTestInitialApplicationArguments {
                    property_names,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsMobileTestInitialApplicationArgumentsVisitor)
    }
}

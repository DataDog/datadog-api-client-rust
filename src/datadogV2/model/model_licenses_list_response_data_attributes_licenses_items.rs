// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An SPDX license entry returned by the licenses list endpoint.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LicensesListResponseDataAttributesLicensesItems {
    /// The human-readable name of the license.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// The SPDX identifier of the license.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// The short name of the license, typically matching the SPDX identifier.
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LicensesListResponseDataAttributesLicensesItems {
    pub fn new(
        display_name: String,
        identifier: String,
        short_name: String,
    ) -> LicensesListResponseDataAttributesLicensesItems {
        LicensesListResponseDataAttributesLicensesItems {
            display_name,
            identifier,
            short_name,
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

impl<'de> Deserialize<'de> for LicensesListResponseDataAttributesLicensesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LicensesListResponseDataAttributesLicensesItemsVisitor;
        impl<'a> Visitor<'a> for LicensesListResponseDataAttributesLicensesItemsVisitor {
            type Value = LicensesListResponseDataAttributesLicensesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut display_name: Option<String> = None;
                let mut identifier: Option<String> = None;
                let mut short_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "display_name" => {
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "identifier" => {
                            identifier = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "short_name" => {
                            short_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let display_name =
                    display_name.ok_or_else(|| M::Error::missing_field("display_name"))?;
                let identifier = identifier.ok_or_else(|| M::Error::missing_field("identifier"))?;
                let short_name = short_name.ok_or_else(|| M::Error::missing_field("short_name"))?;

                let content = LicensesListResponseDataAttributesLicensesItems {
                    display_name,
                    identifier,
                    short_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LicensesListResponseDataAttributesLicensesItemsVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the licenses list response, containing the array of SPDX licenses.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LicensesListResponseDataAttributes {
    /// The list of SPDX licenses returned by the API.
    #[serde(rename = "licenses")]
    pub licenses: Vec<crate::datadogV2::model::LicensesListResponseDataAttributesLicensesItems>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LicensesListResponseDataAttributes {
    pub fn new(
        licenses: Vec<crate::datadogV2::model::LicensesListResponseDataAttributesLicensesItems>,
    ) -> LicensesListResponseDataAttributes {
        LicensesListResponseDataAttributes {
            licenses,
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

impl<'de> Deserialize<'de> for LicensesListResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LicensesListResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for LicensesListResponseDataAttributesVisitor {
            type Value = LicensesListResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut licenses: Option<
                    Vec<crate::datadogV2::model::LicensesListResponseDataAttributesLicensesItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "licenses" => {
                            licenses = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let licenses = licenses.ok_or_else(|| M::Error::missing_field("licenses"))?;

                let content = LicensesListResponseDataAttributes {
                    licenses,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LicensesListResponseDataAttributesVisitor)
    }
}

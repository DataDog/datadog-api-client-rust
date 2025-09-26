// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `AzureScanOptionsInputUpdateDataAttributes` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AzureScanOptionsInputUpdateDataAttributes {
    /// The `attributes` `vuln_containers_os`.
    #[serde(rename = "vuln_containers_os")]
    pub vuln_containers_os: Option<bool>,
    /// The `attributes` `vuln_host_os`.
    #[serde(rename = "vuln_host_os")]
    pub vuln_host_os: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AzureScanOptionsInputUpdateDataAttributes {
    pub fn new() -> AzureScanOptionsInputUpdateDataAttributes {
        AzureScanOptionsInputUpdateDataAttributes {
            vuln_containers_os: None,
            vuln_host_os: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn vuln_containers_os(mut self, value: bool) -> Self {
        self.vuln_containers_os = Some(value);
        self
    }

    pub fn vuln_host_os(mut self, value: bool) -> Self {
        self.vuln_host_os = Some(value);
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

impl Default for AzureScanOptionsInputUpdateDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AzureScanOptionsInputUpdateDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AzureScanOptionsInputUpdateDataAttributesVisitor;
        impl<'a> Visitor<'a> for AzureScanOptionsInputUpdateDataAttributesVisitor {
            type Value = AzureScanOptionsInputUpdateDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut vuln_containers_os: Option<bool> = None;
                let mut vuln_host_os: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "vuln_containers_os" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_containers_os =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_host_os" => {
                            if v.is_null() {
                                continue;
                            }
                            vuln_host_os =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AzureScanOptionsInputUpdateDataAttributes {
                    vuln_containers_os,
                    vuln_host_os,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AzureScanOptionsInputUpdateDataAttributesVisitor)
    }
}

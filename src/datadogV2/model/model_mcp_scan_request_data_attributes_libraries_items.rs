// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A library declaration to include in the dependency scan.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct McpScanRequestDataAttributesLibrariesItems {
    /// The list of dependency PURLs to exclude when resolving transitive dependencies for this library.
    #[serde(rename = "exclusions")]
    pub exclusions: Option<Vec<String>>,
    /// Whether this library is a development-only dependency.
    #[serde(rename = "is_dev")]
    pub is_dev: bool,
    /// Whether this library is a direct (rather than transitive) dependency.
    #[serde(rename = "is_direct")]
    pub is_direct: bool,
    /// The package manager that produced this library entry (for example, `npm`, `pip`, `nuget`).
    #[serde(rename = "package_manager")]
    pub package_manager: String,
    /// The Package URL (PURL) uniquely identifying the library and its version.
    #[serde(rename = "purl")]
    pub purl: String,
    /// The list of target framework identifiers associated with the library.
    #[serde(rename = "target_frameworks")]
    pub target_frameworks: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl McpScanRequestDataAttributesLibrariesItems {
    pub fn new(
        is_dev: bool,
        is_direct: bool,
        package_manager: String,
        purl: String,
    ) -> McpScanRequestDataAttributesLibrariesItems {
        McpScanRequestDataAttributesLibrariesItems {
            exclusions: None,
            is_dev,
            is_direct,
            package_manager,
            purl,
            target_frameworks: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclusions(mut self, value: Vec<String>) -> Self {
        self.exclusions = Some(value);
        self
    }

    pub fn target_frameworks(mut self, value: Vec<String>) -> Self {
        self.target_frameworks = Some(value);
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

impl<'de> Deserialize<'de> for McpScanRequestDataAttributesLibrariesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct McpScanRequestDataAttributesLibrariesItemsVisitor;
        impl<'a> Visitor<'a> for McpScanRequestDataAttributesLibrariesItemsVisitor {
            type Value = McpScanRequestDataAttributesLibrariesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclusions: Option<Vec<String>> = None;
                let mut is_dev: Option<bool> = None;
                let mut is_direct: Option<bool> = None;
                let mut package_manager: Option<String> = None;
                let mut purl: Option<String> = None;
                let mut target_frameworks: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclusions" => {
                            if v.is_null() {
                                continue;
                            }
                            exclusions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_dev" => {
                            is_dev = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_direct" => {
                            is_direct = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_manager" => {
                            package_manager =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "purl" => {
                            purl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_frameworks" => {
                            if v.is_null() {
                                continue;
                            }
                            target_frameworks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_dev = is_dev.ok_or_else(|| M::Error::missing_field("is_dev"))?;
                let is_direct = is_direct.ok_or_else(|| M::Error::missing_field("is_direct"))?;
                let package_manager =
                    package_manager.ok_or_else(|| M::Error::missing_field("package_manager"))?;
                let purl = purl.ok_or_else(|| M::Error::missing_field("purl"))?;

                let content = McpScanRequestDataAttributesLibrariesItems {
                    exclusions,
                    is_dev,
                    is_direct,
                    package_manager,
                    purl,
                    target_frameworks,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(McpScanRequestDataAttributesLibrariesItemsVisitor)
    }
}

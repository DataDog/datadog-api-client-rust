// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of an MCP SCA scan request, describing the libraries to scan and their context.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct McpScanRequestDataAttributes {
    /// The commit hash of the source code being scanned.
    #[serde(rename = "commit_hash")]
    pub commit_hash: String,
    /// The list of libraries to scan for vulnerabilities.
    #[serde(rename = "libraries")]
    pub libraries: Vec<crate::datadogV2::model::McpScanRequestDataAttributesLibrariesItems>,
    /// The name of the resource (typically the repository or project name) being scanned.
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl McpScanRequestDataAttributes {
    pub fn new(
        commit_hash: String,
        libraries: Vec<crate::datadogV2::model::McpScanRequestDataAttributesLibrariesItems>,
        resource_name: String,
    ) -> McpScanRequestDataAttributes {
        McpScanRequestDataAttributes {
            commit_hash,
            libraries,
            resource_name,
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

impl<'de> Deserialize<'de> for McpScanRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct McpScanRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for McpScanRequestDataAttributesVisitor {
            type Value = McpScanRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commit_hash: Option<String> = None;
                let mut libraries: Option<
                    Vec<crate::datadogV2::model::McpScanRequestDataAttributesLibrariesItems>,
                > = None;
                let mut resource_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commit_hash" => {
                            commit_hash =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "libraries" => {
                            libraries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let commit_hash =
                    commit_hash.ok_or_else(|| M::Error::missing_field("commit_hash"))?;
                let libraries = libraries.ok_or_else(|| M::Error::missing_field("libraries"))?;
                let resource_name =
                    resource_name.ok_or_else(|| M::Error::missing_field("resource_name"))?;

                let content = McpScanRequestDataAttributes {
                    commit_hash,
                    libraries,
                    resource_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(McpScanRequestDataAttributesVisitor)
    }
}

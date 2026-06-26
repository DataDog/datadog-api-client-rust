// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A CycloneDX 1.5 Bill of Materials (BOM) document containing vulnerability data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CycloneDXBom {
    /// The BOM format identifier. Must be `CycloneDX`.
    #[serde(rename = "bomFormat")]
    pub bom_format: String,
    /// The list of scanned software components. Cannot be empty.
    #[serde(rename = "components")]
    pub components: Vec<crate::datadogV2::model::CycloneDXComponent>,
    /// Metadata about the BOM, including the scanned asset and the scanner tool.
    #[serde(rename = "metadata")]
    pub metadata: crate::datadogV2::model::CycloneDXMetadata,
    /// The CycloneDX specification version. Must be `1.5`.
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    /// The version number of the BOM document.
    #[serde(rename = "version")]
    pub version: Option<i64>,
    /// The list of detected vulnerabilities. Cannot be empty.
    #[serde(rename = "vulnerabilities")]
    pub vulnerabilities: Vec<crate::datadogV2::model::CycloneDXVulnerability>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CycloneDXBom {
    pub fn new(
        bom_format: String,
        components: Vec<crate::datadogV2::model::CycloneDXComponent>,
        metadata: crate::datadogV2::model::CycloneDXMetadata,
        spec_version: String,
        vulnerabilities: Vec<crate::datadogV2::model::CycloneDXVulnerability>,
    ) -> CycloneDXBom {
        CycloneDXBom {
            bom_format,
            components,
            metadata,
            spec_version,
            version: None,
            vulnerabilities,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
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

impl<'de> Deserialize<'de> for CycloneDXBom {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CycloneDXBomVisitor;
        impl<'a> Visitor<'a> for CycloneDXBomVisitor {
            type Value = CycloneDXBom;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bom_format: Option<String> = None;
                let mut components: Option<Vec<crate::datadogV2::model::CycloneDXComponent>> = None;
                let mut metadata: Option<crate::datadogV2::model::CycloneDXMetadata> = None;
                let mut spec_version: Option<String> = None;
                let mut version: Option<i64> = None;
                let mut vulnerabilities: Option<
                    Vec<crate::datadogV2::model::CycloneDXVulnerability>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bomFormat" => {
                            bom_format = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "components" => {
                            components = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "specVersion" => {
                            spec_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vulnerabilities" => {
                            vulnerabilities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let bom_format = bom_format.ok_or_else(|| M::Error::missing_field("bom_format"))?;
                let components = components.ok_or_else(|| M::Error::missing_field("components"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let spec_version =
                    spec_version.ok_or_else(|| M::Error::missing_field("spec_version"))?;
                let vulnerabilities =
                    vulnerabilities.ok_or_else(|| M::Error::missing_field("vulnerabilities"))?;

                let content = CycloneDXBom {
                    bom_format,
                    components,
                    metadata,
                    spec_version,
                    version,
                    vulnerabilities,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CycloneDXBomVisitor)
    }
}

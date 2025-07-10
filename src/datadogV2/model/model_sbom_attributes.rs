// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API attributes of the SBOM.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SBOMAttributes {
    /// Specifies the format of the BOM. This helps to identify the file as CycloneDX since BOM do not have a filename convention nor does JSON schema support namespaces. This value MUST be `CycloneDX`.
    #[serde(rename = "bomFormat")]
    pub bom_format: String,
    /// A list of software and hardware components.
    #[serde(rename = "components")]
    pub components: Vec<crate::datadogV2::model::SBOMComponent>,
    /// List of dependencies between components of the SBOM.
    #[serde(rename = "dependencies")]
    pub dependencies: Vec<crate::datadogV2::model::SBOMComponentDependency>,
    /// Provides additional information about a BOM.
    #[serde(rename = "metadata")]
    pub metadata: crate::datadogV2::model::SBOMMetadata,
    /// Every BOM generated has a unique serial number, even if the contents of the BOM have not changed overt time. The serial number follows [RFC-4122](<https://datatracker.ietf.org/doc/html/rfc4122>)
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    /// The version of the CycloneDX specification a BOM conforms to.
    #[serde(rename = "specVersion")]
    pub spec_version: crate::datadogV2::model::SpecVersion,
    /// It increments when a BOM is modified. The default value is 1.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SBOMAttributes {
    pub fn new(
        bom_format: String,
        components: Vec<crate::datadogV2::model::SBOMComponent>,
        dependencies: Vec<crate::datadogV2::model::SBOMComponentDependency>,
        metadata: crate::datadogV2::model::SBOMMetadata,
        serial_number: String,
        spec_version: crate::datadogV2::model::SpecVersion,
        version: i64,
    ) -> SBOMAttributes {
        SBOMAttributes {
            bom_format,
            components,
            dependencies,
            metadata,
            serial_number,
            spec_version,
            version,
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

impl<'de> Deserialize<'de> for SBOMAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SBOMAttributesVisitor;
        impl<'a> Visitor<'a> for SBOMAttributesVisitor {
            type Value = SBOMAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bom_format: Option<String> = None;
                let mut components: Option<Vec<crate::datadogV2::model::SBOMComponent>> = None;
                let mut dependencies: Option<
                    Vec<crate::datadogV2::model::SBOMComponentDependency>,
                > = None;
                let mut metadata: Option<crate::datadogV2::model::SBOMMetadata> = None;
                let mut serial_number: Option<String> = None;
                let mut spec_version: Option<crate::datadogV2::model::SpecVersion> = None;
                let mut version: Option<i64> = None;
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
                        "dependencies" => {
                            dependencies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serialNumber" => {
                            serial_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "specVersion" => {
                            spec_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _spec_version) = spec_version {
                                match _spec_version {
                                    crate::datadogV2::model::SpecVersion::UnparsedObject(
                                        _spec_version,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let dependencies =
                    dependencies.ok_or_else(|| M::Error::missing_field("dependencies"))?;
                let metadata = metadata.ok_or_else(|| M::Error::missing_field("metadata"))?;
                let serial_number =
                    serial_number.ok_or_else(|| M::Error::missing_field("serial_number"))?;
                let spec_version =
                    spec_version.ok_or_else(|| M::Error::missing_field("spec_version"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = SBOMAttributes {
                    bom_format,
                    components,
                    dependencies,
                    metadata,
                    serial_number,
                    spec_version,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SBOMAttributesVisitor)
    }
}

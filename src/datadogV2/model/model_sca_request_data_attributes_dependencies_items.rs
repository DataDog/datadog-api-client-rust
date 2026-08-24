// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A dependency found in the repository, including its identity, location, and reachability metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributesDependenciesItems {
    /// A list of patterns or identifiers that should be excluded from analysis for this dependency.
    #[serde(rename = "exclusions")]
    pub exclusions: Option<Vec<String>>,
    /// The group or organization namespace of the dependency (e.g., Maven group ID).
    #[serde(rename = "group", default, with = "::serde_with::rust::double_option")]
    pub group: Option<Option<String>>,
    /// Indicates whether this is a development-only dependency not used in production.
    #[serde(rename = "is_dev")]
    pub is_dev: Option<bool>,
    /// Indicates whether this is a direct dependency (as opposed to a transitive one).
    #[serde(rename = "is_direct", default, with = "::serde_with::rust::double_option")]
    pub is_direct: Option<Option<bool>>,
    /// The programming language ecosystem of this dependency (e.g., java, python, javascript).
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// The list of source file locations where this dependency is declared.
    #[serde(rename = "locations", default, with = "::serde_with::rust::double_option")]
    pub locations: Option<Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems>>>,
    /// The name of the dependency package.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Indicates whether dependency details are intentionally opaque.
    #[serde(rename = "opaque")]
    pub opaque: Option<bool>,
    /// The package manager responsible for this dependency (e.g., maven, pip, npm).
    #[serde(rename = "package_manager")]
    pub package_manager: Option<String>,
    /// The Package URL (PURL) uniquely identifying this dependency.
    #[serde(rename = "purl")]
    pub purl: Option<String>,
    /// Properties describing symbols from this dependency that are reachable in the application code.
    #[serde(rename = "reachable_symbol_properties")]
    pub reachable_symbol_properties: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>>,
    /// Indicates whether this dependency requires transitive dependency enrichment.
    #[serde(rename = "requires_transitive_enrichment")]
    pub requires_transitive_enrichment: Option<bool>,
    /// The target framework identifiers associated with this dependency.
    #[serde(rename = "target_frameworks")]
    pub target_frameworks: Option<Vec<String>>,
    /// The version of the dependency.
    #[serde(rename = "version", default, with = "::serde_with::rust::double_option")]
    pub version: Option<Option<String>>,
    /// Indicates whether the version value represents a version constraint.
    #[serde(rename = "version_constraint")]
    pub version_constraint: Option<bool>,
    /// The version range associated with this dependency when a manifest declares a range.
    #[serde(rename = "version_range")]
    pub version_range: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScaRequestDataAttributesDependenciesItems {
    pub fn new() -> ScaRequestDataAttributesDependenciesItems {
        ScaRequestDataAttributesDependenciesItems {
            exclusions: None,
            group: None,
            is_dev: None,
            is_direct: None,
            language: None,
            locations: None,
            name: None,
            opaque: None,
            package_manager: None,
            purl: None,
            reachable_symbol_properties: None,
            requires_transitive_enrichment: None,
            target_frameworks: None,
            version: None,
            version_constraint: None,
            version_range: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exclusions(mut self, value: Vec<String>) -> Self {
        self.exclusions = Some(value);
        self
    }

    pub fn group(mut self, value: Option<String>) -> Self {
        self.group = Some(value);
        self
    }

    pub fn is_dev(mut self, value: bool) -> Self {
        self.is_dev = Some(value);
        self
    }

    pub fn is_direct(mut self, value: Option<bool>) -> Self {
        self.is_direct = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn locations(
        mut self,
        value: Option<
            Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems>,
        >,
    ) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn opaque(mut self, value: bool) -> Self {
        self.opaque = Some(value);
        self
    }

    pub fn package_manager(mut self, value: String) -> Self {
        self.package_manager = Some(value);
        self
    }

    pub fn purl(mut self, value: String) -> Self {
        self.purl = Some(value);
        self
    }

    pub fn reachable_symbol_properties(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>,
    ) -> Self {
        self.reachable_symbol_properties = Some(value);
        self
    }

    pub fn requires_transitive_enrichment(mut self, value: bool) -> Self {
        self.requires_transitive_enrichment = Some(value);
        self
    }

    pub fn target_frameworks(mut self, value: Vec<String>) -> Self {
        self.target_frameworks = Some(value);
        self
    }

    pub fn version(mut self, value: Option<String>) -> Self {
        self.version = Some(value);
        self
    }

    pub fn version_constraint(mut self, value: bool) -> Self {
        self.version_constraint = Some(value);
        self
    }

    pub fn version_range(mut self, value: String) -> Self {
        self.version_range = Some(value);
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

impl Default for ScaRequestDataAttributesDependenciesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesDependenciesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesDependenciesItemsVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesDependenciesItemsVisitor {
            type Value = ScaRequestDataAttributesDependenciesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclusions: Option<Vec<String>> = None;
                let mut group: Option<Option<String>> = None;
                let mut is_dev: Option<bool> = None;
                let mut is_direct: Option<Option<bool>> = None;
                let mut language: Option<String> = None;
                let mut locations: Option<Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsLocationsItems>>> = None;
                let mut name: Option<String> = None;
                let mut opaque: Option<bool> = None;
                let mut package_manager: Option<String> = None;
                let mut purl: Option<String> = None;
                let mut reachable_symbol_properties: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItemsReachableSymbolPropertiesItems>> = None;
                let mut requires_transitive_enrichment: Option<bool> = None;
                let mut target_frameworks: Option<Vec<String>> = None;
                let mut version: Option<Option<String>> = None;
                let mut version_constraint: Option<bool> = None;
                let mut version_range: Option<String> = None;
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
                        "group" => {
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_dev" => {
                            if v.is_null() {
                                continue;
                            }
                            is_dev = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_direct" => {
                            is_direct = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "opaque" => {
                            if v.is_null() {
                                continue;
                            }
                            opaque = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "package_manager" => {
                            if v.is_null() {
                                continue;
                            }
                            package_manager =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "purl" => {
                            if v.is_null() {
                                continue;
                            }
                            purl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reachable_symbol_properties" => {
                            if v.is_null() {
                                continue;
                            }
                            reachable_symbol_properties =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requires_transitive_enrichment" => {
                            if v.is_null() {
                                continue;
                            }
                            requires_transitive_enrichment =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target_frameworks" => {
                            if v.is_null() {
                                continue;
                            }
                            target_frameworks =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_constraint" => {
                            if v.is_null() {
                                continue;
                            }
                            version_constraint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version_range" => {
                            if v.is_null() {
                                continue;
                            }
                            version_range =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesDependenciesItems {
                    exclusions,
                    group,
                    is_dev,
                    is_direct,
                    language,
                    locations,
                    name,
                    opaque,
                    package_manager,
                    purl,
                    reachable_symbol_properties,
                    requires_transitive_enrichment,
                    target_frameworks,
                    version,
                    version_constraint,
                    version_range,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesDependenciesItemsVisitor)
    }
}

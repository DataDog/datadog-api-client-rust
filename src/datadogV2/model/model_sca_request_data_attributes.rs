// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScaRequestDataAttributes {
    #[serde(rename = "commit")]
    pub commit: Option<crate::datadogV2::model::ScaRequestDataAttributesCommit>,
    #[serde(rename = "dependencies")]
    pub dependencies:
        Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItems>>,
    #[serde(rename = "env")]
    pub env: Option<String>,
    #[serde(rename = "files")]
    pub files: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesFilesItems>>,
    #[serde(rename = "relations")]
    pub relations: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesRelationsItems>>,
    #[serde(rename = "repository")]
    pub repository: Option<crate::datadogV2::model::ScaRequestDataAttributesRepository>,
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "vulnerabilities")]
    pub vulnerabilities:
        Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributes {
    pub fn new() -> ScaRequestDataAttributes {
        ScaRequestDataAttributes {
            commit: None,
            dependencies: None,
            env: None,
            files: None,
            relations: None,
            repository: None,
            service: None,
            tags: None,
            vulnerabilities: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn commit(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesCommit,
    ) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn dependencies(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItems>,
    ) -> Self {
        self.dependencies = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn files(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesFilesItems>,
    ) -> Self {
        self.files = Some(value);
        self
    }

    pub fn relations(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesRelationsItems>,
    ) -> Self {
        self.relations = Some(value);
        self
    }

    pub fn repository(
        mut self,
        value: crate::datadogV2::model::ScaRequestDataAttributesRepository,
    ) -> Self {
        self.repository = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn tags(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn vulnerabilities(
        mut self,
        value: Vec<crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItems>,
    ) -> Self {
        self.vulnerabilities = Some(value);
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

impl Default for ScaRequestDataAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesVisitor {
            type Value = ScaRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut commit: Option<crate::datadogV2::model::ScaRequestDataAttributesCommit> =
                    None;
                let mut dependencies: Option<
                    Vec<crate::datadogV2::model::ScaRequestDataAttributesDependenciesItems>,
                > = None;
                let mut env: Option<String> = None;
                let mut files: Option<
                    Vec<crate::datadogV2::model::ScaRequestDataAttributesFilesItems>,
                > = None;
                let mut relations: Option<
                    Vec<crate::datadogV2::model::ScaRequestDataAttributesRelationsItems>,
                > = None;
                let mut repository: Option<
                    crate::datadogV2::model::ScaRequestDataAttributesRepository,
                > = None;
                let mut service: Option<String> = None;
                let mut tags: Option<std::collections::BTreeMap<String, String>> = None;
                let mut vulnerabilities: Option<
                    Vec<crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "commit" => {
                            if v.is_null() {
                                continue;
                            }
                            commit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dependencies" => {
                            if v.is_null() {
                                continue;
                            }
                            dependencies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "files" => {
                            if v.is_null() {
                                continue;
                            }
                            files = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relations" => {
                            if v.is_null() {
                                continue;
                            }
                            relations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "repository" => {
                            if v.is_null() {
                                continue;
                            }
                            repository = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vulnerabilities" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content = ScaRequestDataAttributes {
                    commit,
                    dependencies,
                    env,
                    files,
                    relations,
                    repository,
                    service,
                    tags,
                    vulnerabilities,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesVisitor)
    }
}

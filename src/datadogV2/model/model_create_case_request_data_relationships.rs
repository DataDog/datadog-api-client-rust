// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of the case to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCaseRequestDataRelationships {
    /// A list of security findings.
    #[serde(rename = "findings")]
    pub findings: crate::datadogV2::model::Findings,
    /// Case management project.
    #[serde(rename = "project")]
    pub project: crate::datadogV2::model::CaseManagementProject,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateCaseRequestDataRelationships {
    pub fn new(
        findings: crate::datadogV2::model::Findings,
        project: crate::datadogV2::model::CaseManagementProject,
    ) -> CreateCaseRequestDataRelationships {
        CreateCaseRequestDataRelationships {
            findings,
            project,
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

impl<'de> Deserialize<'de> for CreateCaseRequestDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateCaseRequestDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for CreateCaseRequestDataRelationshipsVisitor {
            type Value = CreateCaseRequestDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut findings: Option<crate::datadogV2::model::Findings> = None;
                let mut project: Option<crate::datadogV2::model::CaseManagementProject> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "findings" => {
                            findings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project" => {
                            project = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let findings = findings.ok_or_else(|| M::Error::missing_field("findings"))?;
                let project = project.ok_or_else(|| M::Error::missing_field("project"))?;

                let content = CreateCaseRequestDataRelationships {
                    findings,
                    project,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateCaseRequestDataRelationshipsVisitor)
    }
}

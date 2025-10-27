// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Threat hunting job attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ThreatHuntingJobResponseAttributes {
    /// Time when the job was created.
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// The handle of the user who created the job.
    #[serde(rename = "createdByHandle")]
    pub created_by_handle: Option<String>,
    /// The name of the user who created the job.
    #[serde(rename = "createdByName")]
    pub created_by_name: Option<String>,
    /// ID of the rule used to create the job (if it is created from a rule).
    #[serde(rename = "createdFromRuleId")]
    pub created_from_rule_id: Option<String>,
    /// Definition of a threat hunting job.
    #[serde(rename = "jobDefinition")]
    pub job_definition: Option<crate::datadogV2::model::JobDefinition>,
    /// Job name.
    #[serde(rename = "jobName")]
    pub job_name: Option<String>,
    /// Job status.
    #[serde(rename = "jobStatus")]
    pub job_status: Option<String>,
    /// Last modification time of the job.
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ThreatHuntingJobResponseAttributes {
    pub fn new() -> ThreatHuntingJobResponseAttributes {
        ThreatHuntingJobResponseAttributes {
            created_at: None,
            created_by_handle: None,
            created_by_name: None,
            created_from_rule_id: None,
            job_definition: None,
            job_name: None,
            job_status: None,
            modified_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_handle(mut self, value: String) -> Self {
        self.created_by_handle = Some(value);
        self
    }

    pub fn created_by_name(mut self, value: String) -> Self {
        self.created_by_name = Some(value);
        self
    }

    pub fn created_from_rule_id(mut self, value: String) -> Self {
        self.created_from_rule_id = Some(value);
        self
    }

    pub fn job_definition(mut self, value: crate::datadogV2::model::JobDefinition) -> Self {
        self.job_definition = Some(value);
        self
    }

    pub fn job_name(mut self, value: String) -> Self {
        self.job_name = Some(value);
        self
    }

    pub fn job_status(mut self, value: String) -> Self {
        self.job_status = Some(value);
        self
    }

    pub fn modified_at(mut self, value: String) -> Self {
        self.modified_at = Some(value);
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

impl Default for ThreatHuntingJobResponseAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ThreatHuntingJobResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThreatHuntingJobResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ThreatHuntingJobResponseAttributesVisitor {
            type Value = ThreatHuntingJobResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<String> = None;
                let mut created_by_handle: Option<String> = None;
                let mut created_by_name: Option<String> = None;
                let mut created_from_rule_id: Option<String> = None;
                let mut job_definition: Option<crate::datadogV2::model::JobDefinition> = None;
                let mut job_name: Option<String> = None;
                let mut job_status: Option<String> = None;
                let mut modified_at: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "createdAt" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByHandle" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_handle =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdByName" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "createdFromRuleId" => {
                            if v.is_null() {
                                continue;
                            }
                            created_from_rule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobDefinition" => {
                            if v.is_null() {
                                continue;
                            }
                            job_definition =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobName" => {
                            if v.is_null() {
                                continue;
                            }
                            job_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "jobStatus" => {
                            if v.is_null() {
                                continue;
                            }
                            job_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modifiedAt" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ThreatHuntingJobResponseAttributes {
                    created_at,
                    created_by_handle,
                    created_by_name,
                    created_from_rule_id,
                    job_definition,
                    job_name,
                    job_status,
                    modified_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ThreatHuntingJobResponseAttributesVisitor)
    }
}

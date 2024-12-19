// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CSM Hosts and Containers Coverage Analysis attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmHostsAndContainersCoverageAnalysisAttributes {
    /// CSM Coverage Analysis.
    #[serde(rename = "cspm_coverage")]
    pub cspm_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// CSM Coverage Analysis.
    #[serde(rename = "cws_coverage")]
    pub cws_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// The ID of your organization.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// CSM Coverage Analysis.
    #[serde(rename = "total_coverage")]
    pub total_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// CSM Coverage Analysis.
    #[serde(rename = "vm_coverage")]
    pub vm_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmHostsAndContainersCoverageAnalysisAttributes {
    pub fn new() -> CsmHostsAndContainersCoverageAnalysisAttributes {
        CsmHostsAndContainersCoverageAnalysisAttributes {
            cspm_coverage: None,
            cws_coverage: None,
            org_id: None,
            total_coverage: None,
            vm_coverage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cspm_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.cspm_coverage = Some(value);
        self
    }

    pub fn cws_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.cws_coverage = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn total_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.total_coverage = Some(value);
        self
    }

    pub fn vm_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.vm_coverage = Some(value);
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

impl Default for CsmHostsAndContainersCoverageAnalysisAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CsmHostsAndContainersCoverageAnalysisAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmHostsAndContainersCoverageAnalysisAttributesVisitor;
        impl<'a> Visitor<'a> for CsmHostsAndContainersCoverageAnalysisAttributesVisitor {
            type Value = CsmHostsAndContainersCoverageAnalysisAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cspm_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut cws_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut org_id: Option<i64> = None;
                let mut total_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut vm_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cspm_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            cspm_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cws_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            cws_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            total_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vm_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            vm_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CsmHostsAndContainersCoverageAnalysisAttributes {
                    cspm_coverage,
                    cws_coverage,
                    org_id,
                    total_coverage,
                    vm_coverage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmHostsAndContainersCoverageAnalysisAttributesVisitor)
    }
}

// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CSM Cloud Accounts Coverage Analysis attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CsmCloudAccountsCoverageAnalysisAttributes {
    /// CSM Coverage Analysis.
    #[serde(rename = "aws_coverage")]
    pub aws_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// CSM Coverage Analysis.
    #[serde(rename = "azure_coverage")]
    pub azure_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// CSM Coverage Analysis.
    #[serde(rename = "gcp_coverage")]
    pub gcp_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    /// The ID of your organization.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// CSM Coverage Analysis.
    #[serde(rename = "total_coverage")]
    pub total_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CsmCloudAccountsCoverageAnalysisAttributes {
    pub fn new() -> CsmCloudAccountsCoverageAnalysisAttributes {
        CsmCloudAccountsCoverageAnalysisAttributes {
            aws_coverage: None,
            azure_coverage: None,
            gcp_coverage: None,
            org_id: None,
            total_coverage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn aws_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.aws_coverage = Some(value);
        self
    }

    pub fn azure_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.azure_coverage = Some(value);
        self
    }

    pub fn gcp_coverage(mut self, value: crate::datadogV2::model::CsmCoverageAnalysis) -> Self {
        self.gcp_coverage = Some(value);
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

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for CsmCloudAccountsCoverageAnalysisAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CsmCloudAccountsCoverageAnalysisAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CsmCloudAccountsCoverageAnalysisAttributesVisitor;
        impl<'a> Visitor<'a> for CsmCloudAccountsCoverageAnalysisAttributesVisitor {
            type Value = CsmCloudAccountsCoverageAnalysisAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aws_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut azure_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut gcp_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut org_id: Option<i64> = None;
                let mut total_coverage: Option<crate::datadogV2::model::CsmCoverageAnalysis> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aws_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            aws_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "azure_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            azure_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "gcp_coverage" => {
                            if v.is_null() {
                                continue;
                            }
                            gcp_coverage =
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CsmCloudAccountsCoverageAnalysisAttributes {
                    aws_coverage,
                    azure_coverage,
                    gcp_coverage,
                    org_id,
                    total_coverage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CsmCloudAccountsCoverageAnalysisAttributesVisitor)
    }
}

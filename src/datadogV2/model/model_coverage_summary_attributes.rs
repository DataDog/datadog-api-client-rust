// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes object for code coverage summary response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoverageSummaryAttributes {
    /// Coverage statistics broken down by code owner.
    #[serde(
        rename = "codeowners",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub codeowners: Option<
        Option<
            std::collections::BTreeMap<
                String,
                crate::datadogV2::model::CoverageSummaryCodeownerStats,
            >,
        >,
    >,
    /// Total number of coverage flags evaluated.
    #[serde(rename = "evaluated_flags_count")]
    pub evaluated_flags_count: Option<i64>,
    /// Total number of coverage reports evaluated.
    #[serde(rename = "evaluated_reports_count")]
    pub evaluated_reports_count: Option<i64>,
    /// Overall patch coverage percentage.
    #[serde(
        rename = "patch_coverage",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub patch_coverage: Option<Option<f64>>,
    /// Coverage statistics broken down by service.
    #[serde(
        rename = "services",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub services: Option<
        Option<
            std::collections::BTreeMap<
                String,
                crate::datadogV2::model::CoverageSummaryServiceStats,
            >,
        >,
    >,
    /// Overall total coverage percentage.
    #[serde(
        rename = "total_coverage",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_coverage: Option<Option<f64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CoverageSummaryAttributes {
    pub fn new() -> CoverageSummaryAttributes {
        CoverageSummaryAttributes {
            codeowners: None,
            evaluated_flags_count: None,
            evaluated_reports_count: None,
            patch_coverage: None,
            services: None,
            total_coverage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn codeowners(
        mut self,
        value: Option<
            std::collections::BTreeMap<
                String,
                crate::datadogV2::model::CoverageSummaryCodeownerStats,
            >,
        >,
    ) -> Self {
        self.codeowners = Some(value);
        self
    }

    pub fn evaluated_flags_count(mut self, value: i64) -> Self {
        self.evaluated_flags_count = Some(value);
        self
    }

    pub fn evaluated_reports_count(mut self, value: i64) -> Self {
        self.evaluated_reports_count = Some(value);
        self
    }

    pub fn patch_coverage(mut self, value: Option<f64>) -> Self {
        self.patch_coverage = Some(value);
        self
    }

    pub fn services(
        mut self,
        value: Option<
            std::collections::BTreeMap<
                String,
                crate::datadogV2::model::CoverageSummaryServiceStats,
            >,
        >,
    ) -> Self {
        self.services = Some(value);
        self
    }

    pub fn total_coverage(mut self, value: Option<f64>) -> Self {
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

impl Default for CoverageSummaryAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoverageSummaryAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoverageSummaryAttributesVisitor;
        impl<'a> Visitor<'a> for CoverageSummaryAttributesVisitor {
            type Value = CoverageSummaryAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut codeowners: Option<
                    Option<
                        std::collections::BTreeMap<
                            String,
                            crate::datadogV2::model::CoverageSummaryCodeownerStats,
                        >,
                    >,
                > = None;
                let mut evaluated_flags_count: Option<i64> = None;
                let mut evaluated_reports_count: Option<i64> = None;
                let mut patch_coverage: Option<Option<f64>> = None;
                let mut services: Option<
                    Option<
                        std::collections::BTreeMap<
                            String,
                            crate::datadogV2::model::CoverageSummaryServiceStats,
                        >,
                    >,
                > = None;
                let mut total_coverage: Option<Option<f64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "codeowners" => {
                            codeowners = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluated_flags_count" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluated_flags_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "evaluated_reports_count" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluated_reports_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "patch_coverage" => {
                            patch_coverage =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "services" => {
                            services = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_coverage" => {
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

                let content = CoverageSummaryAttributes {
                    codeowners,
                    evaluated_flags_count,
                    evaluated_reports_count,
                    patch_coverage,
                    services,
                    total_coverage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoverageSummaryAttributesVisitor)
    }
}

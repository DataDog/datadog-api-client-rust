// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Cost and Usage Report data export configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataExportConfig {
    /// Name of the S3 bucket where the Cost and Usage Report is stored.
    #[serde(rename = "bucket_name")]
    pub bucket_name: Option<String>,
    /// AWS region of the S3 bucket.
    #[serde(rename = "bucket_region")]
    pub bucket_region: Option<String>,
    /// Name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: Option<String>,
    /// S3 prefix where the Cost and Usage Report is stored.
    #[serde(rename = "report_prefix")]
    pub report_prefix: Option<String>,
    /// Type of the Cost and Usage Report.
    #[serde(rename = "report_type")]
    pub report_type: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataExportConfig {
    pub fn new() -> DataExportConfig {
        DataExportConfig {
            bucket_name: None,
            bucket_region: None,
            report_name: None,
            report_prefix: None,
            report_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bucket_name(mut self, value: String) -> Self {
        self.bucket_name = Some(value);
        self
    }

    pub fn bucket_region(mut self, value: String) -> Self {
        self.bucket_region = Some(value);
        self
    }

    pub fn report_name(mut self, value: String) -> Self {
        self.report_name = Some(value);
        self
    }

    pub fn report_prefix(mut self, value: String) -> Self {
        self.report_prefix = Some(value);
        self
    }

    pub fn report_type(mut self, value: String) -> Self {
        self.report_type = Some(value);
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

impl Default for DataExportConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DataExportConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataExportConfigVisitor;
        impl<'a> Visitor<'a> for DataExportConfigVisitor {
            type Value = DataExportConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bucket_name: Option<String> = None;
                let mut bucket_region: Option<String> = None;
                let mut report_name: Option<String> = None;
                let mut report_prefix: Option<String> = None;
                let mut report_type: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bucket_name" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_region" => {
                            if v.is_null() {
                                continue;
                            }
                            bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_name" => {
                            if v.is_null() {
                                continue;
                            }
                            report_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_prefix" => {
                            if v.is_null() {
                                continue;
                            }
                            report_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_type" => {
                            if v.is_null() {
                                continue;
                            }
                            report_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DataExportConfig {
                    bucket_name,
                    bucket_region,
                    report_name,
                    report_prefix,
                    report_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataExportConfigVisitor)
    }
}

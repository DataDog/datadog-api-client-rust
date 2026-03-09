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
    pub bucket_name: String,
    /// AWS region of the S3 bucket.
    #[serde(rename = "bucket_region")]
    pub bucket_region: String,
    /// Name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// S3 prefix where the Cost and Usage Report is stored.
    #[serde(rename = "report_prefix")]
    pub report_prefix: String,
    /// Type of the Cost and Usage Report. Currently only `CUR2.0` is supported.
    #[serde(rename = "report_type")]
    pub report_type: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataExportConfig {
    pub fn new(
        bucket_name: String,
        bucket_region: String,
        report_name: String,
        report_prefix: String,
        report_type: String,
    ) -> DataExportConfig {
        DataExportConfig {
            bucket_name,
            bucket_region,
            report_name,
            report_prefix,
            report_type,
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
                            bucket_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bucket_region" => {
                            bucket_region =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_name" => {
                            report_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_prefix" => {
                            report_prefix =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "report_type" => {
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
                let bucket_name =
                    bucket_name.ok_or_else(|| M::Error::missing_field("bucket_name"))?;
                let bucket_region =
                    bucket_region.ok_or_else(|| M::Error::missing_field("bucket_region"))?;
                let report_name =
                    report_name.ok_or_else(|| M::Error::missing_field("report_name"))?;
                let report_prefix =
                    report_prefix.ok_or_else(|| M::Error::missing_field("report_prefix"))?;
                let report_type =
                    report_type.ok_or_else(|| M::Error::missing_field("report_type"))?;

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

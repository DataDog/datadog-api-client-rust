// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the sampling quality for a signals and problems query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SignalsProblemsSampleMetadata {
    /// Number of view instances that failed to process.
    #[serde(rename = "failed")]
    pub failed: i32,
    /// Number of view instances requested for sampling.
    #[serde(rename = "requested")]
    pub requested: i32,
    /// List of RUM view IDs that were sampled.
    #[serde(rename = "sampled_view_ids")]
    pub sampled_view_ids: Vec<String>,
    /// Number of view instances successfully processed.
    #[serde(rename = "succeeded")]
    pub succeeded: i32,
    /// Ratio of successfully processed views to requested views.
    #[serde(rename = "success_rate")]
    pub success_rate: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SignalsProblemsSampleMetadata {
    pub fn new(
        failed: i32,
        requested: i32,
        sampled_view_ids: Vec<String>,
        succeeded: i32,
        success_rate: f64,
    ) -> SignalsProblemsSampleMetadata {
        SignalsProblemsSampleMetadata {
            failed,
            requested,
            sampled_view_ids,
            succeeded,
            success_rate,
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

impl<'de> Deserialize<'de> for SignalsProblemsSampleMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignalsProblemsSampleMetadataVisitor;
        impl<'a> Visitor<'a> for SignalsProblemsSampleMetadataVisitor {
            type Value = SignalsProblemsSampleMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failed: Option<i32> = None;
                let mut requested: Option<i32> = None;
                let mut sampled_view_ids: Option<Vec<String>> = None;
                let mut succeeded: Option<i32> = None;
                let mut success_rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failed" => {
                            failed = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requested" => {
                            requested = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sampled_view_ids" => {
                            sampled_view_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "succeeded" => {
                            succeeded = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "success_rate" => {
                            success_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let failed = failed.ok_or_else(|| M::Error::missing_field("failed"))?;
                let requested = requested.ok_or_else(|| M::Error::missing_field("requested"))?;
                let sampled_view_ids =
                    sampled_view_ids.ok_or_else(|| M::Error::missing_field("sampled_view_ids"))?;
                let succeeded = succeeded.ok_or_else(|| M::Error::missing_field("succeeded"))?;
                let success_rate =
                    success_rate.ok_or_else(|| M::Error::missing_field("success_rate"))?;

                let content = SignalsProblemsSampleMetadata {
                    failed,
                    requested,
                    sampled_view_ids,
                    succeeded,
                    success_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SignalsProblemsSampleMetadataVisitor)
    }
}

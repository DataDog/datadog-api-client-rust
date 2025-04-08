// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The maximum amount of data or number of events allowed before the quota is enforced. Can be specified in bytes or events.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineQuotaProcessorLimit {
    /// Unit for quota enforcement in bytes for data size or events for count.
    #[serde(rename = "enforce")]
    pub enforce: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimitEnforceType,
    /// The limit for quota enforcement.
    #[serde(rename = "limit")]
    pub limit: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineQuotaProcessorLimit {
    pub fn new(
        enforce: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimitEnforceType,
        limit: i64,
    ) -> ObservabilityPipelineQuotaProcessorLimit {
        ObservabilityPipelineQuotaProcessorLimit {
            enforce,
            limit,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineQuotaProcessorLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineQuotaProcessorLimitVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineQuotaProcessorLimitVisitor {
            type Value = ObservabilityPipelineQuotaProcessorLimit;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enforce: Option<
                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimitEnforceType,
                > = None;
                let mut limit: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enforce" => {
                            enforce = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _enforce) = enforce {
                                match _enforce {
                                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimitEnforceType::UnparsedObject(_enforce) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enforce = enforce.ok_or_else(|| M::Error::missing_field("enforce"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;

                let content = ObservabilityPipelineQuotaProcessorLimit {
                    enforce,
                    limit,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineQuotaProcessorLimitVisitor)
    }
}

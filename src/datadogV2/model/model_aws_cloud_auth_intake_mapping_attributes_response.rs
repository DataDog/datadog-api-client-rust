// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for AWS cloud authentication intake mapping response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSCloudAuthIntakeMappingAttributesResponse {
    /// AWS caller ARN pattern allowed to authenticate for telemetry submission. ARN syntax and AWS integration membership are validated by the server. For an assumed role, use the STS assumed-role ARN with a trailing /* wildcard to match its sessions.
    #[serde(rename = "arn_pattern")]
    pub arn_pattern: String,
    /// Whether the mapping allows Private Action Runner enrollment. This attribute may be omitted when enrollment is disabled. Enrollment permission is managed separately from the ARN pattern.
    #[serde(rename = "par_enrollment_enabled")]
    pub par_enrollment_enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSCloudAuthIntakeMappingAttributesResponse {
    pub fn new(arn_pattern: String) -> AWSCloudAuthIntakeMappingAttributesResponse {
        AWSCloudAuthIntakeMappingAttributesResponse {
            arn_pattern,
            par_enrollment_enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn par_enrollment_enabled(mut self, value: bool) -> Self {
        self.par_enrollment_enabled = Some(value);
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

impl<'de> Deserialize<'de> for AWSCloudAuthIntakeMappingAttributesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSCloudAuthIntakeMappingAttributesResponseVisitor;
        impl<'a> Visitor<'a> for AWSCloudAuthIntakeMappingAttributesResponseVisitor {
            type Value = AWSCloudAuthIntakeMappingAttributesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arn_pattern: Option<String> = None;
                let mut par_enrollment_enabled: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arn_pattern" => {
                            arn_pattern =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "par_enrollment_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            par_enrollment_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let arn_pattern =
                    arn_pattern.ok_or_else(|| M::Error::missing_field("arn_pattern"))?;

                let content = AWSCloudAuthIntakeMappingAttributesResponse {
                    arn_pattern,
                    par_enrollment_enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSCloudAuthIntakeMappingAttributesResponseVisitor)
    }
}

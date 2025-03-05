// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the AWS on demand task.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsOnDemandAttributes {
    /// The arn of the resource to scan.
    #[serde(rename = "arn")]
    pub arn: Option<String>,
    /// Specifies the assignment timestamp if the task has been already assigned to a scanner.
    #[serde(rename = "assigned_at")]
    pub assigned_at: Option<String>,
    /// The task submission timestamp.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Indicates the status of the task.
    /// QUEUED: the task has been submitted successfully and the resource has not been assigned to a scanner yet.
    /// ASSIGNED: the task has been assigned.
    /// ABORTED: the scan has been aborted after a period of time due to technical reasons, such as resource not found, insufficient permissions, or the absence of a configured scanner.
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsOnDemandAttributes {
    pub fn new() -> AwsOnDemandAttributes {
        AwsOnDemandAttributes {
            arn: None,
            assigned_at: None,
            created_at: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn arn(mut self, value: String) -> Self {
        self.arn = Some(value);
        self
    }

    pub fn assigned_at(mut self, value: String) -> Self {
        self.assigned_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn status(mut self, value: String) -> Self {
        self.status = Some(value);
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

impl Default for AwsOnDemandAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AwsOnDemandAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsOnDemandAttributesVisitor;
        impl<'a> Visitor<'a> for AwsOnDemandAttributesVisitor {
            type Value = AwsOnDemandAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arn: Option<String> = None;
                let mut assigned_at: Option<String> = None;
                let mut created_at: Option<String> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arn" => {
                            if v.is_null() {
                                continue;
                            }
                            arn = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "assigned_at" => {
                            if v.is_null() {
                                continue;
                            }
                            assigned_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AwsOnDemandAttributes {
                    arn,
                    assigned_at,
                    created_at,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsOnDemandAttributesVisitor)
    }
}

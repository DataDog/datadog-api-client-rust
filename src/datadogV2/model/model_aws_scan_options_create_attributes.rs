// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the AWS scan options to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AwsScanOptionsCreateAttributes {
    /// Indicates if scanning of Lambda functions is enabled.
    #[serde(rename = "lambda")]
    pub lambda: bool,
    /// Indicates if scanning for sensitive data is enabled.
    #[serde(rename = "sensitive_data")]
    pub sensitive_data: bool,
    /// Indicates if scanning for vulnerabilities in containers is enabled.
    #[serde(rename = "vuln_containers_os")]
    pub vuln_containers_os: bool,
    /// Indicates if scanning for vulnerabilities in hosts is enabled.
    #[serde(rename = "vuln_host_os")]
    pub vuln_host_os: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AwsScanOptionsCreateAttributes {
    pub fn new(
        lambda: bool,
        sensitive_data: bool,
        vuln_containers_os: bool,
        vuln_host_os: bool,
    ) -> AwsScanOptionsCreateAttributes {
        AwsScanOptionsCreateAttributes {
            lambda,
            sensitive_data,
            vuln_containers_os,
            vuln_host_os,
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

impl<'de> Deserialize<'de> for AwsScanOptionsCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AwsScanOptionsCreateAttributesVisitor;
        impl<'a> Visitor<'a> for AwsScanOptionsCreateAttributesVisitor {
            type Value = AwsScanOptionsCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut lambda: Option<bool> = None;
                let mut sensitive_data: Option<bool> = None;
                let mut vuln_containers_os: Option<bool> = None;
                let mut vuln_host_os: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "lambda" => {
                            lambda = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sensitive_data" => {
                            sensitive_data =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_containers_os" => {
                            vuln_containers_os =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_host_os" => {
                            vuln_host_os =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let lambda = lambda.ok_or_else(|| M::Error::missing_field("lambda"))?;
                let sensitive_data =
                    sensitive_data.ok_or_else(|| M::Error::missing_field("sensitive_data"))?;
                let vuln_containers_os = vuln_containers_os
                    .ok_or_else(|| M::Error::missing_field("vuln_containers_os"))?;
                let vuln_host_os =
                    vuln_host_os.ok_or_else(|| M::Error::missing_field("vuln_host_os"))?;

                let content = AwsScanOptionsCreateAttributes {
                    lambda,
                    sensitive_data,
                    vuln_containers_os,
                    vuln_host_os,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AwsScanOptionsCreateAttributesVisitor)
    }
}

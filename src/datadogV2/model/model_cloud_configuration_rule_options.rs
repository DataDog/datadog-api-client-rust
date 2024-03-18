// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on cloud configuration rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudConfigurationRuleOptions {
    /// Options for cloud_configuration rules.
    /// Fields `resourceType` and `regoRule` are mandatory when managing custom `cloud_configuration` rules.
    ///
    #[serde(rename = "complianceRuleOptions")]
    pub compliance_rule_options: crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudConfigurationRuleOptions {
    pub fn new(
        compliance_rule_options: crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
    ) -> CloudConfigurationRuleOptions {
        CloudConfigurationRuleOptions {
            compliance_rule_options,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for CloudConfigurationRuleOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudConfigurationRuleOptionsVisitor;
        impl<'a> Visitor<'a> for CloudConfigurationRuleOptionsVisitor {
            type Value = CloudConfigurationRuleOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compliance_rule_options: Option<
                    crate::datadogV2::model::CloudConfigurationComplianceRuleOptions,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "complianceRuleOptions" => {
                            compliance_rule_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let compliance_rule_options = compliance_rule_options
                    .ok_or_else(|| M::Error::missing_field("compliance_rule_options"))?;

                let content = CloudConfigurationRuleOptions {
                    compliance_rule_options,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudConfigurationRuleOptionsVisitor)
    }
}

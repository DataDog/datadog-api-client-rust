// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for the enrollment status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnPremManagementServiceGetEnrollmentResponseAttributes {
    /// The reason for enrollment failure, if applicable.
    #[serde(rename = "failure_reason")]
    pub failure_reason: String,
    /// The runner identifier, if enrollment was successful.
    #[serde(rename = "runner_id")]
    pub runner_id: String,
    /// The status of the enrollment.
    #[serde(rename = "status")]
    pub status:
        crate::datadogV2::model::OnPremManagementServiceGetEnrollmentResponseAttributesStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnPremManagementServiceGetEnrollmentResponseAttributes {
    pub fn new(
        failure_reason: String,
        runner_id: String,
        status: crate::datadogV2::model::OnPremManagementServiceGetEnrollmentResponseAttributesStatus,
    ) -> OnPremManagementServiceGetEnrollmentResponseAttributes {
        OnPremManagementServiceGetEnrollmentResponseAttributes {
            failure_reason,
            runner_id,
            status,
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

impl<'de> Deserialize<'de> for OnPremManagementServiceGetEnrollmentResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnPremManagementServiceGetEnrollmentResponseAttributesVisitor;
        impl<'a> Visitor<'a> for OnPremManagementServiceGetEnrollmentResponseAttributesVisitor {
            type Value = OnPremManagementServiceGetEnrollmentResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut failure_reason: Option<String> = None;
                let mut runner_id: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::OnPremManagementServiceGetEnrollmentResponseAttributesStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "failure_reason" => {
                            failure_reason =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "runner_id" => {
                            runner_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::OnPremManagementServiceGetEnrollmentResponseAttributesStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let failure_reason =
                    failure_reason.ok_or_else(|| M::Error::missing_field("failure_reason"))?;
                let runner_id = runner_id.ok_or_else(|| M::Error::missing_field("runner_id"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = OnPremManagementServiceGetEnrollmentResponseAttributes {
                    failure_reason,
                    runner_id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnPremManagementServiceGetEnrollmentResponseAttributesVisitor)
    }
}

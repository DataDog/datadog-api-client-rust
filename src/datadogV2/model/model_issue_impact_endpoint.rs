// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An endpoint whose traces have the issue's errors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueImpactEndpoint {
    /// Number of errors reported by each source. Several sources can report the same error, such as an APM span and
    /// a log, so the counts overlap: don't add them together.
    #[serde(rename = "error_counts")]
    pub error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
    /// Resource name of the trace's root span.
    #[serde(rename = "resource_name")]
    pub resource_name: String,
    /// Service of the trace's root span.
    #[serde(rename = "service")]
    pub service: String,
    /// Whether the entry stands out as a likely driver of the issue, because it accounts for most of the issue's
    /// errors or for far more errors than the other listed entries.
    #[serde(rename = "significant")]
    pub significant: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueImpactEndpoint {
    pub fn new(
        error_counts: crate::datadogV2::model::IssueImpactErrorCounts,
        resource_name: String,
        service: String,
        significant: bool,
    ) -> IssueImpactEndpoint {
        IssueImpactEndpoint {
            error_counts,
            resource_name,
            service,
            significant,
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

impl<'de> Deserialize<'de> for IssueImpactEndpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueImpactEndpointVisitor;
        impl<'a> Visitor<'a> for IssueImpactEndpointVisitor {
            type Value = IssueImpactEndpoint;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_counts: Option<crate::datadogV2::model::IssueImpactErrorCounts> =
                    None;
                let mut resource_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut significant: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_counts" => {
                            error_counts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_name" => {
                            resource_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "significant" => {
                            significant =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error_counts =
                    error_counts.ok_or_else(|| M::Error::missing_field("error_counts"))?;
                let resource_name =
                    resource_name.ok_or_else(|| M::Error::missing_field("resource_name"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let significant =
                    significant.ok_or_else(|| M::Error::missing_field("significant"))?;

                let content = IssueImpactEndpoint {
                    error_counts,
                    resource_name,
                    service,
                    significant,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueImpactEndpointVisitor)
    }
}

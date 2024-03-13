// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Incident integration metadata for the Jira integration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JiraIntegrationMetadata {
    /// Array of Jira issues in this integration metadata.
    #[serde(rename = "issues")]
    pub issues: Vec<crate::datadogV2::model::JiraIntegrationMetadataIssuesItem>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JiraIntegrationMetadata {
    pub fn new(
        issues: Vec<crate::datadogV2::model::JiraIntegrationMetadataIssuesItem>,
    ) -> JiraIntegrationMetadata {
        JiraIntegrationMetadata {
            issues,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for JiraIntegrationMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JiraIntegrationMetadataVisitor;
        impl<'a> Visitor<'a> for JiraIntegrationMetadataVisitor {
            type Value = JiraIntegrationMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut issues: Option<
                    Vec<crate::datadogV2::model::JiraIntegrationMetadataIssuesItem>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "issues" => {
                            issues = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let issues = issues.ok_or_else(|| M::Error::missing_field("issues"))?;

                let content = JiraIntegrationMetadata { issues, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JiraIntegrationMetadataVisitor)
    }
}

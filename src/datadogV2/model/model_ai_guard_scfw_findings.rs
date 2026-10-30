// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Supply Chain Firewall verdict for the version-pinned install packages
/// found in the request. Evaluation requires Supply Chain Firewall to be enabled
/// for the organization and the caller to have the `appsec_vm_read` permission.
/// Without that permission, Supply Chain Firewall evaluation is skipped; the
/// AI Guard evaluation request itself is not rejected. Findings are omitted
/// when Supply Chain Firewall does not produce a result.
/// The recommended action combines the package outcomes.
/// Confirmation support is inferred from the evaluator profile, not from a
/// separate confirmation-capability flag. By default, a non-empty
/// `meta.coding_agent` selects the coding-agent profile, which assumes the caller
/// can ask for confirmation and retains WARN in `recommended_action`.
/// Omitting `meta.coding_agent` or setting it to an empty string selects the
/// default profile, which assumes the caller cannot ask for confirmation and
/// returns ALLOW instead of WARN in `recommended_action`. Package outcomes
/// still retain WARN, and BLOCK is never changed to ALLOW by this rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardScfwFindings {
    /// The packages the firewall evaluated, `ALLOW` ones included.
    #[serde(rename = "packages")]
    pub packages: Option<Vec<crate::datadogV2::model::AIGuardScfwPackageFinding>>,
    /// The Supply Chain Firewall outcome for a package, a matched rule, or the whole request.
    #[serde(rename = "recommended_action")]
    pub recommended_action: crate::datadogV2::model::AIGuardScfwOutcome,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardScfwFindings {
    pub fn new(
        recommended_action: crate::datadogV2::model::AIGuardScfwOutcome,
    ) -> AIGuardScfwFindings {
        AIGuardScfwFindings {
            packages: None,
            recommended_action,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn packages(
        mut self,
        value: Vec<crate::datadogV2::model::AIGuardScfwPackageFinding>,
    ) -> Self {
        self.packages = Some(value);
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

impl<'de> Deserialize<'de> for AIGuardScfwFindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardScfwFindingsVisitor;
        impl<'a> Visitor<'a> for AIGuardScfwFindingsVisitor {
            type Value = AIGuardScfwFindings;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut packages: Option<Vec<crate::datadogV2::model::AIGuardScfwPackageFinding>> =
                    None;
                let mut recommended_action: Option<crate::datadogV2::model::AIGuardScfwOutcome> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "packages" => {
                            if v.is_null() {
                                continue;
                            }
                            packages = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended_action" => {
                            recommended_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _recommended_action) = recommended_action {
                                match _recommended_action {
                                    crate::datadogV2::model::AIGuardScfwOutcome::UnparsedObject(
                                        _recommended_action,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let recommended_action = recommended_action
                    .ok_or_else(|| M::Error::missing_field("recommended_action"))?;

                let content = AIGuardScfwFindings {
                    packages,
                    recommended_action,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardScfwFindingsVisitor)
    }
}

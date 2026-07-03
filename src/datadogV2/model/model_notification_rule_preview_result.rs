// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The preview result for a single rule type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotificationRulePreviewResult {
    /// The notification status for the given rule type. `SUCCESS` means a matching event was found and the notification was sent successfully. `DEFAULT` means no matching event was found and a default placeholder notification was sent instead. `ERROR` means an error occurred while sending the notification.
    #[serde(rename = "notification_status")]
    pub notification_status: crate::datadogV2::model::NotificationRulePreviewNotificationStatus,
    /// Security rule type which can be used in security rules.
    /// Signal-based notification rules can filter signals based on rule types application_security, log_detection,
    /// workload_security, signal_correlation, cloud_configuration and infrastructure_configuration.
    /// Vulnerability-based notification rules can filter vulnerabilities based on rule types application_code_vulnerability,
    /// application_library_vulnerability, attack_path, container_image_vulnerability, identity_risk, misconfiguration,
    /// api_security, host_vulnerability, iac_misconfiguration, sast_vulnerability, secret_vulnerability and workload_activity.
    #[serde(rename = "rule_type")]
    pub rule_type: crate::datadogV2::model::RuleTypesItems,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotificationRulePreviewResult {
    pub fn new(
        notification_status: crate::datadogV2::model::NotificationRulePreviewNotificationStatus,
        rule_type: crate::datadogV2::model::RuleTypesItems,
    ) -> NotificationRulePreviewResult {
        NotificationRulePreviewResult {
            notification_status,
            rule_type,
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

impl<'de> Deserialize<'de> for NotificationRulePreviewResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotificationRulePreviewResultVisitor;
        impl<'a> Visitor<'a> for NotificationRulePreviewResultVisitor {
            type Value = NotificationRulePreviewResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut notification_status: Option<
                    crate::datadogV2::model::NotificationRulePreviewNotificationStatus,
                > = None;
                let mut rule_type: Option<crate::datadogV2::model::RuleTypesItems> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "notification_status" => {
                            notification_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _notification_status) = notification_status {
                                match _notification_status {
                                    crate::datadogV2::model::NotificationRulePreviewNotificationStatus::UnparsedObject(_notification_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rule_type" => {
                            rule_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rule_type) = rule_type {
                                match _rule_type {
                                    crate::datadogV2::model::RuleTypesItems::UnparsedObject(
                                        _rule_type,
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
                let notification_status = notification_status
                    .ok_or_else(|| M::Error::missing_field("notification_status"))?;
                let rule_type = rule_type.ok_or_else(|| M::Error::missing_field("rule_type"))?;

                let content = NotificationRulePreviewResult {
                    notification_status,
                    rule_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotificationRulePreviewResultVisitor)
    }
}

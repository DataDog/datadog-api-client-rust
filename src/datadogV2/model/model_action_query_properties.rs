// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The properties of the action query.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionQueryProperties {
    /// Whether to run this query. If specified, the query will only run if this condition evaluates to `true` in JavaScript and all other conditions are also met.
    #[serde(rename = "condition")]
    pub condition: Option<crate::datadogV2::model::ActionQueryCondition>,
    /// The minimum time in milliseconds that must pass before the query can be triggered again. This is useful for preventing accidental double-clicks from triggering the query multiple times.
    #[serde(rename = "debounceInMs")]
    pub debounce_in_ms: Option<crate::datadogV2::model::ActionQueryDebounceInMs>,
    /// The mocked outputs of the action query. This is useful for testing the app without actually running the action.
    #[serde(rename = "mockedOutputs")]
    pub mocked_outputs: Option<crate::datadogV2::model::ActionQueryMockedOutputs>,
    /// Determines when this query is executed. If set to `false`, the query will run when the app loads and whenever any query arguments change. If set to `true`, the query will only run when manually triggered from elsewhere in the app.
    #[serde(rename = "onlyTriggerManually")]
    pub only_trigger_manually: Option<crate::datadogV2::model::ActionQueryOnlyTriggerManually>,
    /// The post-query transformation function, which is a JavaScript function that changes the query's `.outputs` property after the query's execution.
    #[serde(rename = "outputs")]
    pub outputs: Option<String>,
    /// If specified, the app will poll the query at the specified interval in milliseconds. The minimum polling interval is 15 seconds. The query will only poll when the app's browser tab is active.
    #[serde(rename = "pollingIntervalInMs")]
    pub polling_interval_in_ms: Option<crate::datadogV2::model::ActionQueryPollingIntervalInMs>,
    /// Whether to prompt the user to confirm this query before it runs.
    #[serde(rename = "requiresConfirmation")]
    pub requires_confirmation: Option<crate::datadogV2::model::ActionQueryRequiresConfirmation>,
    /// Whether to display a toast to the user when the query returns an error.
    #[serde(rename = "showToastOnError")]
    pub show_toast_on_error: Option<crate::datadogV2::model::ActionQueryShowToastOnError>,
    /// The definition of the action query.
    #[serde(rename = "spec")]
    pub spec: crate::datadogV2::model::ActionQuerySpec,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ActionQueryProperties {
    pub fn new(spec: crate::datadogV2::model::ActionQuerySpec) -> ActionQueryProperties {
        ActionQueryProperties {
            condition: None,
            debounce_in_ms: None,
            mocked_outputs: None,
            only_trigger_manually: None,
            outputs: None,
            polling_interval_in_ms: None,
            requires_confirmation: None,
            show_toast_on_error: None,
            spec,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn condition(mut self, value: crate::datadogV2::model::ActionQueryCondition) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn debounce_in_ms(
        mut self,
        value: crate::datadogV2::model::ActionQueryDebounceInMs,
    ) -> Self {
        self.debounce_in_ms = Some(value);
        self
    }

    pub fn mocked_outputs(
        mut self,
        value: crate::datadogV2::model::ActionQueryMockedOutputs,
    ) -> Self {
        self.mocked_outputs = Some(value);
        self
    }

    pub fn only_trigger_manually(
        mut self,
        value: crate::datadogV2::model::ActionQueryOnlyTriggerManually,
    ) -> Self {
        self.only_trigger_manually = Some(value);
        self
    }

    pub fn outputs(mut self, value: String) -> Self {
        self.outputs = Some(value);
        self
    }

    pub fn polling_interval_in_ms(
        mut self,
        value: crate::datadogV2::model::ActionQueryPollingIntervalInMs,
    ) -> Self {
        self.polling_interval_in_ms = Some(value);
        self
    }

    pub fn requires_confirmation(
        mut self,
        value: crate::datadogV2::model::ActionQueryRequiresConfirmation,
    ) -> Self {
        self.requires_confirmation = Some(value);
        self
    }

    pub fn show_toast_on_error(
        mut self,
        value: crate::datadogV2::model::ActionQueryShowToastOnError,
    ) -> Self {
        self.show_toast_on_error = Some(value);
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

impl<'de> Deserialize<'de> for ActionQueryProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionQueryPropertiesVisitor;
        impl<'a> Visitor<'a> for ActionQueryPropertiesVisitor {
            type Value = ActionQueryProperties;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut condition: Option<crate::datadogV2::model::ActionQueryCondition> = None;
                let mut debounce_in_ms: Option<crate::datadogV2::model::ActionQueryDebounceInMs> =
                    None;
                let mut mocked_outputs: Option<crate::datadogV2::model::ActionQueryMockedOutputs> =
                    None;
                let mut only_trigger_manually: Option<
                    crate::datadogV2::model::ActionQueryOnlyTriggerManually,
                > = None;
                let mut outputs: Option<String> = None;
                let mut polling_interval_in_ms: Option<
                    crate::datadogV2::model::ActionQueryPollingIntervalInMs,
                > = None;
                let mut requires_confirmation: Option<
                    crate::datadogV2::model::ActionQueryRequiresConfirmation,
                > = None;
                let mut show_toast_on_error: Option<
                    crate::datadogV2::model::ActionQueryShowToastOnError,
                > = None;
                let mut spec: Option<crate::datadogV2::model::ActionQuerySpec> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "condition" => {
                            if v.is_null() {
                                continue;
                            }
                            condition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _condition) = condition {
                                match _condition {
                                    crate::datadogV2::model::ActionQueryCondition::UnparsedObject(_condition) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "debounceInMs" => {
                            if v.is_null() {
                                continue;
                            }
                            debounce_in_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _debounce_in_ms) = debounce_in_ms {
                                match _debounce_in_ms {
                                    crate::datadogV2::model::ActionQueryDebounceInMs::UnparsedObject(_debounce_in_ms) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "mockedOutputs" => {
                            if v.is_null() {
                                continue;
                            }
                            mocked_outputs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mocked_outputs) = mocked_outputs {
                                match _mocked_outputs {
                                    crate::datadogV2::model::ActionQueryMockedOutputs::UnparsedObject(_mocked_outputs) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "onlyTriggerManually" => {
                            if v.is_null() {
                                continue;
                            }
                            only_trigger_manually =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _only_trigger_manually) = only_trigger_manually {
                                match _only_trigger_manually {
                                    crate::datadogV2::model::ActionQueryOnlyTriggerManually::UnparsedObject(_only_trigger_manually) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "outputs" => {
                            if v.is_null() {
                                continue;
                            }
                            outputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pollingIntervalInMs" => {
                            if v.is_null() {
                                continue;
                            }
                            polling_interval_in_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _polling_interval_in_ms) = polling_interval_in_ms {
                                match _polling_interval_in_ms {
                                    crate::datadogV2::model::ActionQueryPollingIntervalInMs::UnparsedObject(_polling_interval_in_ms) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "requiresConfirmation" => {
                            if v.is_null() {
                                continue;
                            }
                            requires_confirmation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _requires_confirmation) = requires_confirmation {
                                match _requires_confirmation {
                                    crate::datadogV2::model::ActionQueryRequiresConfirmation::UnparsedObject(_requires_confirmation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "showToastOnError" => {
                            if v.is_null() {
                                continue;
                            }
                            show_toast_on_error =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _show_toast_on_error) = show_toast_on_error {
                                match _show_toast_on_error {
                                    crate::datadogV2::model::ActionQueryShowToastOnError::UnparsedObject(_show_toast_on_error) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "spec" => {
                            spec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _spec) = spec {
                                match _spec {
                                    crate::datadogV2::model::ActionQuerySpec::UnparsedObject(
                                        _spec,
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
                let spec = spec.ok_or_else(|| M::Error::missing_field("spec"))?;

                let content = ActionQueryProperties {
                    condition,
                    debounce_in_ms,
                    mocked_outputs,
                    only_trigger_manually,
                    outputs,
                    polling_interval_in_ms,
                    requires_confirmation,
                    show_toast_on_error,
                    spec,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ActionQueryPropertiesVisitor)
    }
}

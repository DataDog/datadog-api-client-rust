// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Application Security details describing a service in a given environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityServiceAttributes {
    /// The Datadog Agent versions reporting for the service.
    #[serde(rename = "agent_versions")]
    pub agent_versions: Vec<String>,
    /// The application type of the service, such as `web` or `serverless`.
    #[serde(rename = "app_type")]
    pub app_type: String,
    /// Whether the service is compatible with Application Security Management (Threats).
    #[serde(rename = "asm_threat_compatible")]
    pub asm_threat_compatible: bool,
    /// The number of backend WAF events detected for the service.
    #[serde(rename = "backend_waf_event_count")]
    pub backend_waf_event_count: i64,
    /// The enabled business logic detection rules for the service.
    #[serde(rename = "business_logic")]
    pub business_logic: Vec<String>,
    /// Deprecated: a display color associated with the service in the UI.
    #[deprecated]
    #[serde(rename = "color")]
    pub color: String,
    /// The environment the service runs in.
    #[serde(rename = "env")]
    pub env: String,
    /// The number of Application Security events detected for the service.
    #[serde(rename = "event_count")]
    pub event_count: i64,
    /// Deprecated: the trend of Application Security events over time.
    #[deprecated]
    #[serde(rename = "event_trend")]
    pub event_trend: Vec<i64>,
    /// Whether Application Security Management (Threats) is enabled for the service.
    #[serde(rename = "has_appsec_enabled")]
    pub has_appsec_enabled: bool,
    /// Deprecated: the number of hits for the service.
    #[deprecated]
    #[serde(rename = "hits")]
    pub hits: i64,
    /// Whether Interactive Application Security Testing (IAST) is enabled for the service.
    #[serde(rename = "iast_product_activation")]
    pub iast_product_activation: bool,
    /// The Interactive Application Security Testing (IAST) compatibility status of the service.
    #[serde(rename = "iast_product_compatibility")]
    pub iast_product_compatibility: String,
    /// The reasons explaining the Interactive Application Security Testing (IAST) compatibility status.
    #[serde(rename = "iast_product_compatibility_reasons")]
    pub iast_product_compatibility_reasons: Vec<String>,
    /// The programming languages detected for the service.
    #[serde(rename = "languages")]
    pub languages: Vec<String>,
    /// The Unix timestamp, in seconds, of the last ingested span for the service.
    #[serde(rename = "last_ingested_spans")]
    pub last_ingested_spans: i64,
    /// The Remote Configuration capabilities reported by the service.
    #[serde(rename = "rc_capabilities")]
    pub rc_capabilities: Vec<String>,
    /// The recommended business logic detection rules for the service.
    #[serde(rename = "recommended_business_logic")]
    pub recommended_business_logic: Vec<String>,
    /// Whether Software Composition Analysis (SCA) is enabled for the service.
    #[serde(rename = "risk_product_activation")]
    pub risk_product_activation: bool,
    /// The Software Composition Analysis (SCA) compatibility status of the service.
    #[serde(rename = "risk_product_compatibility")]
    pub risk_product_compatibility: String,
    /// The reasons explaining the Software Composition Analysis (SCA) compatibility status.
    #[serde(rename = "risk_product_compatibility_reasons")]
    pub risk_product_compatibility_reasons: Vec<String>,
    /// The WAF rules versions applied to the service.
    #[serde(rename = "rules_version")]
    pub rules_version: Vec<String>,
    /// The name of the service.
    #[serde(rename = "service")]
    pub service: String,
    /// Deprecated: the number of security signals for the service.
    #[deprecated]
    #[serde(rename = "signal_count")]
    pub signal_count: i64,
    /// Deprecated: the trend of security signals over time.
    #[deprecated]
    #[serde(rename = "signal_trend")]
    pub signal_trend: Vec<i64>,
    /// The data sources that contributed information about the service.
    #[serde(rename = "source")]
    pub source: Vec<String>,
    /// The teams that own the service.
    #[serde(rename = "teams")]
    pub teams: Vec<String>,
    /// The Datadog tracing library versions reporting for the service.
    #[serde(rename = "tracer_versions")]
    pub tracer_versions: Vec<String>,
    /// The Vulnerability Management activation status of the service.
    #[serde(rename = "vm-activation")]
    pub vm_activation: String,
    /// Deprecated: the number of critical-severity vulnerabilities for the service.
    #[deprecated]
    #[serde(rename = "vuln_critical_count")]
    pub vuln_critical_count: i64,
    /// Deprecated: the number of high-severity vulnerabilities for the service.
    #[deprecated]
    #[serde(rename = "vuln_high_count")]
    pub vuln_high_count: i64,
    /// The total number of services available without applying the service filter.
    #[serde(rename = "without_filter_services")]
    pub without_filter_services: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityServiceAttributes {
    pub fn new(
        agent_versions: Vec<String>,
        app_type: String,
        asm_threat_compatible: bool,
        backend_waf_event_count: i64,
        business_logic: Vec<String>,
        color: String,
        env: String,
        event_count: i64,
        event_trend: Vec<i64>,
        has_appsec_enabled: bool,
        hits: i64,
        iast_product_activation: bool,
        iast_product_compatibility: String,
        iast_product_compatibility_reasons: Vec<String>,
        languages: Vec<String>,
        last_ingested_spans: i64,
        rc_capabilities: Vec<String>,
        recommended_business_logic: Vec<String>,
        risk_product_activation: bool,
        risk_product_compatibility: String,
        risk_product_compatibility_reasons: Vec<String>,
        rules_version: Vec<String>,
        service: String,
        signal_count: i64,
        signal_trend: Vec<i64>,
        source: Vec<String>,
        teams: Vec<String>,
        tracer_versions: Vec<String>,
        vm_activation: String,
        vuln_critical_count: i64,
        vuln_high_count: i64,
        without_filter_services: i64,
    ) -> ApplicationSecurityServiceAttributes {
        #[allow(deprecated)]
        ApplicationSecurityServiceAttributes {
            agent_versions,
            app_type,
            asm_threat_compatible,
            backend_waf_event_count,
            business_logic,
            color,
            env,
            event_count,
            event_trend,
            has_appsec_enabled,
            hits,
            iast_product_activation,
            iast_product_compatibility,
            iast_product_compatibility_reasons,
            languages,
            last_ingested_spans,
            rc_capabilities,
            recommended_business_logic,
            risk_product_activation,
            risk_product_compatibility,
            risk_product_compatibility_reasons,
            rules_version,
            service,
            signal_count,
            signal_trend,
            source,
            teams,
            tracer_versions,
            vm_activation,
            vuln_critical_count,
            vuln_high_count,
            without_filter_services,
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

impl<'de> Deserialize<'de> for ApplicationSecurityServiceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityServiceAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityServiceAttributesVisitor {
            type Value = ApplicationSecurityServiceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agent_versions: Option<Vec<String>> = None;
                let mut app_type: Option<String> = None;
                let mut asm_threat_compatible: Option<bool> = None;
                let mut backend_waf_event_count: Option<i64> = None;
                let mut business_logic: Option<Vec<String>> = None;
                let mut color: Option<String> = None;
                let mut env: Option<String> = None;
                let mut event_count: Option<i64> = None;
                let mut event_trend: Option<Vec<i64>> = None;
                let mut has_appsec_enabled: Option<bool> = None;
                let mut hits: Option<i64> = None;
                let mut iast_product_activation: Option<bool> = None;
                let mut iast_product_compatibility: Option<String> = None;
                let mut iast_product_compatibility_reasons: Option<Vec<String>> = None;
                let mut languages: Option<Vec<String>> = None;
                let mut last_ingested_spans: Option<i64> = None;
                let mut rc_capabilities: Option<Vec<String>> = None;
                let mut recommended_business_logic: Option<Vec<String>> = None;
                let mut risk_product_activation: Option<bool> = None;
                let mut risk_product_compatibility: Option<String> = None;
                let mut risk_product_compatibility_reasons: Option<Vec<String>> = None;
                let mut rules_version: Option<Vec<String>> = None;
                let mut service: Option<String> = None;
                let mut signal_count: Option<i64> = None;
                let mut signal_trend: Option<Vec<i64>> = None;
                let mut source: Option<Vec<String>> = None;
                let mut teams: Option<Vec<String>> = None;
                let mut tracer_versions: Option<Vec<String>> = None;
                let mut vm_activation: Option<String> = None;
                let mut vuln_critical_count: Option<i64> = None;
                let mut vuln_high_count: Option<i64> = None;
                let mut without_filter_services: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agent_versions" => {
                            agent_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "app_type" => {
                            app_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "asm_threat_compatible" => {
                            asm_threat_compatible =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "backend_waf_event_count" => {
                            backend_waf_event_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "business_logic" => {
                            business_logic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "color" => {
                            color = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_count" => {
                            event_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_trend" => {
                            event_trend =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_appsec_enabled" => {
                            has_appsec_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "hits" => {
                            hits = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iast_product_activation" => {
                            iast_product_activation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iast_product_compatibility" => {
                            iast_product_compatibility =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "iast_product_compatibility_reasons" => {
                            iast_product_compatibility_reasons =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "languages" => {
                            languages = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_ingested_spans" => {
                            last_ingested_spans =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rc_capabilities" => {
                            rc_capabilities =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recommended_business_logic" => {
                            recommended_business_logic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk_product_activation" => {
                            risk_product_activation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk_product_compatibility" => {
                            risk_product_compatibility =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "risk_product_compatibility_reasons" => {
                            risk_product_compatibility_reasons =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules_version" => {
                            rules_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_count" => {
                            signal_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signal_trend" => {
                            signal_trend =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tracer_versions" => {
                            tracer_versions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vm-activation" => {
                            vm_activation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_critical_count" => {
                            vuln_critical_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vuln_high_count" => {
                            vuln_high_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "without_filter_services" => {
                            without_filter_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let agent_versions =
                    agent_versions.ok_or_else(|| M::Error::missing_field("agent_versions"))?;
                let app_type = app_type.ok_or_else(|| M::Error::missing_field("app_type"))?;
                let asm_threat_compatible = asm_threat_compatible
                    .ok_or_else(|| M::Error::missing_field("asm_threat_compatible"))?;
                let backend_waf_event_count = backend_waf_event_count
                    .ok_or_else(|| M::Error::missing_field("backend_waf_event_count"))?;
                let business_logic =
                    business_logic.ok_or_else(|| M::Error::missing_field("business_logic"))?;
                let color = color.ok_or_else(|| M::Error::missing_field("color"))?;
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;
                let event_count =
                    event_count.ok_or_else(|| M::Error::missing_field("event_count"))?;
                let event_trend =
                    event_trend.ok_or_else(|| M::Error::missing_field("event_trend"))?;
                let has_appsec_enabled = has_appsec_enabled
                    .ok_or_else(|| M::Error::missing_field("has_appsec_enabled"))?;
                let hits = hits.ok_or_else(|| M::Error::missing_field("hits"))?;
                let iast_product_activation = iast_product_activation
                    .ok_or_else(|| M::Error::missing_field("iast_product_activation"))?;
                let iast_product_compatibility = iast_product_compatibility
                    .ok_or_else(|| M::Error::missing_field("iast_product_compatibility"))?;
                let iast_product_compatibility_reasons = iast_product_compatibility_reasons
                    .ok_or_else(|| M::Error::missing_field("iast_product_compatibility_reasons"))?;
                let languages = languages.ok_or_else(|| M::Error::missing_field("languages"))?;
                let last_ingested_spans = last_ingested_spans
                    .ok_or_else(|| M::Error::missing_field("last_ingested_spans"))?;
                let rc_capabilities =
                    rc_capabilities.ok_or_else(|| M::Error::missing_field("rc_capabilities"))?;
                let recommended_business_logic = recommended_business_logic
                    .ok_or_else(|| M::Error::missing_field("recommended_business_logic"))?;
                let risk_product_activation = risk_product_activation
                    .ok_or_else(|| M::Error::missing_field("risk_product_activation"))?;
                let risk_product_compatibility = risk_product_compatibility
                    .ok_or_else(|| M::Error::missing_field("risk_product_compatibility"))?;
                let risk_product_compatibility_reasons = risk_product_compatibility_reasons
                    .ok_or_else(|| M::Error::missing_field("risk_product_compatibility_reasons"))?;
                let rules_version =
                    rules_version.ok_or_else(|| M::Error::missing_field("rules_version"))?;
                let service = service.ok_or_else(|| M::Error::missing_field("service"))?;
                let signal_count =
                    signal_count.ok_or_else(|| M::Error::missing_field("signal_count"))?;
                let signal_trend =
                    signal_trend.ok_or_else(|| M::Error::missing_field("signal_trend"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;
                let teams = teams.ok_or_else(|| M::Error::missing_field("teams"))?;
                let tracer_versions =
                    tracer_versions.ok_or_else(|| M::Error::missing_field("tracer_versions"))?;
                let vm_activation =
                    vm_activation.ok_or_else(|| M::Error::missing_field("vm_activation"))?;
                let vuln_critical_count = vuln_critical_count
                    .ok_or_else(|| M::Error::missing_field("vuln_critical_count"))?;
                let vuln_high_count =
                    vuln_high_count.ok_or_else(|| M::Error::missing_field("vuln_high_count"))?;
                let without_filter_services = without_filter_services
                    .ok_or_else(|| M::Error::missing_field("without_filter_services"))?;

                #[allow(deprecated)]
                let content = ApplicationSecurityServiceAttributes {
                    agent_versions,
                    app_type,
                    asm_threat_compatible,
                    backend_waf_event_count,
                    business_logic,
                    color,
                    env,
                    event_count,
                    event_trend,
                    has_appsec_enabled,
                    hits,
                    iast_product_activation,
                    iast_product_compatibility,
                    iast_product_compatibility_reasons,
                    languages,
                    last_ingested_spans,
                    rc_capabilities,
                    recommended_business_logic,
                    risk_product_activation,
                    risk_product_compatibility,
                    risk_product_compatibility_reasons,
                    rules_version,
                    service,
                    signal_count,
                    signal_trend,
                    source,
                    teams,
                    tracer_versions,
                    vm_activation,
                    vuln_critical_count,
                    vuln_high_count,
                    without_filter_services,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityServiceAttributesVisitor)
    }
}

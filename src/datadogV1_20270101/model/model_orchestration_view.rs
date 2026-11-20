// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Serialize, Deserialize, Serializer, Deserializer};


#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrchestrationView {
	SERVICE,
	CLUSTER,
	JOB,
	NODE,
	DEPLOYMENT,
	NAMESPACE,
	POD,
	GENERICWORKLOAD,
	INFERENCEWORKLOAD,
	REPLICASET,
	CRONJOB,
	DAEMONSET,
	STATEFULSET,
	PERSISTENTVOLUME,
	PERSISTENTVOLUMECLAIM,
	ROLE,
	ROLEBINDING,
	CLUSTERROLE,
	CLUSTERROLEBINDING,
	SERVICEACCOUNT,
	INGRESS,
	CONFIGMAP,
	VERTICALPODAUTOSCALER,
	HORIZONTALPODAUTOSCALER,
	CRD,
	CR,
	NETWORKPOLICY,
	PODDISRUPTIONBUDGET,
	ECSTASK,
	ECSTASKDEFINITION,
	ECSDAEMONTASKDEFINITION,
	ECSSERVICE,
	ECSDAEMON,
	ECSCLUSTER,
	ECSCONTAINERINSTANCE,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for OrchestrationView {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICE => String::from("service"),
            Self::CLUSTER => String::from("cluster"),
            Self::JOB => String::from("job"),
            Self::NODE => String::from("node"),
            Self::DEPLOYMENT => String::from("deployment"),
            Self::NAMESPACE => String::from("namespace"),
            Self::POD => String::from("pod"),
            Self::GENERICWORKLOAD => String::from("genericWorkload"),
            Self::INFERENCEWORKLOAD => String::from("inferenceWorkload"),
            Self::REPLICASET => String::from("replicaSet"),
            Self::CRONJOB => String::from("cronJob"),
            Self::DAEMONSET => String::from("daemonSet"),
            Self::STATEFULSET => String::from("statefulSet"),
            Self::PERSISTENTVOLUME => String::from("persistentVolume"),
            Self::PERSISTENTVOLUMECLAIM => String::from("persistentVolumeClaim"),
            Self::ROLE => String::from("role"),
            Self::ROLEBINDING => String::from("roleBinding"),
            Self::CLUSTERROLE => String::from("clusterRole"),
            Self::CLUSTERROLEBINDING => String::from("clusterRoleBinding"),
            Self::SERVICEACCOUNT => String::from("serviceAccount"),
            Self::INGRESS => String::from("ingress"),
            Self::CONFIGMAP => String::from("configMap"),
            Self::VERTICALPODAUTOSCALER => String::from("verticalPodAutoscaler"),
            Self::HORIZONTALPODAUTOSCALER => String::from("horizontalPodAutoscaler"),
            Self::CRD => String::from("crd"),
            Self::CR => String::from("cr"),
            Self::NETWORKPOLICY => String::from("networkPolicy"),
            Self::PODDISRUPTIONBUDGET => String::from("podDisruptionBudget"),
            Self::ECSTASK => String::from("ecsTask"),
            Self::ECSTASKDEFINITION => String::from("ecsTaskDefinition"),
            Self::ECSDAEMONTASKDEFINITION => String::from("ecsDaemonTaskDefinition"),
            Self::ECSSERVICE => String::from("ecsService"),
            Self::ECSDAEMON => String::from("ecsDaemon"),
            Self::ECSCLUSTER => String::from("ecsCluster"),
            Self::ECSCONTAINERINSTANCE => String::from("ecsContainerInstance"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for OrchestrationView {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for OrchestrationView {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {"service" => Self::SERVICE,"cluster" => Self::CLUSTER,"job" => Self::JOB,"node" => Self::NODE,"deployment" => Self::DEPLOYMENT,"namespace" => Self::NAMESPACE,"pod" => Self::POD,"genericWorkload" => Self::GENERICWORKLOAD,"inferenceWorkload" => Self::INFERENCEWORKLOAD,"replicaSet" => Self::REPLICASET,"cronJob" => Self::CRONJOB,"daemonSet" => Self::DAEMONSET,"statefulSet" => Self::STATEFULSET,"persistentVolume" => Self::PERSISTENTVOLUME,"persistentVolumeClaim" => Self::PERSISTENTVOLUMECLAIM,"role" => Self::ROLE,"roleBinding" => Self::ROLEBINDING,"clusterRole" => Self::CLUSTERROLE,"clusterRoleBinding" => Self::CLUSTERROLEBINDING,"serviceAccount" => Self::SERVICEACCOUNT,"ingress" => Self::INGRESS,"configMap" => Self::CONFIGMAP,"verticalPodAutoscaler" => Self::VERTICALPODAUTOSCALER,"horizontalPodAutoscaler" => Self::HORIZONTALPODAUTOSCALER,"crd" => Self::CRD,"cr" => Self::CR,"networkPolicy" => Self::NETWORKPOLICY,"podDisruptionBudget" => Self::PODDISRUPTIONBUDGET,"ecsTask" => Self::ECSTASK,"ecsTaskDefinition" => Self::ECSTASKDEFINITION,"ecsDaemonTaskDefinition" => Self::ECSDAEMONTASKDEFINITION,"ecsService" => Self::ECSSERVICE,"ecsDaemon" => Self::ECSDAEMON,"ecsCluster" => Self::ECSCLUSTER,"ecsContainerInstance" => Self::ECSCONTAINERINSTANCE,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject { value: serde_json::Value::String(s.into()) }),
        })
    }
}

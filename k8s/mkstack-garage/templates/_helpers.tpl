{{/*
Expand the name of the chart.
*/}}
{{- define "mkstack-garage.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "mkstack-garage.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Chart label.
*/}}
{{- define "mkstack-garage.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels.
*/}}
{{- define "mkstack-garage.labels" -}}
helm.sh/chart: {{ include "mkstack-garage.chart" . }}
{{ include "mkstack-garage.selectorLabels" . }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels.
*/}}
{{- define "mkstack-garage.selectorLabels" -}}
app.kubernetes.io/name: {{ include "mkstack-garage.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
RPC headless service name.
*/}}
{{- define "mkstack-garage.rpcServiceName" -}}
{{- printf "%s-rpc" (include "mkstack-garage.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Secret name for rpc_secret.
*/}}
{{- define "mkstack-garage.rpcSecretName" -}}
{{- printf "%s-rpc" (include "mkstack-garage.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Secret name for admin/metrics tokens.
*/}}
{{- define "mkstack-garage.adminSecretName" -}}
{{- printf "%s-admin" (include "mkstack-garage.fullname" .) | trunc 63 | trimSuffix "-" }}
{{- end }}

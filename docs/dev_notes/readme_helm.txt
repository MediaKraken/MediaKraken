# udpate value after install  namespace then release
helm get values -n monitoring monitoring > values.yaml
 
Then install using those values and override with your single override.
 
helm upgrade --install --atomic --wait -n monitoring monitoring prometheus-community/kube-prometheus-stack \
  --values values.yaml \
  --set prometheus.prometheusSpec.podMonitorSelectorNilUsesHelmValues=false




 
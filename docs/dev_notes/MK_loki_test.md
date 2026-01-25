# from worker node
curl -X POST "http://loki-headless.monitoring.svc.mkcluster.local:3100/loki/api/v1/push" \
  -H "Content-Type: application/json" \
  -d '{
    "streams": [{
      "stream": {"mediakraken": "storage-test"},
      "values": [["'"$(date +%s)"'000000000", "storage test message"]]
    }]
  }'

# Query the log
curl -G "http://loki-headless.monitoring.svc.mkcluster.local:3100/loki/api/v1/query_range" \
  --data-urlencode 'query={mediakraken="storage-test"}' \
  --data-urlencode 'limit=10'



apiVersion: v1
kind: ConfigMap
metadata:
  name: grafana-datasources
  namespace: grafana
  labels:
    grafana_datasource: "1"
data:
  loki.yaml: |
    apiVersion: 1
    datasources:
      - name: Loki
        type: loki
        access: proxy
        url: http://loki-gateway.loki.svc.cluster.local
        isDefault: true
        jsonData:
          maxLines: 1000
          httpHeaderName1: X-Scope-OrgID
        secureJsonData:
          httpHeaderValue1: default
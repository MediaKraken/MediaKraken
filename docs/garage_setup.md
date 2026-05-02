apiVersion: v1
kind: Namespace
metadata:
  name: garage-s3

**********************************************


kubectl -n garage-s3 create secret generic garage-secrets \
  --from-literal=rpc-secret="$(openssl rand -hex 32)" \
  --from-literal=admin-token="$(openssl rand -base64 32)"

<!-- # Generate a secret
export RPC_SECRET=$(openssl rand -hex 32)

# Create the Kubernetes secret
kubectl create secret generic garage-rpc-secret --from-literal=rpc-secret=$RPC_SECRET -n garage-s3

# Add an admin token (use a secure string)
kubectl patch secret garage-rpc-secret -p '{"data":{"admin-token":"'$(echo -n "your-super-garage-admin-token" | base64)'"}}' -n garage-s3 -->

***************************************************

---
kind: StorageClass
apiVersion: storage.k8s.io/v1
metadata:
  name: topolvm-xfs-data
  namespace: garage-s3
provisioner: topolvm.io
parameters:
  "csi.storage.k8s.io/fstype": "xfs"
  "topolvm.cybozu.com/device-class": "vgk8sgarage"
volumeBindingMode: WaitForFirstConsumer
allowVolumeExpansion: true
---
kind: StorageClass
apiVersion: storage.k8s.io/v1
metadata:
  name: topolvm-xfs-meta
  namespace: garage-s3
provisioner: topolvm.io
parameters:
  "csi.storage.k8s.io/fstype": "xfs"
  "topolvm.cybozu.com/device-class": "vgk8sgarage"
volumeBindingMode: WaitForFirstConsumer
allowVolumeExpansion: true

************************************************************

<!-- apiVersion: v1
kind: ConfigMap
metadata:
  name: garage-config
  namespace: garage-s3
data:
  garage.toml: |
    metadata_dir = "/var/lib/garage/meta"
    data_dir = "/var/lib/garage/data"
    replication_factor = 1
    rpc_secret_file = "/etc/garage/rpc-secret"

    [rpc]
    rpc_bind_addr = "[::]:3901"
    rpc_public_addr = "garage.garage-s3.svc.cluster.local:3901"
    discovery_dns = "garage.garage-s3.svc.cluster.local"
    discovery_dns_port = 3901

    [s3_api]
    api_bind_addr = "[::]:3900"
    s3_region = "garage"

    [s3_web]
    bind_addr = "[::]:3902"
    root_domain = ".s3.garage.localhost"

    [admin]
    api_bind_addr = "0.0.0.0:3903"
    admin_token = "your-super-garage-admin-token" -->

********************************************************

apiVersion: v1
kind: Service
metadata:
  name: garage
  namespace: garage-s3
spec:
  clusterIP: None
  publishNotReadyAddresses: true
  selector:
    app: garage
  ports:
    - name: s3
      port: 3900
      targetPort: 3900
    - name: rpc
      port: 3901
      targetPort: 3901
    - name: web
      port: 3902
      targetPort: 3902
    - name: admin
      port: 3903
      targetPort: 3903
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: garage
  namespace: garage-s3
spec:
  serviceName: garage
  replicas: 3
  selector:
    matchLabels:
      app: garage
  template:
    metadata:
      labels:
        app: garage
    spec:
      initContainers:
        - name: render-config
          image: busybox:1.36
          command:
            - sh
            - -c
            - |
              cat > /work/garage.toml <<EOF
              metadata_dir = "/var/lib/garage/meta"
              data_dir = "/var/lib/garage/data"

              replication_factor = 1
              consistency_mode = "consistent"

              rpc_bind_addr = "[::]:3901"
              rpc_public_addr = "${POD_IP}:3901"
              rpc_secret_file = "/etc/garage/rpc-secret"

              [kubernetes_discovery]
              namespace = "garage-s3"
              service_name = "garage"

              [s3_api]
              api_bind_addr = "[::]:3900"
              s3_region = "garage"
              root_domain = ".s3.garage.localhost"

              [s3_web]
              bind_addr = "[::]:3902"
              root_domain = ".web.garage.localhost"
              index = "index.html"

              [admin]
              api_bind_addr = "[::]:3903"
              admin_token_file = "/etc/garage/admin-token"
              EOF
          env:
            - name: POD_NAME
              valueFrom:
                fieldRef:
                  fieldPath: metadata.name
            - name: POD_IP
              valueFrom:
                fieldRef:
                    fieldPath: status.podIP
          volumeMounts:
            - name: rendered-config
              mountPath: /work

      containers:
        - name: garage
          image: dxflrs/garage:v2.3.0
          command:
            - /garage
          args:
            - -c
            - /etc/garage/garage.toml
            - server
          ports:
            - name: s3
              containerPort: 3900
            - name: rpc
              containerPort: 3901
            - name: web
              containerPort: 3902
            - name: admin
              containerPort: 3903
          volumeMounts:
            - name: rendered-config
              mountPath: /etc/garage/garage.toml
              subPath: garage.toml
              readOnly: true
            - name: garage-secrets
              mountPath: /etc/garage/rpc-secret
              subPath: rpc-secret
              readOnly: true
            - name: garage-secrets
              mountPath: /etc/garage/admin-token
              subPath: admin-token
              readOnly: true
            - name: garage-data
              mountPath: /var/lib/garage
      volumes:
        - name: rendered-config
          emptyDir: {}
        - name: garage-secrets
          secret:
            secretName: garage-secrets
            defaultMode: 0600
  # This section defines 'meta' and 'data' using TopoLVM
  volumeClaimTemplates:
  - metadata:
      name: garage-meta
    spec:
      accessModes: [ "ReadWriteOnce" ]
      storageClassName: "topolvm-xfs-meta"
      resources:
        requests:
          storage: 500Gi
  - metadata:
      name: garage-data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      storageClassName: "topolvm-xfs-data"
      resources:
        requests:
          storage: 9000Gi
<!-- ---
apiVersion: v1
kind: Service
metadata:
  name: garage-admin-api
  namespace: garage-s3
spec:
  selector:
    app: garage
  ports:
    - name: admin-api
      port: 3903
      targetPort: 3903 -->

********************************************************

apiVersion: apps/v1
kind: Deployment
metadata:
  name: garage-webui
  namespace: garage-s3
spec:
  replicas: 1
  selector:
    matchLabels:
      app: garage-webui
  template:
    metadata:
      labels:
        app: garage-webui
    spec:
      containers:
      - name: webui
        image: khairul169/garage-webui:latest
        ports:
        - containerPort: 8080
        env:
        - name: GARAGE_ADMIN_URL
          value: "http://garage-admin-api:3903"
        - name: GARAGE_ADMIN_TOKEN
          valueFrom:
            secretKeyRef:
              name: garage-rpc-secret
              key: admin-token
---
apiVersion: v1
kind: Service
metadata:
  name: garage-webui
  namespace: garage-s3
spec:
  type: LoadBalancer # Change to NodePort or LoadBalancer for external access
  selector:
    app: garage-webui
  ports:
    - port: 80
      targetPort: 8080
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: garage-webui-ingress
  namespace: garage-s3
  annotations:
    # Use this if you use NGINX Ingress Controller
    kubernetes.io/ingress.class: "nginx"
    # Use this if you have cert-manager installed for automated SSL
    cert-manager.io/cluster-issuer: "letsencrypt-garageui"
spec:
  tls:
  - hosts:
    - garage-ui.mediakraken.media
    secretName: garage-webui-tls
  rules:
  - host: garage-ui.mediakraken.media
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: garage-webui
            port:
              number: 80

***********************************

# this lists all 3 nodes and show the IDS
kubectl exec -it garage-0 -n garage -- /garage status

<!-- kubectl exec -it garage-0 -n garage-s3 -- /garage node connect $(kubectl get pod garage-1 -n garage-s3 -o jsonpath='{.status.podIP}'):3901
kubectl exec -it garage-0 -n garage-s3 -- /garage node connect $(kubectl get pod garage-2 -n garage-s3 -o jsonpath='{.status.podIP}'):3901


kubectl get pod garage-0 -n garage-s3 -o jsonpath='{.status.podIP}'
kubectl exec -it garage-0 -n garage-s3 -- /garage node connect 10.233.93.105:3901
kubectl exec -it garage-0 -n garage-s3 -- /garage node connect 10.233.95.111:3901 -->



kubectl port-forward -n garage-s3 garage-0 3903:3903

curl -H "Authorization: Bearer Q6BviePNmOL5ERWfC59nevT+4cweldllHypyWX04iwo=" \
     -s http://127.0.0.1:3903/v2/GetClusterStatus | python3 -m json.tool

<!-- kubectl -n garage-s3 logs garage-0 | grep 'Node ID'
kubectl -n garage-s3 logs garage-1 | grep 'Node ID'
kubectl -n garage-s3 logs garage-2 | grep 'Node ID' -->

# run from a curl
kubectl -n garage-s3 run curltest --rm -it \
  --image=curlimages/curl:latest \
  --restart=Never -- sh

curl -H "Authorization: Bearer Q6BviePNmOL5ERWfC59nevT+4cweldllHypyWX04iwo=" \
  -H "Content-Type: application/json" \
  -X POST http://127.0.0.1:3903/v2/ConnectClusterNodes \
  -d '[
    "a2b735068c0c782c@10.233.95.123:3901",
    "349f65588ab9a21a@10.233.93.124:3901"
  ]'

<!-- curl -H "Authorization: Bearer Q6BviePNmOL5ERWfC59nevT+4cweldllHypyWX04iwo=" \
     -H "Content-Type: application/json" \
     -X POST http://127.0.0.1:3903/v2/ConnectClusterNodes \
     -d '["garage-1.garage.garage-s3:3901", "garage-2.garage.garage-s3:3901"]' -->

*******************************************************


kubectl -n garage exec -it garage-0 -- /garage status
kubectl -n garage exec -it garage-0 -- /garage layout assign -z dc1 -c 9000G 2241570881cc8da5
kubectl -n garage exec -it garage-0 -- /garage layout assign -z dc1 -c 9000G e73016f1b4a1aab9
kubectl -n garage exec -it garage-0 -- /garage layout assign -z dc1 -c 9000G f62b51319104adfc
kubectl -n garage exec -it garage-0 -- /garage layout apply --version 1


# Replace the placeholders before applying:
#   rpc_secret    : openssl rand -hex 32
#   admin_token   : openssl rand -base64 32
#   metrics_token : openssl rand -base64 32


kubectl -n garage exec -it garage-0 -- /garage key create mediakraken-key

kubectl -n garage exec -it garage-0 -- /garage bucket create mediakraken

kubectl -n garage exec -it garage-0 -- /garage bucket allow --read --write --owner mediakraken --key mediakraken-key

kubectl create secret generic garage-s3-credentials \
  --from-literal=ACCESS_KEY_ID=GK1c9e6a0782746b2aa6c0a68a \
  --from-literal=ACCESS_SECRET_KEY=eceb5f36e68ca3ac55f57ed563f36cb3d6ceca8b5401a4aed650162b75aba099 \
  --from-literal=AWS_REGION=garage \
  -n mediakraken
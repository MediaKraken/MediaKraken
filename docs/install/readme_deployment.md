
############### cnpg stuff
# gen password  
## echo -n 'metaman' | base64
### bWV0YW1hbg==

# for the backup stuff......
kubectl create secret generic s3-backup-creds \
  --from-literal=ACCESS_KEY_ID=GKd0ac807197ae9636872ceed1 \
  --from-literal=ACCESS_SECRET_KEY=5442fe4af549468b0196d2bfd4d69ceb11c7faa42c001b56e0d70ad611cefa29 \
  --from-literal=AWS_REGION=garage \
  -n cnpg-system

# set db secret
kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='592d-9b85-41da-9d1' -n mediakraken

<!-- # stuff below to do
# stuff to add to MK
https://operatorhub.io/operator/elastic-cloud-eck
Elastic Cloud on Kubernetes (ECK) is the official operator by Elastic for automating the deployment, provisioning, management, and orchestration of Elasticsearch, Kibana, APM Server, Beats, Enterprise Search, Elastic Agent, Elastic Maps Server, and Logstash on Kubernetes.

# stuff to add to DEV stack CI/CD/etc

# misc stuff to play with
https://artifacthub.io/packages/helm/cloudhippie/ansible-semaphore
Modern and open-source alternative to AWX/Tower
https://semaphoreui.com/

https://artifacthub.io/packages/helm/geek-cookbook/network-ups-tools
Network UPS Tools is a collection of programs which provide a common interface for monitoring and administering UPS, PDU and SCD hardware.

https://artifacthub.io/packages/helm/geek-cookbook/owncast
live video and web chat server

https://artifacthub.io/packages/helm/agones/agones
Host, Run and Scale dedicated game servers on Kubernetes

https://artifacthub.io/packages/helm/fmjstudios/vaultwarden
Unofficial Bitwarden compatible server

https://artifacthub.io/packages/helm/crowdsec/crowdsec
CrowdSec - Real-time & crowdsourced protection against aggressive IPs

https://artifacthub.io/packages/helm/uptime-kuma/uptime-kuma
A self-hosted Monitoring tool like "Uptime-Robot".

https://artifacthub.io/packages/helm/oneuptime/oneuptime
OneUptime is a comprehensive solution for monitoring and managing your online services.

https://artifacthub.io/packages/helm/crystalnet/romm
RomM (Rom Manager) is a web based retro roms manager integrated with IGDB.

https://artifacthub.io/packages/helm/si-gitops/nut-exporter
Installs NUT exporter in Kubernetes

on dev
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

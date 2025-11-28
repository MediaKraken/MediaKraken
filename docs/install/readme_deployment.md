
# configure stackgres database cluster
## get the ui password
kubectl get secret -n stackgres stackgres-restapi-admin --template '{{ printf "password = %s\n" (.data.clearPassword | base64decode) }}'

## create db cluster
https://mkstackgres.mediakraken.org
### log db server
mkdblogs
32gb space
### production profile
mkdbinstance
prodfile, 48GB, 12CPU
#### for now, set loadblanacer in cluster type
full to use!!!!!!!!!!
autovacuum_vacuum_cost_delay=2ms
max_connections=200
shared_buffers=12GB
effective_cache_size=36GB
maintenance_work_mem=2GB
checkpoint_segments=6
checkpoint_completion_target=0.9
checkpoint_timeout=15min
default_statistics_target=100
random_page_cost=1
effective_io_concurrency=200
work_mem = 116508kB
huge_pages=try
min_wal_size=1GB
max_wal_size=4GB
max_worker_processes=10
max_parallel_workers_per_gather=4
max_parallel_workers=10
max_parallel_maintenance_workers=4
shared_preload_libraries=pg_stat_statements,auto_explain,timescaledb
wal_buffers=16MB
log_min_duration_statement=1s
log_temp_files=0kB
tcp_keepalives_count=9
tcp_keepalives_idle=5min
tcp_keepalives_interval=75s
track_activity_query_size=4kB

in the pgcluster config (https://mkstackgres.mediakraken.org/admin/stackgres/sgpgconfig/postgres-16-generated-from-default-1741405922020/edit)
shared_preload_libraries=pg_stat_statements,auto_explain,timescaledb
need to restart
### setup cluster
stackgres cluster/custom
mkdatabase
2 instances
version 17 (latest)
no ssl
500gb storage, nfs8k class (local option now)
monitoring and prometheus option
#### new cluster extentions
pgcrypto
pg_stat_statements
pg_trgm
timescaledb
vector
vectorscale

# get pg password
kubectl get secret mkdbinstance --namespace=stackgres --template '{{ printf "%s" (index .data "superuser-password" | base64decode) }}'


############### cnpg stuff
# gen password  
## echo -n 'metaman' | base64
### bWV0YW1hbg==

# for the backup stuff......
kubectl create secret generic aws-creds \
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

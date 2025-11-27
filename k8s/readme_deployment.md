
```virt-customize -a debian-13-genericcloud-amd64-20251006-2257.qcow2 --install qemu-guest-agent --run-command 'systemctl enable qemu-guest-agent.service'```

```virt-customize -a debian-13-genericcloud-amd64-20251006-2257.qcow2 --run-command "echo -n > /etc/machine-id"```

```qm create 9001 --name "debian-13-cloudinit-template-mk" --memory 2048 --cores 2 --net0 virtio,bridge=vmbr0 && qm importdisk 9001 debian-13-genericcloud-amd64-20251006-2257.qcow2 local-lvm && qm set 9001 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-9001-disk-0 && qm set 9001 --boot c --bootdisk scsi0 && qm set 9001 --ide2 local-lvm:cloudinit && qm set 9001 --agent enabled=1 && qm template 9001```

## Setup OpenTofu
Run the following on your deployment node

### Download the installer script:
```wget --secure-protocol=TLSv1_2 --https-only https://get.opentofu.org/install-opentofu.sh -O install-opentofu.sh```
### Give it execution permissions:
```chmod +x install-opentofu.sh```
### Run the installer:
```./install-opentofu.sh --install-method deb```
### Remove the installer:
```rm install-opentofu.sh```

## install helm
```curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3```
```chmod 700 get_helm.sh```
```./get_helm.sh```

## Setup Kubespray
Run the following on your deployment node

```git clone https://github.com/kubernetes-incubator/kubespray.git```

```cd kubespray```

```apt install python3-pip -y```

```pip3 install -r requirements.txt --break-system-packages```
```change clustername in group_vars/k8s_cluster/k8s-cluster.yml file to mkcluster.local```

```run kubespray_update.py in docker_build```

```ansible-galaxy collection install kubernetes.core```
```ansible-galaxy collection install community.kubernetes```
```ansible-galaxy collection install cloud.common```

# follow proxmox readme to create control planes and worker nodes
tofu init
tofu plan
tofu apply

# run tofu from cluster directory to build k8s cluster/operators/etc
tofu init
tofu plan
tofu apply
some things might fail....wait a bit for parts to spin up and rerun apply

# get/user/keys/etc for k8sdashboard
ssh metaman@192.168.50.50
kubectl get secret admin-user -n kubernetes-dashboard -o jsonpath={".data.token"} | base64 -d

# run tofu from environment directory to build k8s operators/etc and mediakraken itself
tofu init
tofu plan
tofu apply
some things might fail....wait a bit for parts to spin up and rerun apply

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

# setup graphana dash for rabbitmq
import 10991

# had to do the dragonfly db yml by hand and it fired up????????

# longhorn security
USER=metaman; PASSWORD=metaman; echo "${USER}:$(openssl passwd -stdin -apr1 <<< ${PASSWORD})" >> auth
kubectl -n longhorn-system create secret generic basic-auth --from-file=auth

# longhorn
remove schedule from preexiting drives
add disks to each nodes
    call em metaman1, 2, 3
    do NOT set reserved space
/mnt/volume/disk2   is the one for images/etc

volume, create volume
volume 900gb metadata
attach to host.....worker1
now you have a shiney replicating volume

/dev/mapper/vgk8s1-vgk8s1lv 512gb
/dev/mapper/vgk8s2-vgk8s2lv  1tb


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


##############
helm package mkstack-mediakraken

helm install --create-namespace --namespace mediakraken mkstack-mediakraken https://github.com/MediaKraken/MediaKraken/raw/dev/k8s/mkstack-mediakraken-0.1.8.tgz

# set db secret
kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='592d-9b85-41da-9d1' -n mediakraken

<!-- # stuff below to do
# stuff to add to MK
https://operatorhub.io/operator/elastic-cloud-eck
Elastic Cloud on Kubernetes (ECK) is the official operator by Elastic for automating the deployment, provisioning, management, and orchestration of Elasticsearch, Kibana, APM Server, Beats, Enterprise Search, Elastic Agent, Elastic Maps Server, and Logstash on Kubernetes.

# stuff to add to DEV stack CI/CD/etc

# misc stuff to play with
https://artifacthub.io/packages/helm/helm-hass/home-assistant
This chart bootstraps a deployment on a cluster using the package manager.

https://artifacthub.io/packages/helm/bitnami/vault
Vault is a tool for securely managing and accessing secrets using a unified interface. Features secure storage, dynamic secrets, data encryption and revocation.

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

https://artifacthub.io/packages/helm/bitnami/consul
HashiCorp Consul is a tool for discovering and configuring services in your infrastructure.

https://artifacthub.io/packages/helm/si-gitops/nut-exporter
Installs NUT exporter in Kubernetes

on dev

kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

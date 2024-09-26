# this setup is run from a debian linux install
it will create proxmox vm's via opentofu
it will then install k8s cluster via kubespray

# install process

## Setup Proxmox
### Setup roles/etc for OpenTofu
Run the following commands on your Proxmox node

```pveum role add terraform-role -privs "VM.Allocate VM.Clone VM.Config.CDROM VM.Config.CPU VM.Config.Cloudinit VM.Config.Disk VM.Config.HWType VM.Config.Memory VM.Config.Network VM.Config.Options VM.Monitor VM.Audit VM.PowerMgmt Datastore.AllocateSpace Datastore.Audit User.Modify Sys.Audit Sys.Console Sys.Modify VM.Migrate Pool.Allocate SDN.Use"```

```pveum user add terraform@pve```

```pveum aclmod / -user terraform@pve -role terraform-role```

```pveum user token add terraform@pve terraform-token --privsep=0```

### Setup template to use
Run the following commands on your Proxmox node

```apt-get update && apt install libguestfs-tools -y```

```wget https://cloud.debian.org/images/cloud/bookworm/20240717-1811/debian-12-genericcloud-amd64-20240717-1811.qcow2```

```virt-customize -a debian-12-genericcloud-amd64-20240717-1811.qcow2 --install qemu-guest-agent --run-command 'systemctl enable qemu-guest-agent.service'```

```virt-customize -a debian-12-genericcloud-amd64-20240717-1811.qcow2 --run-command "echo -n > /etc/machine-id"```

```qm create 9000 --name "debian-12-cloudinit-template-mk" --memory 2048 --cores 2 --net0 virtio,bridge=vmbr0 && qm importdisk 9000 debian-12-genericcloud-amd64-20240717-1811.qcow2 local-lvm && qm set 9000 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-9000-disk-0 && qm set 9000 --boot c --bootdisk scsi0 && qm set 9000 --ide2 local-lvm:cloudinit && qm set 9000 --agent enabled=1 && qm template 9000```

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

```cp -rfp kubespray/inventory/sample MediaKraken/k8s/cluster/kubespray/mkcluster```
```cp -rfp kubespray/inventory/sample MediaKraken/k8s_devenv/cluster/kubespray/mkclusterdev```

```cd kubespray```

```apt install python3-pip -y```

```pip3 install -r requirements.txt --break-system-packages```
```change clustername in k8s-cluster.yml file```

```cp -R ../MediaKraken/k8s/cluster/kubespray/mkcluster inventory/mkcluster```

```cp -R ../MediaKraken/k8s_devenv/cluster/kubespray/mkclusterdev inventory/mkclusterdev```

```ansible-galaxy collection install kubernetes.core```
```ansible-galaxy collection install community.kubernetes```
```ansible-galaxy collection install cloud.common```

# on control plane - don't using helm instead now
```sudo apt install python3-pip -y```
```sudo pip3 install kubernetes --break-system-packages```


# follow opentofu/proxmox readme to create control planes and workers

# run tofu from cluster directory to build k8s cluster/operators/etc




# stuff below to do
# TODO had to split up the rbac user for dashboard by hand........
kubectl -n kubernetes-dashboard port-forward svc/kubernetes-dashboard-kong-proxy 8443:443
ssh -L 8443:127.0.0.1:8443 metaman@192.168.1.50
http://localhost:8001/api/v1/namespaces/kubernetes-dashboard/services/https:kubernetes-dashboard-kong-proxy:443/proxy/

helm repo add kubeshark https://helm.kubeshark.co
‍helm install kubeshark kubeshark/kubeshark

https://artifacthub.io/packages/helm/geek-cookbook/unifi-poller
https://artifacthub.io/packages/helm/loeken-at-home/vaultwarden
https://artifacthub.io/packages/olm/community-operators/galaxy-operator
https://grafana.com/grafana/dashboards/3020-teamspeak-3/
https://artifacthub.io/packages/helm/cloudhippie/ansible-semaphore
https://artifacthub.io/packages/helm/geek-cookbook/network-ups-tools
https://artifacthub.io/packages/helm/bitnami/vault
https://artifacthub.io/packages/helm/si-gitops/nut-exporter
https://artifacthub.io/packages/helm/m0nsterrr/jellyfin
https://artifacthub.io/packages/tekton-task/tekton-tasks/grype
https://artifacthub.io/packages/helm/oneuptime/oneuptime
https://artifacthub.io/packages/helm/qonstrukt/unifi-controller
https://artifacthub.io/packages/helm/geek-cookbook/owncast
https://artifacthub.io/packages/helm/rock8s/plane
https://artifacthub.io/packages/helm/elastic/eck-operator
https://artifacthub.io/packages/helm/elk-stack/elk-stack
https://artifacthub.io/packages/helm/fmjstudios/ntfy
https://operatorhub.io/operator/elastic-cloud-eck
https://artifacthub.io/packages/helm/crowdsec/crowdsec
https://artifacthub.io/packages/helm/curie-df-helm-charts/nexus
https://artifacthub.io/packages/helm/sagikazarmark/sftpgo
https://artifacthub.io/packages/helm/bitnami/nats
https://artifacthub.io/packages/helm/crystalnet/romm
https://artifacthub.io/packages/helm/jfrog/artifactory-oss
https://artifacthub.io/packages/container/jitesoft-trivy/trivy
https://artifacthub.io/packages/helm/joxit/docker-registry-ui
https://artifacthub.io/packages/container/jitesoft-trivy/trivy
https://artifacthub.io/packages/helm/mojo2600/pihole
https://artifacthub.io/packages/helm/wyrihaximusnet/pi-hole-exporter
https://artifacthub.io/packages/helm/fmjstudios/vaultwarden
https://artifacthub.io/packages/helm/docker-postfix/mail
https://artifacthub.io/packages/helm/sonarqube/sonarqube-dce
https://artifacthub.io/packages/helm/uptime-kuma/uptime-kuma
https://artifacthub.io/packages/helm/agones/agones
https://artifacthub.io/packages/helm/helm-hass/home-assistant

https://artifacthub.io/packages/helm/bitnami/consul

https://artifacthub.io/packages/helm/bitnami/sonarqube

https://artifacthub.io/packages/helm/geek-cookbook/wireguard

https://artifacthub.io/packages/helm/elk-stack/elk-stack

https://stackgres.io/doc/latest/install/helm/

on dev
<!-- kubectl apply -f https://raw.githubusercontent.com/skooner-k8s/skooner/master/kubernetes-skooner.yaml
kubectl apply -f https://raw.githubusercontent.com/skooner-k8s/skooner/master/kubernetes-skooner-nodeport.yaml
kubectl get svc --namespace=kube-system
http://192.168.1.50:31732/
setup token for skooner
kubectl create serviceaccount skooner-sa
kubectl create clusterrolebinding skooner-sa --clusterrole=cluster-admin --serviceaccount=default:skooner-sa
kubectl create token skooner-sa -->
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml

# https://stackgres.io/doc/latest/quickstart/
# on one master to setup the pg cluster
helm install --create-namespace --namespace mkdatabase stackgres-operator \
 --set-string adminui.service.type=LoadBalancer \
 --set grafana.autoEmbed=true https://stackgres.io/downloads/stackgres-k8s/stackgres/latest/helm/stackgres-operator.tgz

cat << 'EOF' | kubectl create -f -
apiVersion: stackgres.io/v1
kind: SGCluster
metadata:
  name: simple
spec:
  instances: 2
  postgres:
    version: 'latest'
  pods:
    persistentVolume: 
      size: '250Gi'
      storageClass: nfs-csi-8k
EOF

kubectl get pods --watch

# wait till 6/6

kubectl exec -ti "$(kubectl get pod --selector app=StackGresCluster,stackgres.io/cluster=true,role=master -o name)" -c postgres-util -- psql


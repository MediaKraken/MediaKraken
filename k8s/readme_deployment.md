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

## Setup Kubespray
Run the following on your deployment node

```git clone https://github.com/kubernetes-incubator/kubespray.git```

#```cp -rfp kubespray/inventory/sample MediaKraken/k8s/opentofu/k8s/kubespray/mkcluster```

```cd kubespray```

```apt install python3-pip -y```

```pip3 install -r requirements.txt --break-system-packages```

```cp -R ../MediaKraken/k8s/opentofu/k8s/kubespray/mkcluster inventory/mkcluster```

```cp -R ../MediaKraken/k8s_devenv/kubespray/mkclusterdev inventory/mkclusterdev```

```ansible-playbook -b -v -u metaman -i inventory/mkcluster/inventory.ini cluster.yml --ssh-common-args='-o StrictHostKeyChecking=accept-new'```

# Setup kube config on master nodes (from docker_k8s direcory)
```ansible-playbook -b -v -u metaman -i opentofu/k8s/kubespray/mkcluster/inventory.ini playbooks/kube.yml --ask-sudo-pass```

# Setup Helm on master nodes (from docker_k8s direcory)
```ansible-playbook -b -v -u metaman -i opentofu/k8s/kubespray/mkcluster/inventory.ini playbooks/helm.yml```

# setup NFS capability
```ansible-playbook -b -v -u metaman -i opentofu/k8s/kubespray/mkcluster/inventory.ini playbooks/nfs.yml --ask-sudo-pass```

# setup operator
```ansible-playbook -b -v -u metaman -i opentofu/k8s/kubespray/mkcluster/inventory.ini playbooks/operator.yml```



# stuff below to do

# Add kubernetes-dashboard repository
helm repo add kubernetes-dashboard https://kubernetes.github.io/dashboard/
# Deploy a Helm Release named "kubernetes-dashboard" using the kubernetes-dashboard chart
helm upgrade --install kubernetes-dashboard kubernetes-dashboard/kubernetes-dashboard --create-namespace --namespace kubernetes-dashboard


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




# https://www.dragonflydb.io/docs/getting-started/kubernetes-operator
kubectl apply -f https://raw.githubusercontent.com/dragonflydb/dragonfly-operator/main/manifests/dragonfly-operator.yaml

kubectl apply -f "https://github.com/rabbitmq/cluster-operator/releases/latest/download/cluster-operator.yml"

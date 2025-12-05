# create a deployment node to install tools on
## I've been using debian........and instructions reflect that
```
apt install python3-pip python3-venv -y
```

## Setup OpenTofu
```
wget --secure-protocol=TLSv1_2 --https-only https://get.opentofu.org/install-opentofu.sh -O install-opentofu.sh
chmod +x install-opentofu.sh
./install-opentofu.sh --install-method deb
rm install-opentofu.sh
```

## setup helm
```
curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
chmod 700 get_helm.sh
./get_helm.sh
```

## setup kubespray
```
cd ~
git clone https://github.com/kubernetes-incubator/kubespray.git
cd kubespray
python3 -m venv env_name
source env_name/bin/activate
pip3 install -r ~/kubespray/requirements.txt --break-system-packages
pip3 install dotenv proxmoxer requests paramiko openssh_wrapper --break-system-packages

TODO copy inventory/etc
change clustername in inventory/mkcluster/group_vars/k8s_cluster/k8s-cluster.yml file to mkcluster.local

ansible-galaxy collection install kubernetes.core
ansible-galaxy collection install community.kubernetes
ansible-galaxy collection install cloud.common
```
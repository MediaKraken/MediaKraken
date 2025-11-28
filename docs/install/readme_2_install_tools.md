# create a deployment node to install tools on
## I've been using debian........and instructions reflect that

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
git clone https://github.com/kubernetes-incubator/kubespray.git

cd kubespray

apt install python3-pip -y

TODO setup venv due to ansible versions
pip3 install -r requirements.txt --break-system-packages
change clustername in group_vars/k8s_cluster/k8s-cluster.yml file to mkcluster.local

TODO run kubespray_update.py in docker_build

ansible-galaxy collection install kubernetes.core
ansible-galaxy collection install community.kubernetes
ansible-galaxy collection install cloud.common
```
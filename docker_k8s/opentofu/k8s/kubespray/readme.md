git clone https://github.com/kubernetes-incubator/kubespray.git

cp -rfp kubespray/inventory/sample MediaKraken/docker_k8s/opentofu/k8s/kubespray/mkcluster

cd kubespray

apt install python3-pip -y
pip3 install -r requirements.txt --break-system-packages

cp -R ../MediaKraken/docker_k8s/opentofu/k8s/kubespray/mkcluster inventory/mkcluster

ssh-copy-id root@192.168.1.50
ssh-copy-id root@192.168.1.51
ssh-copy-id root@192.168.1.52
ssh-copy-id root@192.168.1.60
ssh-copy-id root@192.168.1.61
ssh-copy-id root@192.168.1.62


ansible-playbook -b -v -u metaman -i inventory/mkcluster/inventory.ini cluster.yml


# on the master nodes
mkdir -p $HOME/.kube && sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config && sudo chown $(id -u):$(id -g) $HOME/.kube/config

kubectl get nodes

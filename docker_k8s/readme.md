https://www.linuxtechi.com/install-kubernetes-cluster-on-debian/

debian12 net
ssh

su
apt-get install sudo curl gpg -y

/sbin/adduser metaman sudo

relog

sudo hostnamectl set-hostname "mkcube1.beaverbay.local"

sudo hostnamectl set-hostname "mkcube2.beaverbay.local"

sudo hostnamectl set-hostname "mkcube3.beaverbay.local"

nano /etc/hosts
192.168.1.132   mkcube1.beaverbay.local   mkcube1
192.168.1.134   mkcube2.beaverbay.local   mkcube2
192.168.1.135   mkcube3.beaverbay.local   mkcube3

sudo swapoff -a
sudo sed -i '/ swap / s/^\(.*\)$/#\1/g' /etc/fstab

/*
3) Add Firewall Rules for Kubernetes Cluster
In case, OS firewall is enabled on your Debian systems then allow following ports on master and worker nodes respectively.

On Master node, run

$ sudo ufw allow 6443/tcp
$ sudo ufw allow 2379/tcp
$ sudo ufw allow 2380/tcp
$ sudo ufw allow 10250/tcp
$ sudo ufw allow 10251/tcp
$ sudo ufw allow 10252/tcp
$ sudo ufw allow 10255/tcp
$ sudo ufw reload
On Worker Nodes,

$ sudo ufw allow 10250/tcp
$ sudo ufw allow 30000:32767/tcp
$ sudo ufw reload
Note: If firewall is disabled on your Debian 12/11 systems, then you can skip this step.
*/

cat <<EOF | sudo tee /etc/modules-load.d/containerd.conf 
overlay 
br_netfilter
EOF

sudo modprobe overlay
sudo modprobe br_netfilter

cat <<EOF | sudo tee /etc/sysctl.d/99-kubernetes-k8s.conf
net.bridge.bridge-nf-call-iptables = 1
net.ipv4.ip_forward = 1 
net.bridge.bridge-nf-call-ip6tables = 1 
EOF

sudo sysctl --system
sudo apt-get update
sudo apt -y install containerd

containerd config default | sudo tee /etc/containerd/config.toml >/dev/null 2>&1

sudo nano /etc/containerd/config.toml

search SystemdCgroup and set to true

sudo systemctl restart containerd

echo "deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.28/deb/ /" | sudo tee /etc/apt/sources.list.d/kubernetes.list
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.28/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg

sudo apt update
sudo apt install kubelet kubeadm kubectl -y
sudo apt-mark hold kubelet kubeadm kubectl

MASTER
vi kubelet.yaml

apiVersion: kubeadm.k8s.io/v1beta3
kind: InitConfiguration
---
apiVersion: kubeadm.k8s.io/v1beta3
kind: ClusterConfiguration
kubernetesVersion: "1.28.0" # Replace with your desired version
controlPlaneEndpoint: "mkcube1"
---
apiVersion: kubelet.config.k8s.io/v1beta1
kind: KubeletConfiguration

sudo kubeadm init --config kubelet.yaml

um?

mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config

# i had to do this to fix the config
sudo kubeadm reset

kubectl get nodes
kubectl cluster-info

# on workers!
sudo kubeadm join mkcube1:6443 --token 21nm87.fakestuffiiau \
--discovery-token-ca-cert-hash sha256:28b5fakestuff24b78009

# MASTER at step 8
kubectl apply -f https://raw.githubusercontent.com/projectcalico/calico/v3.26.1/manifests/calico.yaml


on all nodes
sudo ufw allow 179/tcp
$ sudo ufw allow 4789/udp
$ sudo ufw allow 51820/udp
$ sudo ufw allow 51821/udp
$ sudo ufw reload



kubectl get pods -n kube-system


# cluxster is up

kubectl create deployment nginx-app --image=nginx --replicas 2
kubectl expose deployment nginx-app --name=nginx-web-svc --type NodePort --port 80 --target-port 80
kubectl describe svc nginx-web-svc


see running pods
kubectl get pods -A

kubectl get deployments --all-namespaces


# nice video
https://www.youtube.com/watch?v=r2zuL9MW6wc


# convert
curl -L https://github.com/kubernetes/kompose/releases/download/v1.34.0/kompose-linux-amd64 -o kompose
chmod +x kompose
sudo mv ./kompose /usr/local/bin/kompose
kompose convert -f docker-compose.yml




# start api
kubectl proxy --port=8080 &
curl http://localhost:8080/api/

# windows doom
https://dl.k8s.io/release/v1.30.0/bin/windows/amd64/kubectl.exe


# setup mediakraken
kubectl create namespace mediakraken

kubectl apply -f https://raw.githubusercontent.com/MediaKraken/MediaKraken/dev/docker_k8s/mkstack-multicast-deployment.yaml


# on master
https://github.com/kubernetes-csi/csi-driver-nfs/blob/master/docs/install-csi-driver-v4.7.0.md
curl -skSL https://raw.githubusercontent.com/kubernetes-csi/csi-driver-nfs/v4.7.0/deploy/install-driver.sh | bash -s v4.7.0 --

# worker nodes
apt install -y nfs-common

# was running fo rlast five months
# storage
zfs create wdblack/sharenfs
zfs set atime=off wdblack/sharenfs


# experiment
zfs create wdblack/sharenfs8k
zfs set atime=off wdblack/sharenfs8k
zfs set recordsize=8K wdblack/sharenfs8k
zfs set primarycache=metadata wdblack/sharenfs8k
zfs set logbias=throughput wdblack/sharenfs8k

<!-- zfs set sharenfs=on wdblack/sharenfs
# zfs set sharenfs="on,rw=@192.168.1.0/24" wdblack/sharenfs
zfs get sharenfs
zfs share -a

zfs set sharenfs=off pool54_62/batocera
zfs set sharenfs=off wdblack/sharenfs
zfs unshare -a
systemctl restart nfs-kernel-server -->

chown -R nobody:nogroup /wdblack/sharenfs
chmod -R 777 /wdblack/sharenfs/
nano /etc/exports
/wdblack/sharenfs 192.168.1.0/24(rw,sync,no_subtree_check)
/wdblack/sharenfs8k 192.168.1.0/24(rw,sync,no_subtree_check)
exportfs -a
exportfs -r
exportfs -v







# do the storage setup/claim/etc
kubectl apply -f nfs-pvc.yaml
kubectl apply -f nfs-pvc-8k.yaml
kubectl get storageclasses
kubectl describe storageclasses nfs-csi


# installed all claims
kubectl apply -f claim




# secrets
# kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='S!B\*d$zDsb='

kubectl create secret generic db-password --from-literal=username=devuser --from-literal=password='S!B\*d$zDsb='
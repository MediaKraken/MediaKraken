kubectl get pods -n cnpg-system

kubectl cnpg status pgcluster-with-metrics -v -n cnpg-system


# promote a node
kubectl cnpg promote CLUSTER CLUSTER-INSTANCE

# this command will restart a whole cluster in a rollout fashion
kubectl cnpg restart CLUSTER

# this command will restart a single instance, according to the policy above
kubectl cnpg restart CLUSTER INSTANCE

# remove instaces from a cluster
kubectl cnpg destroy CLUSTER INSTANCE

# destroy cluster
kubectl delete cluster pgcluster-with-metrics -n cnpg-system


# old way that's deprecated
# kubelet --feature-gates ImageVolume=true
# kube-apiserver --feature-gates kube:ImageVolume=true


Edit the API Server manifest: Open the kube-apiserver manifest file
Add the flag --feature-gates=ImageVolume=true to the command arguments. The API server should restart automatically after you save the file.

sudo nano /etc/kubernetes/manifests/kube-apiserver.yaml

Modify the Kubelet configuration: Edit the Kubelet configuration file and add the featureGates block:

featureGates:
  ImageVolume: true

Restart the Kubelet service using the appropriate command for your system.   
    sudo systemctl daemon-reload && sudo systemctl restart kubelet

# shows it's set to true
kubectl cluster-info dump | grep feature-gate


# for the backup stuff......
kubectl create secret generic aws-creds \
  --from-literal=ACCESS_KEY_ID=GK17c18e134f2b45e40faf0c2b \
  --from-literal=ACCESS_SECRET_KEY=0a898de4b9a9f37fbca7a7a2dc95a8cb59518eda11eab4a960fa434a52b40c91 \
  --from-literal=AWS_REGION=garage \
  -n cnpg-system

# status of backup
kubectl describe backup backup-devenv -n cnpg-system

kubectl delete backup backup-devenv -n cnpg-system

kubectl logs -n cnpg-system <POD>

kubectl cnpg logs cluster pgcluster-with-metrics -n cnpg-system 

kubectl get pvc -l cnpg.io/clusterName=pgcluster-with-metrics -n cnpg-system 

kubectl get pod -l cnpg.io/clusterName=pgcluster-with-metrics -n cnpg-system

kubectl cnpg status -n cnpg-system pgcluster-with-metrics

# encode the username/password
echo -n 'bmodbuser' | base64  # Outputs: Ym1vZGJ1c2Vy
echo -n 'bmodbpasswordfg423oi23iho' | base64  # Outputs: Ym1vZGJwYXNzd29yZGZnNDIzb2kyM2lobw==

echo -n 'garage' | base64  # Outputs: Z2FyYWdl

echo -n 'bmodbsuperuser' | base64  # Outputs: Ym1vZGJzdXBlcnVzZXI=
echo -n 'bmodbsuperpasswordfg423oi23iho' | base64  # Outputs: Ym1vZGJzdXBlcnBhc3N3b3JkZmc0MjNvaTIzaWhv
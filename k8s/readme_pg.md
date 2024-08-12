
kubectl create -f https://operatorhub.io/install/postgresql.yaml
kubectl get csv -n operators

# docs
https://operatorhub.io/operator/postgresql

git clone --depth 1 https://github.com/SpootDev/postgres-operator-examples
cd postgres-operator-examples

kubectl apply -k kustomize/install/namespace
kubectl apply --server-side -k kustomize/install/default
kubectl -n postgres-operator get pods \
  --selector=postgres-operator.crunchydata.com/control-plane=postgres-operator \
  --field-selector=status.phase=Running

kubectl apply -k kustomize/postgres

# status
kubectl -n postgres-operator describe postgresclusters.postgres-operator.crunchydata.com hippo
kubectl -n postgres-operator get svc --selector=postgres-operator.crunchydata.com/cluster=hippo

cant find dns as no storage to start the primary
psql $(kubectl -n postgres-operator get secrets hippo-pguser-hippo -o go-template='{{.data.uri | base64decode}}')


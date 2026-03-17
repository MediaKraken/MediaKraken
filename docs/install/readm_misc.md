kubectl get clusterissuers

kubectl describe clusterissuer letsencrypt-cluster

kubectl get certificates,certificaterequests,orders,challenges -A


kubectl get ingress -A

kubectl describe ingress cert-http-ingress-com-pgadmin -n pgadmin

kubectl logs <ingress-controller-pod-name> -n pgadmin

kubectl get certificates -A

kubectl describe challenge letsencrypt-monitoring-priv-1-1053747751-197586073 -n monitoring

kubectl delete challenge letsencrypt-monitoring-priv-1-1053747751-197586073 -n monitoring

kubectl get challenges -A
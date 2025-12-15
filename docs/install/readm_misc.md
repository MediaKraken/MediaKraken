kubectl get clusterissuers

kubectl describe clusterissuer letsencrypt-cluster

kubectl get certificates,certificaterequests,orders,challenges -A


kubectl get ingress -A

kubectl describe ingress cert-http-ingress-com-pgadmin -n pgadmin

kubectl logs <ingress-controller-pod-name> -n pgadmin

kubectl get certificates -A
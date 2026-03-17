kubectl describe clusterissuer letsencrypt-cluster -n cert-manager

kubectl get certificate -A -n cert-manager

kubectl describe certificate <certificate-name> -n cert-manager

kubectl get orders,challenges -A

kubectl logs -n cert-manager deploy/cert-manager --tail=200



cmctl renew <certificate-name> -n cert-manager



kubectl describe challenge kubernetes-dashboard-certs-2-4286013329-1311853614 -n kubernetes-dashboard


dig TXT _acme-challenge.mkk8s.mediakraken.media


kubectl get challenges -A
terraform {
  backend "kubernetes" {
    secret_suffix    = "state"
    config_path      = "~/.kube/config"
  }
}


/*
helm install --create-namespace --namespace mkdatabase stackgres-operator \
 --set-string adminui.service.type=LoadBalancer \
 --set grafana.autoEmbed=true https://stackgres.io/downloads/stackgres-k8s/stackgres/latest/helm/stackgres-operator.tgz
 */
 
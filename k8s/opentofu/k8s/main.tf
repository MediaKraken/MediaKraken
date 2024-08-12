terraform {
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.31.0"
    }
  }
}

provider "kubernetes" {
  config_path    = "~/.kube/config"
}

resource "kubernetes_namespace" "mediakraken" {
  metadata {
    labels = {
      app = "mediakraken"
    }
    name = "mediakraken"
  }
}
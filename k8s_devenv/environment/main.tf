resource "terraform_data" "dragonfly" {
  # setup dragonfly operator and cluster
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/dragonflydb.yml"
  }
}

resource "terraform_data" "k8sdashboard" {
  # setup k8s dashboard
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/k8sdashboard.yml"
  }
  depends_on = [
    terraform_data.dragonfly
  ]
}

resource "terraform_data" "mailhog" {
  # setup mailhog
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/mailhog.yml"
  }
  depends_on = [
    terraform_data.k8sdashboard
  ]
}

resource "terraform_data" "docker_registry" {
  # setup docker registry
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/docker-registry.yml"
  }
  depends_on = [
    terraform_data.mailhog
  ]
}

resource "terraform_data" "docker_ui" {
  # setup docker ui
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/docker-ui.yml"
  }
  depends_on = [
    terraform_data.docker_registry
  ]
}

resource "terraform_data" "jenkins" {
  # setup jenkins
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/jenkins.yml"
  }
  depends_on = [
    terraform_data.docker_ui
  ]
}

resource "terraform_data" "monitoring" {
  # setup monitoring
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/grafana_prometheus.yml"
  }
  depends_on = [
    terraform_data.jenkins
  ]
}

resource "terraform_data" "gocd" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/gocd.yml"
  }
  depends_on = [
    terraform_data.monitoring
  ]
}

# resource "terraform_data" "grype" {
#   provisioner "local-exec" {
#     command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/grype.yml"
#   }
#   depends_on = [
#     terraform_data.gocd
#   ]
# }

resource "terraform_data" "jfrog" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/jfrog_artifactory_oss.yml"
  }
  depends_on = [
    terraform_data.gocd
  ]
}

resource "terraform_data" "kellnr" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/kellnr.yml"
  }
  depends_on = [
    terraform_data.jfrog
  ]
}

resource "terraform_data" "plane" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/plane.yml"
  }
  depends_on = [
    terraform_data.kellnr
  ]
}

resource "terraform_data" "sftp" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/sftp.yml"
  }
  depends_on = [
    terraform_data.plane
  ]
}

resource "terraform_data" "sonarqube" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/sonarqube.yml"
  }
  depends_on = [
    terraform_data.sftp
  ]
}

resource "terraform_data" "sonatype_nexus" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/sonatype_nexus.yml"
  }
  depends_on = [
    terraform_data.sonarqube
  ]
}

resource "terraform_data" "trivy" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/trivy.yml"
  }
  depends_on = [
    terraform_data.sonatype_nexus
  ]
}

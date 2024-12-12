resource "terraform_data" "dragonfly" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/dragonflydb.yml"
  }
}

resource "terraform_data" "rabbitmq" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/rabbitmq.yml"
  }
  depends_on = [
    terraform_data.dragonfly
  ]
}

resource "terraform_data" "postfix" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/postfix.yml"
  }
  depends_on = [
    terraform_data.rabbitmq
  ]
}

resource "terraform_data" "wireguard" {
  provisioner "local-exec" {
    command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/wireguard.yml"
  }
  depends_on = [
    terraform_data.postfix
  ]
}

# resource "terraform_data" "mediakraken" {
#   # setup mediakraken
#   provisioner "local-exec" {
#     command = "ansible-playbook -b -v -u ${var.vm_user} -i inventory.ini playbooks/mediakraken.yml"
#   }
#   depends_on = [
#     terraform_data.wireguard
#   ]
# }

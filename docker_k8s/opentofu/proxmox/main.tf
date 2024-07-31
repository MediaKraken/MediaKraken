terraform {
  required_providers {
    proxmox = {
      source = "telmate/proxmox"
      version = "3.0.1-rc3"
    }
  }
}

provider "proxmox" {
 pm_api_url = var.api_url
 pm_api_token_id = var.token_id
 pm_api_token_secret = var.token_secret
 pm_tls_insecure = true
}

resource "proxmox_vm_qemu" "mkk8scontrol" {
 vmid       = 1000
 name       = "mkcontrol${count.index + 1}"
 desc       = "k8s Control Plane"
 count      = 1
 target_node = var.proxmox_host
 clone      = "debian-12-cloudinit-template-mk"
 hotplug    = "network,disk"
 cores      = 4
 sockets    = 1
 cpu        = "host"
 memory     = 8192
 agent      = 1
 os_type    = "Linux"
 full_clone = "true"
 scsihw     = "virtio-scsi-pci"
 boot       = "order=scsi0"
 bootdisk   = "scsi0"
 onboot     = "true"

 disks {
   scsi {
      scsi0 {
        disk {
          size = "16G"
          storage = var.storage_name
        }
      }
    }
  }

 network {
  model   = "virtio"
  bridge  = var.nic_name
  firewall  = false
  link_down = false
 }

 vga {
  type = "std"
 }

 lifecycle {
   ignore_changes = [
     network,
   ]
 }
}


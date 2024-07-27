provider "proxmox" {
 pm_api_url   = "https://your-proxmox-host:8006/api2/json"
 pm_user      = "blog_example"
 pm_password  = "your-api-key"
 pm_tls_insecure = true
}

resource "proxmox_vm_qemu" "my_vm" {
 name       = "my-vm"
 target_node = "pve"
 clone      = "ubuntu-template"
 storage    = "local-lvm"
 cores      = 2
 memory     = 2048
}

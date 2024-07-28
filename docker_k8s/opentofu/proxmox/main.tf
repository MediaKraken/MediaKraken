provider "proxmox" {
 pm_api_url   = "https://192.168.1.11:8006/api2/json"
 pm_user      = "metaman"
 pm_password  = "12d03d06-d66e-4950-abbf-41a2757cfed6"
 pm_tls_insecure = true
}

resource "proxmox_vm_qemu" "mkcontrol1" {
 name       = "mkcontrol1"
 target_node = "pvezfs"
 clone      = "ubuntu-template"
 storage    = "local-lvm"
 cores      = 4
 memory     = 8192
}

resource "proxmox_vm_qemu" "mkcontrol2" {
 name       = "mkcontrol2"
 target_node = "pvezfs"
 clone      = "ubuntu-template"
 storage    = "local-lvm"
 cores      = 4
 memory     = 8192
}

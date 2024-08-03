https://registry.terraform.io/providers/Telmate/proxmox/latest/docs

https://pve.proxmox.com/wiki/Cloud-Init_Support


# setup roles/etc for opentofu
pveum role add terraform-role -privs "VM.Allocate VM.Clone VM.Config.CDROM VM.Config.CPU VM.Config.Cloudinit VM.Config.Disk VM.Config.HWType VM.Config.Memory VM.Config.Network VM.Config.Options VM.Monitor VM.Audit VM.PowerMgmt Datastore.AllocateSpace Datastore.Audit User.Modify Sys.Audit Sys.Console Sys.Modify VM.Migrate Pool.Allocate SDN.Use"
pveum user add terraform@pve
pveum aclmod / -user terraform@pve -role terraform-role
pveum user token add terraform@pve terraform-token --privsep=0


# setup template
apt-get update && apt install libguestfs-tools -y
wget https://cloud.debian.org/images/cloud/bookworm/20240717-1811/debian-12-genericcloud-amd64-20240717-1811.qcow2
virt-customize -a debian-12-genericcloud-amd64-20240717-1811.qcow2 --install qemu-guest-agent --run-command 'systemctl enable qemu-guest-agent.service'
virt-customize -a debian-12-genericcloud-amd64-20240717-1811.qcow2 --run-command "echo -n > /etc/machine-id"

qm create 9000 --name "debian-12-cloudinit-template-mk" --memory 2048 --cores 2 --net0 virtio,bridge=vmbr0 \
  && qm importdisk 9000 debian-12-genericcloud-amd64-20240717-1811.qcow2 local-lvm \
  && qm set 9000 --scsihw virtio-scsi-pci --scsi0 local-lvm:vm-9000-disk-0 \
  && qm set 9000 --boot c --bootdisk scsi0 \
  && qm set 9000 --ide2 local-lvm:cloudinit \
  && qm set 9000 --agent enabled=1 \
  && qm template 9000

# apt install python3-paramiko
import paramiko
import asyncio
import sys

class CommonNetworkSSH:
    """
    Class for interfacing via SSH
    """

    def __init__(self, host, user_name, user_password):
        # Create an SSH session to be used for all our requests
        self.ssh_connection = paramiko.SSHClient()
        self.ssh_connection.set_missing_host_key_policy(
            paramiko.AutoAddPolicy())
        self.ssh_connection.connect(
            host, username=user_name, password=user_password)

    def com_net_ssh_run_sudo_command(self, command_text, sudo_password='metaman'):
        """
        Run specified command as sudo so it will send the password
        """
        ssh_stdin, ssh_stdout, ssh_stderr = self.ssh_connection.exec_command(command_text,
                                                                             get_pty=True)
        ssh_stdin.write(sudo_password + '\n')
        ssh_stdin.flush()
        return ssh_stdout.read()

    def com_net_ssh_run_command(self, command_text):
        """
        Run specified command and write output
        """
        ssh_stdin, ssh_stdout, ssh_stderr = self.ssh_connection.exec_command(command_text,
                                                                             get_pty=True)
        return ssh_stdout.read()

    def com_net_ssh_close(self):
        """
        Close the ssh connection
        """
        self.ssh_connection.close()


async def main(loop):
    # connection to proxmox instance
    prox_inst = CommonNetworkSSH(host='192.168.1.n',
                                user_name='root',
                                user_password='fakepassword!')
    # grab the disk list
    disk_out = prox_inst.com_net_ssh_run_command("lsblk -o NAME,MODEL,SERIAL,SIZE,STATE -d")
    for disk in disk_out.split(b'\r\n'):
        disk = disk.decode('utf-8')
        if ("running" in disk or "live" in disk) and "sr0" not in disk and "nvme" not in disk:
            disk_name = disk.split(' ')[0]
            # get wwid for disk
            wwid = prox_inst.com_net_ssh_run_command("/lib/udev/scsi_id -g -u -d /dev/" 
                                                     + disk_name).strip().decode('utf-8')
            print("    wwid    \"" + wwid + "\" # " + disk_name)

if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main(loop))
    loop.close()
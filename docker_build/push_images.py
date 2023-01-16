
# must login to docker hub first *IF* one wants to push to dockerhub
# docker login --username=mediakraken

import os
import shlex
import subprocess

try:
    from dotenv import load_dotenv
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt-get install python3-dotenv -y'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from dotenv import load_dotenv

import docker_images_list

# load .env stats
load_dotenv()

cmd1 = ["docker", "images"]
cmd2 = ["grep", "mediakraken/"]
cmd3 = ["grep", os.environ['BRANCH']]
# start
cmd1_proc = subprocess.run(cmd1, stdout=subprocess.PIPE)
#print(cmd1_proc.stdout)
cmd2_proc = subprocess.run(cmd2, input=cmd1_proc.stdout, capture_output=True)
#print(cmd2_proc.stdout)
cmd3_proc = subprocess.run(cmd3, input=cmd2_proc.stdout, stdout=subprocess.PIPE)
#print(cmd3_proc.stdout)
for line in cmd3_proc.stdout.splitlines():
    # print(line.rstrip(), flush=True)
    # print(line.decode().split(' ', 1)[0], os.environ['BRANCH'])
    line_str = line.decode()
    cmd = ["docker", "push", line_str.split(' ', 1)[0] + ':' + os.environ['BRANCH']]
    pid_push_proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, shell=False)    
    while True:
        line2 = pid_push_proc.stdout.readline()
        if not line2:
            break
        print(line2.rstrip(), flush=True)
    pid_push_proc.wait()

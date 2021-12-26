import os
import subprocess
import shlex
import shutil

# shouldn't need to build out Rust since that's container build


def subprocess_run(command_string):
    pid_build_proc = subprocess.Popen(shlex.split(command_string),
                                      stdout=subprocess.PIPE, shell=False)
    while True:
        line = pid_build_proc.stdout.readline()
        if not line:
            break
        print(line.rstrip(), flush=True)
    pid_build_proc.wait()


# build dir to hold mediakraken data (docker data)
os.makedirs("/var/opt/mediakraken", exist_ok=True)
shutil.copy("../docker/test/*.yml", "/var/opt/mediakraken/")

# build out docker and docker-compose
print("build out docker and docker-compose")
subprocess_run('python3 ../docker_compose/mediakraken_setup.py')

# pull and start mailcow
print("pull and start mailcow")
subprocess_run('docker-compose -f ../docker/test/docker-compose-mailcow.yml pull')
os.chdir("/var/opt/mediakraken")
subprocess_run('docker-compose -f docker-compose-mailcow.yml up -d')

# build out test images
print("build out test images")
subprocess_run('python3 build_and_deploy.py -t')

# # build out BASE images
# print("build out base images")
# subprocess_run('python3 build_and_deploy.py -b')
#
# # pull the test images
# print("pull the test images")
# # why pull stuff I just built? subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull')
# subprocess_run('docker-compose -f ../docker/test/docker-compose.yml up -d')

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


# build dir(s) to hold mediakraken data (docker data)
os.makedirs("/var/opt/mediakraken/sonatype/deploy", exist_ok=True)
shutil.copy("../docker/test/mksonatype/deploy/*.kar", "/var/opt/mediakraken/sonatype/deploy/.")
os.makedirs("/var/opt/mediakraken/trac/projects", exist_ok=True)
shutil.copy("../docker/test/mktrac/.htpasswd", "/var/opt/mediakraken/trac/.")
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
os.chdir("/home/metaman/MediaKraken/docker_build")
subprocess_run('python3 build_and_deploy.py -t')

# pull the test images, for sonatype, etc
print("pull the test images")
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull')
os.chdir("/var/opt/mediakraken")
subprocess_run('docker-compose -f docker-compose.yml up -d')

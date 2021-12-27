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


# so the shutils will work
working_directory = os.getcwd().replace("docker_build", "")

print("Path:", os.getcwd())
# build dir(s) to hold mediakraken data (docker data)
os.makedirs("/var/opt/mediakraken/sonatype/deploy", exist_ok=True)
print("Path:", os.getcwd())
shutil.copytree(os.path.join(working_directory, "docker/test/mksonatype/deploy"),
                         "/var/opt/mediakraken/sonatype/deploy", dirs_exist_ok=True)

print("Path:", os.getcwd())
os.makedirs("/var/opt/mediakraken/trac/projects", exist_ok=True)
print("Path:", os.getcwd())
shutil.copy(os.path.join(working_directory, "docker/test/mktrac/.htpasswd"), "/var/opt/mediakraken/trac/.")
print("Path:", os.getcwd())
shutil.copy(os.path.join(working_directory, "docker/test/docker-compose.yml"), "/var/opt/mediakraken/.")
shutil.copy(os.path.join(working_directory, "docker/test/docker-compose-mailcow.yml"), "/var/opt/mediakraken/.")
shutil.copy(os.path.join(working_directory, "docker/test/docker-compose-thebuggenie.yml"), "/var/opt/mediakraken/.")
print("Path:", os.getcwd())

# build out docker and docker-compose
print("build out docker and docker-compose")
subprocess_run('python3 ../docker_compose/mediakraken_setup.py')

# pull and start mailcow
print("pull and start mailcow")
subprocess_run('docker-compose -f ../docker/test/docker-compose-mailcow.yml pull')
print("Path:", os.getcwd())
os.chdir("/var/opt/mediakraken")
print("Path:", os.getcwd())
subprocess_run('docker-compose -f docker-compose-mailcow.yml up -d')
print("Path:", os.getcwd())

# build out test images
print("build out test images")
os.chdir("/home/metaman/MediaKraken/docker_build")
print("Path:", os.getcwd())
subprocess_run('python3 build_and_deploy.py -t')

# pull the test images, for sonatype, etc
print("pull the test images")
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull')
print("Path:", os.getcwd())
os.chdir("/var/opt/mediakraken")
print("Path:", os.getcwd())
subprocess_run('docker-compose -f docker-compose.yml up -d')

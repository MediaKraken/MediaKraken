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

# build dir(s) to hold mediakraken data (docker data)
os.makedirs("/var/opt/mediakraken/sonatype/deploy", exist_ok=True)
shutil.copytree(os.path.join(working_directory, "docker/test/mksonatype/deploy"),
                         "/var/opt/mediakraken/sonatype/deploy", dirs_exist_ok=True)

os.makedirs("/var/opt/mediakraken/trac", exist_ok=True)
shutil.copy(os.path.join(working_directory, "docker/test/mktrac/.htpasswd"), "/var/opt/mediakraken/trac/.")
shutil.copy(os.path.join(working_directory, "docker/test/docker-compose.yml"), "/var/opt/mediakraken/.")
shutil.copy(os.path.join(working_directory, "docker/test/docker-compose-thebuggenie.yml"), "/var/opt/mediakraken/.")

# build out docker and docker-compose
print("build out docker and docker-compose")
subprocess_run('python3 ../docker_compose/mediakraken_setup.py')

# pull and start mailcow
print("pull and start mailcow")
os.chdir("/var/opt/mediakraken")
subprocess_run('git clone https://github.com/mailcow/mailcow-dockerized')
os.chdir("/var/opt/mediakraken/mailcow-dockerized")
subprocess_run('./generate_config.sh')
subprocess_run('docker-compose -f docker-compose.yml up -d')

# pull sonatype, teamcity and pgadmin images
os.chdir("/home/metaman/MediaKraken/docker_build")
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull mktestsonatype')
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull mktestpgadmin')
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull mktestteamcity')
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull mktestteamcityagent')
subprocess_run('docker-compose -f ../docker/test/docker-compose.yml pull mktestteamcitydb')

# pause user to setup sonatype
print("Access and configure sonatype at http://th-mkbuild-1.beaverbay.local:8081")
input("Press Enter to continue...after configuration of Alpine, Debian, CentOS, PyPi are completed")

# build out test images
print("build out test images")
subprocess_run('python3 build_and_deploy.py -t')

# start up the "test" images
os.chdir("/var/opt/mediakraken")
subprocess_run('docker-compose -f docker-compose.yml up -d')

print("setup has completed. Access Jenkins at http://th-mkbuild-1.beaverbay.local:8080 for build pipeline")
print("setup has completed. Access TeamCity at http://th-mkbuild-1.beaverbay.local:8111 for build pipeline")

import subprocess
import shlex

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


# build out docker and docker-compose
print("build out docker and docker-compose")
subprocess_run('python3 ../docker_compose/mediakraken_setup.py')

# pull and start mailcow
print("pull and start mailcow")
subprocess_run('docker-compose -t ./docker/test/docker-compose-mailcow.yml pull')
subprocess_run('docker-compose -t ./docker/test/docker-compose-mailcow.yml up -d')

# build out test images
print("build out test images")
subprocess_run('python3 ./docker_build/build_and_deploy.py -t')

# build out BASE images
print("build out base images")
subprocess_run('python3 ./docker_build/build_and_deploy.py -b')

# pull the test images
print("pull the test images")
subprocess_run('docker-compose -t ./docker/test/docker-compose.yml pull')
subprocess_run('docker-compose -t ./docker/test/docker-compose.yml up -d')

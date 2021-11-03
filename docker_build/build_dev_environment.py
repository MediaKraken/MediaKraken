import subprocess
import shlex

# shouldn't need to build out Rust since that's container build
# TODO do I really need a registry for the test/dev environment?


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

# build the jenkins build for mediakraken
print("build the jenkins build for mediakraken")
subprocess_run('docker build ../docker/test/mkjenkins/. -t mkjenkins')

# selenium mediakraken test image
print("selenium mediakraken test image")
subprocess_run('docker build ../docker/test/mkselenium/. -t mkselenium')

# trac instance for bug tracking
print("trac instance for bug tracking")
subprocess_run('docker build ../docker/test/mktrac/. -t mktrac')

import argparse
import os
import subprocess

CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken/src'))

parser = argparse.ArgumentParser(
    description='This program builds and deploys MediaKraken Libraries')
parser.add_argument('-e', '--email', required=False,
                    help='Send results email', action="store_true")
# set args.image variable if entered - ex. mkwebaxum
parser.add_argument('-l', '--library', metavar='library', required=False,
                    help='Library to build')
args = parser.parse_args()

# TODO check for failures?

# publish to kellnr
libs_to_publish = ["mk_lib_common",
                   "mk_lib_compression",
                   "mk_lib_filler",
                   "mk_lib_image",
                   "mk_lib_logging",
                   "mk_lib_rabbitmq",
                   "mk_lib_database",
                   "mk_lib_file",
                   "mk_lib_hash",
                   "mk_lib_network",
                   "mk_lib_hardware",
                   "mk_lib_metadata"]

if args.library:
    libs_to_publish = [args.library]

run_path = os.getcwd()
for lib_name in libs_to_publish:
    print(lib_name)
    os.chdir(run_path + "/../src/" + lib_name)
    # build the library
    pid_proc = subprocess.Popen(['cargo', 'build', '-p', lib_name])
    pid_proc.wait()
    # TODO send to mailhog the results, don't publish if failed
    # publish the libary
    pid_proc = subprocess.Popen(["cargo", "publish", "--registry", "kellnr", "-p", lib_name, "--allow-dirty", "--token=3dPvCZSd1V8xKpiCGqpXCBbkj4Jtyyqc"])
    pid_proc.wait()

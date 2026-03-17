import argparse
import os
import subprocess
import network_email
import shlex

try:
    from dotenv import load_dotenv
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt-get install python3-dotenv -y'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from dotenv import load_dotenv

CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken/src'))

parser = argparse.ArgumentParser(
    description='This program builds and deploys MediaKraken Libraries')
# set args.image variable if entered - ex. mkwebaxum
parser.add_argument('-l', '--library', metavar='library', required=False,
                    help='Library to build')
args = parser.parse_args()

# load .env stats
load_dotenv()

# publish to kellnr
libs_to_publish = ["ed2k",
                   "ssdp",
                   "weectrl",
                   "mk_lib_logging",
                   "mk_lib_common",
                   "mk_lib_compression",
                   "mk_lib_filler",
                   "mk_lib_image",
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
    pid_build_proc = subprocess.Popen(['cargo', 'build', '-p', lib_name],
                                      stdout=subprocess.PIPE,
                                      stderr=subprocess.PIPE,
                                      shell=False)
    (out, err) = pid_build_proc.communicate()
    email_body = err.decode("utf-8")
    subject_text = ' FAILED'
    if email_body.find('Finished') != -1 and email_body.find(' in') != -1:
        subject_text = ' SUCCESS'
        # publish the libary
        pid_push_proc = subprocess.Popen(["cargo", "publish", "--registry", "kellnr", "-p",
                                            lib_name, "--allow-dirty", "--token=Gandua8sU346qZkR41zFRMQiIyLvkFZ2"],
                stdout=subprocess.PIPE, shell=False)
        while True:
            line = pid_push_proc.stdout.readline()
            if not line:
                break
            print(line.rstrip(), flush=True)
        pid_push_proc.wait()
    # send success/fail email
    network_email.com_net_send_email(os.environ['MAILUSER'],
                                        os.environ['MAILPASS'],
                                        os.environ['MAILUSER'],
                                        'Build ' + lib_name + ' crate: '
                                        + subject_text,
                                        email_body,
                                        smtp_server=os.environ['MAILSERVER'],
                                        smtp_port=os.environ['MAILPORT'])

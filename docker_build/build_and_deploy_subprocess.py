# must login to docker hub first *IF* one wants to push to dockerhub
# docker login --username=mediakraken

import argparse
import os
import shlex
import shutil
import subprocess
import sys
import docker_images_list
import network_email

parser = argparse.ArgumentParser(
    description='This program builds and deploys MediaKraken')
# set args.image variable if entered - ex. mkwebaxum
parser.add_argument('-i', '--image', metavar='image', required=False,
                    help='Image to build')
parser.add_argument('-e', '--email', required=False,
                    help='Send results email', action="store_true")
parser.add_argument('-p', '--push', required=False,
                    help='Push images to Hub', action="store_true")
parser.add_argument('-r', '--rebuild', required=False,
                    help='Force rebuild with no cached layers', action="store_true")
parser.add_argument('-v', '--version', metavar='version', required=False,
                    help='The build version dev/prod or other branch')
args = parser.parse_args()

print('Number of arguments:', len(sys.argv), 'arguments.')
print('Argument List:', args)

# start
CWD_HOME_DIRECTORY = os.getcwd().rsplit('MediaKraken', 1)[0]
git_branch = args.version
if git_branch != 'prod':
    git_branch = 'dev'

if not os.path.exists(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken')):
    # backup to main dir with checkouts
    os.chdir(CWD_HOME_DIRECTORY)
    pid_proc = subprocess.Popen(
        shlex.split('git clone -b %s https://github.com/MediaKraken/MediaKraken'
                    % git_branch))
    pid_proc.wait()
else:
    if git_branch == 'prod':
        # cd to MediaKraken_Deployment dir
        os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken'))
        # pull down latest code
        pid_proc = subprocess.Popen(['git', 'pull'])
        pid_proc.wait()
        pid_proc = subprocess.Popen(['git', 'checkout', git_branch])
        pid_proc.wait()

if args.rebuild:
    # include pull to force update to new image
    docker_no_cache = '--pull --no-cache'
else:
    docker_no_cache = ''
if args.image is not None:
    print("Docker Image:", args.image)
    # do the actual build process for docker image
    os.chdir(os.path.join(CWD_HOME_DIRECTORY,
                          'MediaKraken/docker',
                          docker_images_list.DOCKER_IMAGES[args.image][1],
                          args.image))
    # flip the cargo files around
    try:
        shutil.copy(os.path.join(CWD_HOME_DIRECTORY,
                                 'MediaKraken/docker',
                                 docker_images_list.DOCKER_IMAGES[args.image][1],
                                 docker_images_list.DOCKER_IMAGES[args.image][0],
                                 'Cargo-docker.toml'), os.path.join(CWD_HOME_DIRECTORY,
                                                                    'MediaKraken/docker',
                                                                    docker_images_list.DOCKER_IMAGES[args.image][1],
                                                                    docker_images_list.DOCKER_IMAGES[args.image][0],
                                                                    'Cargo.toml'))
    except FileNotFoundError:
        pass
    # BuildKit is the default builder for users on Docker Desktop and Docker Engine v23.0 and later.
    # Let the mirror's be passed, if not used it will just throw a warning
    pid_build_proc = subprocess.Popen(shlex.split('docker build %s'
                                                  ' -t mediakraken/%s:%s'
                                                  ' --build-arg BRANCHTAG=%s'
                                                  ' --build-arg ALPMIRROR=%s'
                                                  ' --build-arg DEBMIRROR=%s'
                                                  ' --build-arg PIPMIRROR=%s'
                                                  ' --build-arg PIPMIRRORPORT=%s .' %
                                                  (docker_no_cache,
                                                   docker_images_list.DOCKER_IMAGES[args.image][0],
                                                   git_branch, git_branch,
                                                   docker_images_list.ALPINE_MIRROR,
                                                   docker_images_list.DEBIAN_MIRROR,
                                                   docker_images_list.PYPI_MIRROR,
                                                   docker_images_list.PYPI_MIRROR_PORT)),
                                      stdout=subprocess.PIPE,
                                      stderr=subprocess.PIPE,
                                      shell=False)
    (out, err) = pid_build_proc.communicate()
    # flip the cargo files back around
    try:
        shutil.copy(os.path.join(CWD_HOME_DIRECTORY,
                                 'MediaKraken/docker',
                                 docker_images_list.DOCKER_IMAGES[args.image][1],
                                 docker_images_list.DOCKER_IMAGES[args.image][0],
                                 'Cargo-local.toml'), os.path.join(CWD_HOME_DIRECTORY,
                                                                   'MediaKraken/docker',
                                                                   docker_images_list.DOCKER_IMAGES[args.image][1],
                                                                   docker_images_list.DOCKER_IMAGES[args.image][0],
                                                                   'Cargo.toml'))
    except FileNotFoundError:
        pass
    email_body = err.decode("utf-8")
    subject_text = ' FAILED'
    if email_body.find('Successfully tagged mediakraken') != -1 or email_body.find('writing image sha256') != -1:
        subject_text = ' SUCCESS'
        # push to remote repo
        if args.push:
            pid_push_proc = subprocess.Popen(
                shlex.split('docker push mediakraken/%s:%s'
                            % (docker_images_list.DOCKER_IMAGES[args.image][0],
                                git_branch)),
                stdout=subprocess.PIPE, shell=False)
            while True:
                line = pid_push_proc.stdout.readline()
                if not line:
                    break
                print(line.rstrip(), flush=True)
            pid_push_proc.wait()
    if args.email:
        # send success/fail email
        network_email.com_net_send_email(os.environ['MAILUSER'],
                                         os.environ['MAILPASS'],
                                         os.environ['MAILUSER'],
                                         'Build ' + args.version + ' image: '
                                         + docker_images_list.DOCKER_IMAGES[args.image][0]
                                         + subject_text,
                                         email_body,
                                         smtp_server=os.environ['MAILSERVER'],
                                         smtp_port=os.environ['MAILPORT'])

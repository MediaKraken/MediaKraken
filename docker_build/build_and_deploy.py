# must login to docker hub first *IF* one wants to push to dockerhub
# docker login --username=mediakraken

import argparse
import os
import shlex
import subprocess
import sys
import time

try:
    from dotenv import load_dotenv
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt-get install python3-dotenv -y'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from dotenv import load_dotenv

import docker_images_list

# TODO proxy docker build -t mediakraken/mkbase38py3 --build-arg http_proxy="http://proxyip:8080"
#  --build-arg ALPMIRROR=dl-cdn.alpinelinux.org --build-arg PIPMIRROR=pypi.python.org .

# build BASE images - first...as required for rest of images
# python3 build_and_deploy.py -b -v dev
# build rest of images
# python3 build_and_deploy.py -v dev

parser = argparse.ArgumentParser(
    description='This program builds and deploys MediaKraken')
parser.add_argument('-b', '--base', required=False,
                    help='Base images', action="store_true")
parser.add_argument('-c', '--core', required=False,
                    help='Core images', action="store_true")
parser.add_argument('-g', '--gamebase', required=False,
                    help='Game Base images', action="store_true")
parser.add_argument('-k', '--gameserver', required=False,
                    help='Game Server images', action="store_true")
parser.add_argument('-e', '--email', required=False,
                    help='Send results email', action="store_true")
# set args.image variable if entered - ex. mkwebaxum
parser.add_argument('-i', '--image', metavar='image', required=False,
                    help='Image to build')
parser.add_argument('-o', '--option', required=False,
                    help='Build option images', action="store_true")
parser.add_argument('-p', '--push', required=False,
                    help='Push images to Hub', action="store_true")
parser.add_argument('-r', '--rebuild', required=False,
                    help='Force rebuild with no cached layers', action="store_true")
parser.add_argument('-t', '--testing', required=False,
                    help='Build testing images', action="store_true")
parser.add_argument('-v', '--version', metavar='version', required=False,
                    help='The build version dev/prod or other branch')
args = parser.parse_args()

# load .env stats
load_dotenv()

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

# below is needed for the source sync to work!
os.chdir(os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken/docker_build'))
# sync the latest scratch OS into the image locations for build
pid_proc = subprocess.Popen(
    [os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken', 'docker_build/source_sync_local_lib.sh')])
pid_proc.wait()

images_to_build = []
# begin build process
if args.image:
    images_to_build.append(args.image)
else:
    if args.base:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "base":
                images_to_build.append(build_image)

    if args.core:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "core":
                images_to_build.append(build_image)

    if args.gamebase:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "game_base":
                images_to_build.append(build_image)

    if args.gameserver:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "game_server":
                images_to_build.append(build_image)

    if args.option:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "option":
                images_to_build.append(build_image)

    if args.testing:
        for build_image in docker_images_list.DOCKER_IMAGES:
            if docker_images_list.DOCKER_IMAGES[build_image][1] == "test":
                images_to_build.append(build_image)

print("To Build:", images_to_build)
if len(images_to_build):
    for build_image in images_to_build:
        # -p for push all the time for now
        # -e for email all the time for now
        print("Launching build for:", build_image)
        subprocess.Popen(['python3', os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken',
                                                  'docker_build/build_and_deploy_subprocess.py'), '-i', build_image, '-v', git_branch, '-e', '-p'])

processname = 'build_and_deploy_subprocess'
while 1:
    tmp = os.popen("ps -Af").read()
    proccount = tmp.count(processname)
    if proccount == 0:
        break
    time.sleep(5)

# purge the none images
pid_proc = subprocess.Popen(
    [os.path.join(CWD_HOME_DIRECTORY, 'MediaKraken', 'docker_build/purge_images_none.sh')])
pid_proc.wait()

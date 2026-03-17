# must login to docker hub first *IF* one wants to push to dockerhub
# docker login --username=mediakraken

import argparse
import asyncio
import os
import shlex
import subprocess
import sys

try:
    from dotenv import load_dotenv
except ModuleNotFoundError:
    install_pid = subprocess.Popen(
        shlex.split('apt-get install python3-dotenv -y'),
        stdout=subprocess.PIPE,
        shell=False
    )
    install_pid.wait()
    from dotenv import load_dotenv

import docker_images_list

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
parser.add_argument('-i', '--image', metavar='image', required=False,
                    help='Image to build')
parser.add_argument('-p', '--push', required=False,
                    help='Push images to Hub', action="store_true")
parser.add_argument('-r', '--rebuild', required=False,
                    help='Force rebuild with no cached layers', action="store_true")
parser.add_argument('-t', '--testing', required=False,
                    help='Build testing images', action="store_true")
parser.add_argument('-v', '--version', metavar='version', required=False,
                    help='The build version dev/prod or other branch')
args = parser.parse_args()

load_dotenv()

print('Number of arguments:', len(sys.argv), 'arguments.')
print('Argument List:', args)


def run_command(cmd, cwd=None):
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode != 0:
        raise RuntimeError(f"command failed: {' '.join(cmd)}")


async def run_build(build_image, git_branch, cwd_home_directory, semaphore):
    async with semaphore:
        print("Launching build for:", build_image)

        cmd = [
            'python3',
            os.path.join(
                cwd_home_directory,
                'MediaKraken',
                'docker_build',
                'build_and_deploy_subprocess.py'
            ),
            '-i', build_image,
            '-v', git_branch,
        ]

        if args.email:
            cmd.append('-e')
        if args.push:
            cmd.append('-p')
        if args.rebuild:
            cmd.append('-r')

        process = await asyncio.create_subprocess_exec(*cmd)
        return_code = await process.wait()

        if return_code != 0:
            raise RuntimeError(f"build failed for {build_image} with exit code {return_code}")

        print("Completed build for:", build_image)


async def main():
    cwd_home_directory = os.getcwd().rsplit('MediaKraken', 1)[0]

    git_branch = args.version
    if git_branch != 'prod':
        git_branch = 'dev'

    mediakraken_path = os.path.join(cwd_home_directory, 'MediaKraken')

    if not os.path.exists(mediakraken_path):
        os.chdir(cwd_home_directory)
        run_command([
            'git', 'clone', '-b', git_branch,
            'https://github.com/MediaKraken/MediaKraken'
        ])
    else:
        if git_branch == 'prod':
            os.chdir(mediakraken_path)
            run_command(['git', 'pull'])
            run_command(['git', 'checkout', git_branch])

    os.chdir(os.path.join(cwd_home_directory, 'MediaKraken', 'docker_build'))

    run_command([
        os.path.join(
            cwd_home_directory,
            'MediaKraken',
            'docker_build',
            'source_sync_local_lib.sh'
        )
    ])

    images_to_build = []

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

        if args.testing:
            for build_image in docker_images_list.DOCKER_IMAGES:
                if docker_images_list.DOCKER_IMAGES[build_image][1] == "test":
                    images_to_build.append(build_image)

    print("To Build:", images_to_build)

    if images_to_build:
        semaphore = asyncio.Semaphore(4)
        tasks = [
            run_build(build_image, git_branch, cwd_home_directory, semaphore)
            for build_image in images_to_build
        ]
        await asyncio.gather(*tasks)

    run_command([
        os.path.join(
            cwd_home_directory,
            'MediaKraken',
            'docker_build',
            'purge_images_none.sh'
        )
    ])


if __name__ == "__main__":
    asyncio.run(main())
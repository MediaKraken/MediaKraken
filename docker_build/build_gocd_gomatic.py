import shlex
import subprocess
import docker_images_list

try:
    from gomatic import ExecTask, GoCdConfigurator, HostRestClient
except ModuleNotFoundError:
    install_pid = subprocess.Popen(shlex.split('apt-get install -y python3-pip'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    install_pid = subprocess.Popen(shlex.split('pip3 install gomatic'),
                                   stdout=subprocess.PIPE, shell=False)
    install_pid.wait()
    from gomatic import ExecTask, GoCdConfigurator, HostRestClient

# TODO pip3 install pylint bandit pyflakes vulture dead bashate yamllint pydocstyle flawfinder isort
# TODO for python/linting....can't import
# TODO pip3 install pytest selenium psutil flask guessit gomatic
# TODO apt-get install -y python3-pip wget shellcheck cppcheck nodejs npm unzip
# TODO chmod 666 /var/run/docker.sock

# TODO npm install htmlhint -g

# TODO curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s
# TODO mv bin/dotenv-linter /usr/bin/.

# TODO npm install -g --save-dev stylelint stylelint-config-standard

# TODO curl -sL https://raw.githubusercontent.com/epi052/feroxbuster/master/install-nix.sh | bash
# TODO mv feroxbuster /usr/bin/.
"""
wget https://github.com/hadolint/hadolint/releases/download/v2.8.0/hadolint-Linux-x86_64
mv hadolint-Linux-x86_64 /usr/bin/hadolint
chmod +x /usr/bin/hadolint
"""

# connect to gocd instance
configurator = GoCdConfigurator(HostRestClient("th-mkbuild-1:8153"))
pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_code_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")

# start creating stages and jobs
stage = pipeline.ensure_stage("code_count")
job = stage.ensure_job("cloc_job")
job.add_task(ExecTask(['cloc', '.']))

stage = pipeline.ensure_stage("linting")
# If a directory is given instead of a filename, *.cpp, *.cxx, *.cc, *.c++, *.c,
# *.tpp, and *.txx files are checked recursively from the given directory.
# TODO should I use --quiet flag?
job = stage.ensure_job("c_cpp_cppcheck")
job.add_task(ExecTask(['cppcheck', '--enable=all', '--platform=unix64', '.']))
job = stage.ensure_job("c_cpp_flawfinder")
job.add_task(ExecTask(['flawfinder', '.']))

job = stage.ensure_job("dockerfile_hadolint")
# TODO use -t error later on to fail only on error or above
# TODO maybe use --no-fail.......but then it doesn't error and display nice in gocd
job.add_task(ExecTask(['bash', '-c', 'hadolint $(git ls-files | grep Dockerfile)']))

job = stage.ensure_job("python_pyflakes")
job.add_task(ExecTask(['pyflakes', '.']))  # it didn't like the git method
job = stage.ensure_job("python_pylint")
job.add_task(ExecTask(['bash', '-c', 'pylint $(git ls-files *.py)']))
job = stage.ensure_job("python_pydocstyle")
job.add_task(ExecTask(['bash', '-c', 'pydocstyle $(git ls-files *.py)']))
job = stage.ensure_job("python_isort")
job.add_task(ExecTask(['isort', '--check', '.']))

job = stage.ensure_job("shell_shellcheck")
job.add_task(ExecTask(['bash', '-c', 'shellcheck $(git ls-files *.sh)']))
job = stage.ensure_job("shell_bashate")
job.add_task(ExecTask(['bash', '-c', 'bashate $(git ls-files *.sh)']))

# job = stage.ensure_job("rust_clippy")
# job.add_task(ExecTask(['cloc', '.']))

job = stage.ensure_job("html_htmlhint")
job.add_task(ExecTask(['htmlhint', 'docker/core/mkwebapp/templates/**/*.html']))

job = stage.ensure_job("env_dotenv-linter")
job.add_task(ExecTask(['bash', '-c', 'dotenv-linter $(git ls-files | grep -F \.env)']))

job = stage.ensure_job("yml_yamllint")
job.add_task(ExecTask(['bash', '-c', 'yamllint $(git ls-files *.yml)']))

job = stage.ensure_job("css_stylelint")
job.add_task(ExecTask(['bash', '-c', 'npx stylelint docker/core/mkwebapp/static/**/*.css']))


stage = pipeline.ensure_stage("dead_code")
job = stage.ensure_job("python_vulture")
job.add_task(ExecTask(['bash', '-c', 'vulture $(git ls-files *.py)']))

job = stage.ensure_job("python_dead")
job.add_task(ExecTask(['bash', '-c', 'dead']))  # --files $(git ls-files *.py)'])) needs to be regex


stage = pipeline.ensure_stage("code_security")
job = stage.ensure_job("bandit_python")
job.add_task(ExecTask(['bandit', '-r', '.']))

pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_build_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")

stage = pipeline.ensure_stage("docker_build_base")
for build_group in (docker_images_list.STAGE_ONE_IMAGES,
                    docker_images_list.STAGE_TWO_IMAGES,
                    docker_images_list.STAGE_ONE_GAME_SERVERS,):
    for docker_images in build_group:
        job = stage.ensure_job("build_base_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_build_core")
for build_group in (docker_images_list.STAGE_CORE_IMAGES,):
    for docker_images in build_group:
        job = stage.ensure_job("build_core_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_build_games")
for build_group in (docker_images_list.STAGE_TWO_GAME_SERVERS,):
    for docker_images in build_group:
        job = stage.ensure_job("build_games_%s" % build_group[docker_images][0])
        job.add_task(ExecTask(['bash', '-c', 'docker build -t mediakraken/%s:refactor'
                                             ' --build-arg ALPMIRROR=%s'
                                             ' --build-arg DEBMIRROR=%s'
                                             ' --build-arg PIPMIRROR=%s'
                                             ' ./docker/%s/%s/.' % (build_group[docker_images][0],
                                                                    docker_images_list.ALPINE_MIRROR,
                                                                    docker_images_list.DEBIAN_MIRROR,
                                                                    docker_images_list.PYPI_MIRROR,
                                                                    build_group[docker_images][2],
                                                                    docker_images)]))


stage = pipeline.ensure_stage("docker_security")
job = stage.ensure_job("docker_dockerbench")
job.add_task(ExecTask(['./docker/test/docker_bench_security.sh']))


pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_security_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")
stage = pipeline.ensure_stage("security_mediakraken")
job = stage.ensure_job("mediakraken_start")
job.add_task(ExecTask(['./docker_compose/mediakraken_stop.sh']))
job = stage.ensure_job("security_feroxbuster")
job.add_task(ExecTask(['bash', '-c', 'feroxbuster -u https://th-mkbuild-1:8900'
                                     'x -x pdf -x js,html -x php txt json,docx']))

pipeline = configurator \
    .ensure_pipeline_group("MediaKraken") \
    .ensure_replacement_of_pipeline("mediakraken_test_pipeline") \
    .set_git_url("https://github.com/MediaKraken/MediaKraken")
stage = pipeline.ensure_stage("test_mediakraken")
# TODO test selenium
# TODO ab website
job = stage.ensure_job('mediakraken_stop')
job.add_task(ExecTask(['./docker_compose/mediakraken_stop.sh']))

# pipeline = configurator \
#     .ensure_pipeline_group("MediaKraken") \
#     .ensure_replacement_of_pipeline("mediakraken_deploy_pipeline") \
#     .set_git_url("https://github.com/MediaKraken/MediaKraken")
# TODO push to dockerhub

configurator.save_updated_config()

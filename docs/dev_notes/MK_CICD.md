= Description of validation and testing process via CI =

The Jenkins docker image I use is jenkins/jenkins:lts
    Then I install pip requirements file and other plugins

The ELK docker image I use is https://github.com/spujadas/elk-docker
    This is used to view logs as they are output via MK programs


CI/CD pipelines are built with Debian Bullseye

# setup the Debian environment
apt-get install -y python3-dotenv python3-pip wget cloc shellcheck \
  cppcheck nodejs npm unzip

# install the python tools 
pip3 install pylint pyflakes bandit vulture dead bashate yamllint \ 
  pydocstyle flawfinder isort pytest selenium psutil flask guessit gomatic

# Pipelines
## mediakraken_code_pipeline - This pipeline will cloc, lint and other static tests
### code_count
#### cloc
### linting
#### cppcheck - http://cppcheck.sourceforge.net/ - c/cpp static checker
#### flawfinder - https://dwheeler.com/flawfinder - c/cpp static flaw finder
#### hadolint
#### pyflakes - https://github.com/PyCQA/pyflakes - Python code linter
#### pylint - https://www.pylint.org/ - Python code linter
#### pydocstyle
#### isort
#### shellcheck
#### bashate
#### htmlhint
#### dotevn-linter
#### yamllint
#### npx styelint
### dead_code
#### vulture
#### dead
### code_security
#### bandit - https://github.com/PyCQA/bandit - Static Python security
## mediakraken_unit_test_pipeline - Run actual tests against code via pytest, cargo, etc
## mediakraken_build_pipeline - Build the Docker images
## mediakraken_security_pipeline - Run security tests against images and actual website
## mediakraken_test_pipeline - Run Selecium against entire website.
## mediakraken_deploy_pipeline - Push changes to DockerHub
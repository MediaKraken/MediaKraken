= Description of validation and testing process via CI =

The Jenkins docker image I use is jenkins/jenkins:lts
    Then I install pip requirements file and other plugins

The ELK docker image I use is https://github.com/spujadas/elk-docker
    This is used to view logs as they are output via MK programs

# debian bullseye
apt install python3-dotenv python3-pip wget cloc shellcheck
pip3 install pylint pyflakes bandit

Run the following tests from under the MediaKraken_CI
    python3 validate_code.py
#        Bandit (https://github.com/PyCQA/bandit) to find unsecured code - against MediaKraken source
#            pip3 install bandit==1.7.0
        # TODO FIX Graudit (https://github.com/wireghoul/graudit) - against MediaKraken_Deployment source
            cd ~
            git clone https://github.com/wireghoul/graudit
            ln -s ~/graudit/graudit /bin/graudit
        Vulture to find dead code - against MediaKraken source
            pip3 install vulture==2.3
        Taint to find unsecured code - against MediaKraken source
            pip3 install python-taint==0.42
    python3 validate_docker.py
#        Docker Bench Security - https://github.com/docker/docker-bench-security
#            docker_bench_security.sh from the above link
#        Hadolint (https://github.com/hadolint/hadolint) - lint your Dockerfile
#            # docker pull hadolint/hadolint
#            wget https://github.com/hadolint/hadolint/releases/download/v2.0.0/hadolint-Linux-x86_64
#            mv hadolint-Linux-x86_64 /usr/bin/hadolint
#            chmod +x /usr/bin/hadolint
        Trivy (https://github.com/aquasecurity/trivy) Vulnerability Scanner for Containers - against docker images - alerts on apps/packages installed in the OS
            apt-get install wget apt-transport-https gnupg lsb-release
            wget -qO - https://aquasecurity.github.io/trivy-repo/deb/public.key | apt-key add -
            echo deb https://aquasecurity.github.io/trivy-repo/deb $(lsb_release -sc) main | tee -a /etc/apt/sources.list.d/trivy.list
            apt-get update
            apt-get install trivy
            # to run as server
            trivy server --listen 0.0.0.0:9999
            trivy client --remote http://localhost:9999 alpine:3.10
    python3 validate_web_security
            nikto - https://github.com/sullo/nikto Nikto web server scanner
                git clone https://github.com/sullo/nikto
            mablanco/rapidscan - https://github.com/skavngr/rapidscan
            # TODO Sitadel - https://github.com/shenril/Sitadel
            Wapiti - https://wapiti.sourceforge.io/ - Wapiti allows you to audit the security of your websites or web applications.
                apt install wapiti
    python3 validate_web_selenium


Maybe:
https://github.com/PyCQA/isort
https://github.com/epi052/feroxbuster - website attacker
https://github.com/ffuf/ffuf - website attacker
https://github.com/vintasoftware/python-linters-and-code-analysis
https://github.com/analysis-tools-dev/static-analysis#python
https://github.com/Yelp/detect-secrets
https://github.com/pawamoy/dependenpy
https://github.com/almandin/fuxploider
https://github.com/drwetter/testssl.sh
https://github.com/facebook/pyre-check
https://github.com/xd009642/tarpaulin
https://github.com/myint/scspell --report-only
https://github.com/rust-lang/rust-clippy
metasploit
Clair - docker vuln scanner
    https://github.com/arminc/clair-scanner
    https://github.com/arminc/clair-local-scan
dagda - static analysis of known vulnerabilities, trojans, viruses, malware & other malicious threats in docker images/containers
    https://github.com/eliasgranderubio/dagda
pocsuite3 - remote vulnerability testing
    https://github.com/knownsec/pocsuite3
safety - pip requirements security check
    https://pyup.io/safety/
Sonarqube - code scanner
    https://docs.sonarqube.org/latest/setup/install-server/
Vuls - os scanner via ssh
    https://vuls.io/docs/en/tutorial-docker.html
    https://github.com/ishiDACo/vulsrepo
https://coala.io/#/home?lang=Python
http://jwilk.net/software/pydiatra
https://github.com/PyCQA/pydocstyle
https://github.com/pyupio/safety
https://github.com/PyCQA/flake8-bugbear

Nonproject:
core intruque = attack surface checker - looks for stuff running that's insecure
    https://core.intrigue.io/
archerysec - vuln db and monitor
    https://www.archerysec.com/index.html

implement
https://github.com/dotenv-linter/dotenv-linter - env file linter
https://github.com/wemake-services/dotenv-linter
https://github.com/wemake-services/wemake-python-styleguide
https://github.com/PyCQA/prospector
https://github.com/RetireJS/retire.js
https://github.com/channable/dbcritic
https://github.com/openstack/bashate
https://github.com/megalinter/megalinter/#run-mega-linter-locally


gocd:
cloc
pylint - python linter
    https://www.pylint.org/
pyflakes - python linter
    https://github.com/PyCQA/pyflakes
Hadolint (https://github.com/hadolint/hadolint) - lint your Dockerfile
    wget https://github.com/hadolint/hadolint/releases/download/v2.0.0/hadolint-Linux-x86_64
    mv hadolint-Linux-x86_64 /usr/bin/hadolint
    chmod +x /usr/bin/hadolint
Shellcheck - shell script checker
#    https://github.com/anordal/shellharden
    https://github.com/koalaman/shellcheck
Bandit (https://github.com/PyCQA/bandit) to find unsecured code - against MediaKraken source
            pip3 install bandit==1.7.0
cppcheck - c/c++ code checker
    http://cppcheck.sourceforge.net/
Docker Bench Security - https://github.com/docker/docker-bench-security
    docker_bench_security.sh from the above link
= Description of validation and testing process via CI =

The Jenkins docker image I use is jenkins/jenkins:lts
    Then I install pip requirements file and other plugins

The ELK docker image I use is https://github.com/spujadas/elk-docker
    This is used to view logs as they are output via MK programs

# debian bullseye
apt-get install python3-dotenv python3-pip wget cloc shellcheck
pip3 install pylint pyflakes bandit

Run the following tests from under the MediaKraken_CI
    python3 validate_code.py
#        Bandit (https://github.com/PyCQA/bandit) to find unsecured code - against MediaKraken source
#            pip3 install bandit==1.7.0
        # TODO FIX Graudit (https://github.com/wireghoul/graudit) - against MediaKraken_Deployment source
            cd ~
            git clone https://github.com/wireghoul/graudit
            ln -s ~/graudit/graudit /bin/graudit
#        Vulture to find dead code - against MediaKraken source
#            pip3 install vulture==2.3
#        Taint to find unsecured code - against MediaKraken source
#            pip3 install python-taint==0.42  (dead project)
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
                apt-get install wapiti
    python3 validate_web_selenium


lint:
https://github.com/PyCQA/prospector   - python linter
https://coala.io/#/home?lang=Python     - linter for alot of langs and web code
https://github.com/mrtazz/checkmake         - experimental linter/analyzer for Makefiles     I don't have any makefiles
https://github.com/myint/scspell --report-only      - find spelling mistakes
https://hegel.js.org/               - static type checker for js
https://github.com/rslint/rslint        - JavaScript and TypeScript linter
https://flow.org/en/docs/cli/           -FLOW IS A STATIC TYPE CHECKER FOR JAVASCRIPT.
https://github.com/wemake-services/wemake-python-styleguide    - uber linter?
https://github.com/slomkowski/nginx-config-formatter

security:
https://github.com/Yelp/detect-secrets      - detecting and preventing secrets in code
https://github.com/Checkmarx/kics           - Find security vulnerabilities, compliance issues, and infrastructure misconfigurations early in the development cycle of your infrastructure-as-code (docke, k8s)
https://github.com/anchore/anchore-engine
https://github.com/anchore/syft    - docker
https://github.com/anchore/grype   - docker


security - app running
https://github.com/proferosec/log4jScanner
https://github.com/ffuf/ffuf - website attacker
https://github.com/drwetter/testssl.sh          - check port for tls/ssl support and some flaws
https://github.com/almandin/fuxploider      - File upload vulnerability scanner and exploitation tool.
https://github.com/johannesschaefer/webnettools
https://www.offensive-security.com/metasploit-unleashed/wmap-web-scanner/
https://www.zaproxy.org/   - web scanner
https://www.arachni-scanner.com/    - web scanner
http://rgaucher.info/beta/grabber/     - web scanner
https://cirt.net/nikto2     - web scanner
https://github.com/vwt-digital/sec-helpers/tree/master    - web scanner
https://subgraph.com/vega/   - web scanner
http://w3af.org/    - web scanner
https://github.com/stark0de/nginxpwner  - nginx boo boo finder
https://github.com/shenril/Sitadel  - attacker

howto:
https://medium.com/techbull/deploying-elk-stack-for-apache-logs-analysis-3d23648dafa6


https://github.com/xd009642/tarpaulin     - code coverage for rust
https://github.com/jhinch/nginx-linter  - nginx linter
https://dev.to/simdrouin/validate-your-nginx-configuration-files-easily-with-docker-4ihi  - validate ngindx config

https://github.com/phan/phan            - static analyzer for PHP
https://psalm.dev/docs/running_psalm/installation/     - php
https://github.com/phpstan/phpstan          - PHP Static Analysis Tool
https://github.com/collections/code-quality-in-php
https://www.npmjs.com/package/uglify-js

https://github.com/twbs/bootlint            - HTML linter for Bootstrap proje    eol?
https://github.com/yandex/gixy   - Gixy is a tool to analyze Nginx configuration.   2-years ago
https://github.com/wemake-services/dotenv-linter     - env file linter, same exe name as other linter
https://sqlmap.org/
https://github.com/rapid7/metasploit-framework/wiki/Nightly-Installers
https://www.arachni-scanner.com/
https://nmap.org/book/install.html
https://www.open-scap.org/getting-started/
https://wazuh.com/
http://w3af.org/
https://github.com/rust-lang/rustfmt
https://crates.io/crates/cargo-tarpaulin
https://github.com/sullo/nikto
https://subgraph.com/vega/download.html
https://github.com/future-architect/vuls
https://developers.google.com/closure/compiler   - Closure Compiler is a tool for making JavaScript download and run faster
https://html-validate.org/              -Offline HTML5 validator
https://validator.github.io/validator/      -  catch unintended mistakes in your HTML, CSS, and SVG
https://github.com/purcell/sqlint       -  SQL linter supporting ANSI and PostgreSQL
https://github.com/jarulraj/sqlcheck    -  identify anti-patterns in SQL queries
https://github.com/vintasoftware/python-linters-and-code-analysis
https://github.com/analysis-tools-dev/static-analysis#python
https://github.com/facebook/pyre-check
https://github.com/rust-lang/rust-clippy
metasploit
Clair - docker vuln scanner
    https://github.com/arminc/clair-scanner
    https://github.com/arminc/clair-local-scan
dagda - static analysis of known vulnerabilities, trojans, viruses, malware & other malicious threats in docker images/containers
    https://github.com/eliasgranderubio/dagda
pocsuite3 - remote vulnerability testing
    https://github.com/knownsec/pocsuite3
Sonarqube - code scanner
    https://docs.sonarqube.org/latest/setup/install-server/
    docker run -d --name sonarqube -e SONAR_ES_BOOTSTRAP_CHECKS_DISABLE=true -p 9000:9000 sonarqube:latest    (admin/admin login)
Vuls - os scanner via ssh
    https://vuls.io/docs/en/tutorial-docker.html
    https://github.com/ishiDACo/vulsrepo
https://github.com/pyupio/safety                - Safety checks your installed dependencies for known security vulnerabilities - python
https://github.com/PyCQA/flake8-bugbear         - A plugin for Flake8 finding likely bugs and design problems in your program.
https://github.com/RetireJS/retire.js              - scanner detecting the use of JavaScript libraries with known vulnerabilities
https://github.com/channable/dbcritic           - dbcritic finds problems in a database schema.
https://github.com/megalinter/megalinter/#run-mega-linter-locally
cargo audit
cargo spellcheck
cargo udeps
https://www.codacy.com/product     - security/etc scanner for alot of langs
https://github.com/ZupIT/horusec   - static code analys on alot of langs
https://huskyci.opensource.globo.com/docs/quickstart/overview    - huskyCI is an open-source tool that orchestrates security tests inside CI pipelines of multiple projects and centralizes all results into a database for further analysis and metrics.
https://pyre-check.org/     - type checker python3
https://lgtm.com/help/lgtm/getting-started    - dont' I have a lgtm account?
https://gitlab.com/pwoolcoc/soup
https://github.com/deadlinks/cargo-deadlinks
https://sunjay.dev/2016/07/25/rust-code-coverage


Nonproject:
core intruque = attack surface checker - looks for stuff running that's insecure
    https://core.intrigue.io/
    docker run -e LANG=C.UTF-8 --memory=8g -p 0.0.0.0:7777:7777 -v /var/opt/mediakraken/intrigue-data:/data -it "intrigueio/intrigue-core:latest"
$ signed up via github.........doenst' appear to bring in my code
https://app.snyk.io/org/spootdev/manage/members



implemented in gocd:
https://github.com/adrienverge/yamllint     - yml linter
cloc - code line counter
pylint - python linter
    https://www.pylint.org/
pyflakes - python linter
    https://github.com/PyCQA/pyflakes
Hadolint (https://github.com/hadolint/hadolint) - lint your Dockerfile
    wget https://github.com/hadolint/hadolint/releases/download/v2.0.0/hadolint-Linux-x86_64
    mv hadolint-Linux-x86_64 /usr/bin/hadolint
    chmod +x /usr/bin/hadolint
Shellcheck - shell script checker
    https://github.com/koalaman/shellcheck
Bandit (https://github.com/PyCQA/bandit) to find unsecured code - against MediaKraken source
            pip3 install bandit==1.7.0
cppcheck - c/c++ code checker
    http://cppcheck.sourceforge.net/
Docker Bench Security - https://github.com/docker/docker-bench-security
    docker_bench_security.sh from the above link
Vulture to find dead code - against MediaKraken source
    pip3 install vulture==2.3
dead - to find dead python code
https://github.com/openstack/bashate    - pep8 for bash
https://github.com/HTMLHint/HTMLHint        - static code analysis tool you need for your HTML
https://github.com/dotenv-linter/dotenv-linter       - env file linter
https://github.com/PyCQA/pydocstyle             - docstring style checker for python code
https://dwheeler.com/flawfinder/     - c/c++ flaw finder
https://github.com/stylelint/stylelint      - A mighty, modern linter that helps you avoid errors and enforce conventions in your styles. (css, etc)
https://github.com/epi052/feroxbuster - website attacker
https://github.com/PyCQA/isort   - sorts and updates your imports


standalone app:
archerysec - vuln db and monitor
    https://www.archerysec.com/index.html
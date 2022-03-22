pipeline {
  agent any
  stages {
      parallel {
    stage('Code Validation') {
        stage('Python Lint') {
          steps {
            sh 'python3 -m pylint --output-format=parseable --fail-under=<threshold value> module --msg-template="{path}:{line}: [{msg_id}({symbol}), {obj}] {msg}" | tee pylint.log || echo "pylint exited with $?"'
            echo "linting Success, Generating Report"
            recordIssues enabledForFailure: true, aggregatingResults: true, tool: pyLint(pattern: 'pylint.log')
          }
        }

        stage('Sloccount') {
          steps {
            sh 'sloccount --duplicates --wide --details path-to-code/ > sloccount.sc'
            sloccountPublish encoding: '', pattern: ''
          }
        }

        stage('Dockerfile Lint') {
          steps {
            sh 'git --version'
          }
        }

      }
    }

    stage('Base Stage 1') {
      steps {
        sh 'git --version'
      }
    }

    stage('Base Stage 2') {
      steps {
        sh 'git --version'
      }
    }

    stage('Build Core') {
      steps {
        sh 'git --version'
      }
    }

    stage('Build Game Stage 1') {
      steps {
        sh 'git --version'
      }
    }

    stage('Build Game Core') {
      steps {
        sh 'git --version'
      }
    }

    stage('Docker Security') {
      parallel {
        stage('Docker Bench') {
          steps {
            sh 'git --version'
          }
        }

        stage('Trivy') {
          steps {
            sh 'git --version'
          }
        }

      }
    }

    stage('Start MediaKraken') {
      steps {
        sh 'git --version'
      }
    }

    stage('Website Testing') {
      parallel {
        stage('ab') {
          steps {
            sh 'ab -n 10000 -c 30 https://localhost:8900/'
          }
        }

        stage('Nikto') {
          steps {
            sh 'perl ./nikto/program/nikto.pl -h https://localhost:8900'
          }
        }

        stage('Rapidscan') {
          steps {
            sh 'docker run -ti mablanco/rapidscan https://localhost:8900'
          }
        }

        stage('wapiti') {
          steps {
            sh 'wapiti -u https://localhost:8900'
          }
        }

      }
    }

    stage('Stop MediaKraken') {
      steps {
        sh 'git --version'
      }
    }

  }
}
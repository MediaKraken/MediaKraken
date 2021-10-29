pipeline {
  agent any
  stages {
    stage('Code Validation') {
      parallel {
        stage('Python Lint') {
          steps {
            sh 'git --version'
          }
        }

        stage('Sloccount') {
          steps {
            sh 'git --version'
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
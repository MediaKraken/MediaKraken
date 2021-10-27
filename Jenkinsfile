pipeline {
  agent any
  stages {
    stage('Git Pull') {
      steps {
        git(branch: 'refactor', credentialsId: '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', url: 'https://github.com/MediaKraken/MediaKraken')
      }
    }

  }
}
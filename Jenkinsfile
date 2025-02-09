pipeline {
    agent any
    stages {
        stage('Check Rust Dependencies') {
            steps { 
                script {
                    sh 'cargo audit'
                }
            }
        }
        stage('Run Test Cases') {
            steps { 
                script {
                    sh 'cargo test'
                }
            }
        }
        stage('Code Coverage') {
            steps {
                script {
                    catchError(message: 'Oops will be fixed in the future', stageResult: 'UNSTABLE') {
                        // some block
                        sh 'cargo llvm-cov --doctests --html'
                    }
                }
            }
        }
    }
}




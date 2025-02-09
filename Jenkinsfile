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
        stage('Welcome Step1') {
            steps { 
                echo 'Welcome to LambdaTest'
            }
        }
        stage('Welcome Step2') {
            steps { 
                echo 'Welcome to LambdaTest'
            }
        }
    }
}




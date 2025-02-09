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
        stage('Welcome Step2') {
            steps { 
                echo 'Welcome to LambdaTest'
            }
        }
    }
}




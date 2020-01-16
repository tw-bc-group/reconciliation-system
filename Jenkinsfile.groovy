pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stages {
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "make fmt"
            }
        }
        stage('Clippy') {
            steps {
                sh "make clippy"
            }
        }
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        stage("Move plugins to test folder") {
            steps {
                sh "cp ${env.WORKSPACE}/target/debug/*.so ${env.WORKSPACE}/reconciliation/tests/plugin/"
            }
        }
        stage('Test') {
            steps {
                sh "make test"
            }
        }
    }
}
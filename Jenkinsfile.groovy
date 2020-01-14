pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stage('Rustfmt') {
        steps {
            // The build will fail if rustfmt thinks any changes are required.
            sh "rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo fmt --all"
        }
    }
    stages {
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        stage("Move plugins to test folder") {
            steps {
                sh "cp /var/lib/jenkins/workspace/reconciliation-system/target/debug/*.so /var/lib/jenkins/workspace/reconciliation-system/reconciliation/tests/flush/plugin/"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "rustup component add clippy --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo clippy --all"
            }
        }
    }
}
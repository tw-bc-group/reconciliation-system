pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stages {
        stage("Prepare") {
            steps {
                sh "apt update && apt install clang llvm"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo fmt --all"
            }
        }
        stage('Clippy') {
            steps {
                sh "rustup component add clippy --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo clippy --all"
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
                sh "cargo test"
            }
        }
    }
}
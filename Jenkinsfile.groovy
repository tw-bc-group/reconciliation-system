pipeline {
    agent {
        docker {
            image "zzybing/rust-libclang:latest"
        }
    }
    stages {
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo fmt --all"
            }
        }
        // why comment this state: Running with more than 40 minutes on CI.
        // stage('Clippy') {
        //     steps {
        //         sh "rustup component add clippy --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo clippy --all"
        //     }
        // }
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
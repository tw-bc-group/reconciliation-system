pipeline {
    agent none
    stages {
        stage('Rustfmt') {
            agent {
                docker {
                    image "zzybing/rust-libclang:latest"
                }
            }
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "rustup component add rustfmt --toolchain 1.40.0-x86_64-unknown-linux-gnu; cargo fmt --all"
            }
        }
        // stage('Build and Test') {
        //     agent {
        //         docker {
        //             image "zzybing/rust-libclang:latest"
        //         }
        //     }
        //     steps {
        //         sh "cargo build"
        //         sh "cp ${env.WORKSPACE}/target/debug/*.so ${env.WORKSPACE}/reconciliation/tests/plugin/"
        //         sh "cargo test"
        //     }
        // }
        stage('Dockerize') {
            agent any
            steps {
                sh 'make image'
                sh 'make publish'
            }
        }
    }
}
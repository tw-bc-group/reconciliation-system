pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build --release"
            }
        }
        stage("Move libplugin_sample.so file to test folder") {
            steps {
                sh "mv /var/lib/jenkins/workspace/reconciliation-system/target/release/libplugin_sample.so /var/lib/jenkins/workspace/reconciliation-system/reconciliation/tests/plugin/"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "cargo +nightly clippy --all"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "cargo +nightly fmt --all"
            }
        }
        stage('Doc') {
            steps {
                sh "cargo doc"
                // We run a python `SimpleHTTPServer` against
                // /var/lib/jenkins/jobs/<repo>/branches/master/javadoc to display our docs
                step([$class    : 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll   : false])
            }
        }
    }
}
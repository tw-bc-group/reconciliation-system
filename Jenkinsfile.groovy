pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh "/home/ubuntu/.cargo/bin/cargo +stable build"
            }
        }
        stage("Move libplugin_sample.so file to test folder") {
            steps {
                sh "mv /var/lib/jenkins/workspace/reconciliation-system/target/debug/libplugin_sample.so /var/lib/jenkins/workspace/reconciliation-system/reconciliation/tests/plugin/"
            }
        }
        stage('Test') {
            steps {
                sh "/home/ubuntu/.cargo/bin/cargo +stable test"
            }
        }
        stage('Clippy') {
            steps {
                sh "/home/ubuntu/.cargo/bin/cargo +nightly clippy --all"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are required.
                sh "/home/ubuntu/.cargo/bin/cargo +nightly fmt --all"
            }
        }
        stage('Doc') {
            steps {
                sh "/home/ubuntu/.cargo/bin/cargo doc"
                // We run a python `SimpleHTTPServer` against
                // /var/lib/jenkins/jobs/<repo>/branches/master/javadoc to display our docs
                step([$class    : 'JavadocArchiver',
                      javadocDir: 'target/doc',
                      keepAll   : false])
            }
        }
    }
}
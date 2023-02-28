# About this directory
This directory contains secrets that are used by the application. These secrets are not checked into source control, but are instead stored in a secure location. The secrets are then copied into this directory by a script that runs as part of the build process.

# Secrets
The following secrets are used by the application:
- api_key.json: Contains the API key(s) used to authenticate with the third party APIs. Request an API key from APILayer and tankerkoenig.de and store it in a separate file without the `invalid_` prefix.

- azure_key.json: Contains the credentials to authenticate with a azure account. Replace with the details from Azure and store it in a separate file without the `invalid_` prefix.

- kafka_key.json: Contains the credentials to authenticate with a local kafka installation. Replace with the details from your Kafka cluster and store it in a separate file without the `invalid_` prefix.

# Important
You can see the invalid keys as they are part of the integration tests.
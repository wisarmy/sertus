# Sertus
Sertus is a service status monitoring tool that supports multiple checkers, including processes, scripts, APIs, and more.

# Features
- [x] Supports Prometheus metrics
- [ ] Enables flows with concurrency
- [ ] Allows for setting intervals for flows
- [ ] Divides flows configuration into multiple flow config files
- [ ] Supports script checkers
- [ ] Supports API checkers

# Get Started
To get started with Sertus, follow these simple steps:

1. Initialize the Sertus configuration by running the following command:
```shell
sertus init
```
2. Edit the configuration file to specify the task checkers for flows.

3. Start the Sertus daemon by running the following command:
``` shell
sertus daemon
```

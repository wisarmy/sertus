# Sertus
Sertus is a service status monitoring tool that supports multiple checkers, including processes, scripts, APIs, and more.

# Features
- [x] Supports Prometheus metrics
- [x] Enables flows with concurrency
- [x] Allows for setting intervals for flows
- [ ] Divides flows configuration into multiple flow config files
- [x] Supports script checkers
- [ ] Supports API checkers

# Get Started
To get started with Sertus, follow these simple steps:

1. Initialize the Sertus configuration by running the following command:
```shell
sertus init
```
The default configuration file `~/.sertus/config.toml` will be generated.

2. Edit the configuration file to specify the task checkers for flows.


3. Start the Sertus daemon by running the following command:
``` shell
sertus daemon
```

# Configuration Example
```toml
[metrics]
addr = "127.0.0.1:9296"
bucket = "sertus"

[[flows]]
name = "flow 1"
interval = 3

[[flows.tasks]]
name = "task 1"

[flows.tasks.checker]
type = "ProcessChecker"
prefix = "process prefix"

[[flows.tasks]]
name = "task 2"

[flows.tasks.checker]
type = "ScriptChecker"
path = "/root/.sertus/echo.sh"
```



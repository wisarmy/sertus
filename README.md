# Sertus
Sertus is a service status monitoring tool that supports multiple checkers, including processes, scripts, APIs, and more.

# Features
- [x] Supports Prometheus metrics
    - [x] Supports Prometheus metrics server
    - [x] Supports Prometheus push gateway
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
# use metrics server
[metrics.Server]
addr = "127.0.0.1:9296"
bucket = "sertus"

# or use prometheus push gateway
#[metrics.PushGateway]
#endpoint = "http://127.0.0.1:9091/metrics/job/sertus/instance/127.0.0.1"
#interval = 10


[[flows]]
name = "flow 1"
interval = 3

[[flows.tasks]]
name = "check process"
checker.ProcessChecker = { prefix = "process prefix" }

[[flows.tasks]]
name = "check script"
checker.ScriptChecker = { path = "~/.sertus/scripts/script.sh" }
```



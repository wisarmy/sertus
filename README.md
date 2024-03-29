# Sertus
Sertus is a service status monitoring tool written in rust that supports multiple checkers, including processes, scripts with metrics, APIs, and more.

# Features
- [x] Supports Prometheus metrics
    - [x] Supports Prometheus metrics server
    - [x] Supports Prometheus push gateway
- [x] Enables flows with concurrency
- [x] Allows for setting intervals for flows
- [ ] Divides flows configuration into multiple flow config files
- [x] Supports script checkers
    - [x] Supports custom metrics
- [ ] Supports API checkers

# Get Started
To get started with Sertus, follow these simple steps:

1. Initialize the Sertus configuration by running the following command:
```shell
sertus init
// or interactively create config
sertus init -i
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

# or use prometheus push gateway
#[metrics.PushGateway]
#endpoint = "http://127.0.0.1:9091/metrics/job/sertus/instance/127.0.0.1"
#interval = Option<u64> default 10(s)
#idle_timeout = Option<u64> default 60(s)

[[flows]]
name = "flow 1"
interval = 3

[[flows.tasks]]
name = "check process"
checker.ProcessChecker = { prefix = "process prefix" }

[[flows.tasks]]
name = "check script"
checker.ScriptChecker = { path = "~/.sertus/scripts/script.sh" }

# the bin is option, default "bash", if use python:
[[flows.tasks]]
name = "check py script"
checker.ScriptChecker = { path = "~/.sertus/scripts/script.py" , bin = "python3"}
```
# ScriptChecker & Metrics labels
By default, Metrics has labels for flow and task. If you want to add custom labels in ScriptChecker, you should echo like `#label {k=v, x=y}` in your script.
Example:
```bash
#!/bin/bash

# stdout 
echo "#label {k=v, x=y}"
echo "ok msg"

# or stderr
echo "#label {k=v, x=y}" >&2
echo "err msg" >&2
exit 1
```
# ScriptChecker & Custom Metrics
If you want to add custom metrics in ScriptChecker, you should echo like `#metric key type {k=v, x=y} value` in your script. In addition, the key will be prefixed with `sertus_`.
Example:
```bash
#!/bin/bash
# the real key is sertus_key_xxx 
echo "#metric key_xxx gauge {k=v, x=y} 1.0"
echo "#metric key_xxx counter {k=v, x=y} 1"
```
:warning: ScriptChecker fails in any of the following cases:
- has stderr
- exit code != 0

# Metrics 
`sertus_flow_task_status` gauge:
- `1.0` task succeed
- `0.0` task failed
- `-1.0` task checker happened unknown error, please check the sertus log



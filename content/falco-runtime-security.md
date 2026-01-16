+++
title = "Falco: The Security Camera for Your Kubernetes Cluster"
template = "page.html"
date = 2024-07-22T13:15:00Z
[taxonomies]
tags = ["kubernetes", "security", "falco", "runtime"]
[extra]
summary = "Detecting suspicious behavior in real-time because scanners only catch known problems"
+++

Static scanning is great. You find known CVEs, misconfigurations, bad practices. But what happens when someone exploits a zero-day? Or a developer does something they shouldn't? Or that "trusted" third-party container starts behaving weird?

You need eyes on what's actually happening at runtime.

# The Gap in Static Security

Most Kubernetes security focuses on prevention:
- Image scanning catches known vulnerabilities
- Admission controllers block bad configurations
- RBAC restricts who can do what

But none of these help when something slips through. And something always slips through.

[Falco](https://falco.org/) fills this gap. It watches system calls in real-time and alerts when something looks wrong.

# What Falco Does

From the [Falco documentation](https://falco.org/docs/): "Falco is a cloud native security tool that provides runtime security across hosts, containers, Kubernetes, and cloud environments."

It monitors syscalls at the kernel level. When a container:
- Reads `/etc/shadow`
- Opens a shell
- Makes unexpected network connections
- Modifies system binaries
- Does basically anything suspicious

Falco sees it and alerts immediately.

# How It Works

Falco uses a kernel driver (either a kernel module or eBPF probe) to capture syscalls. Rules define what's suspicious:

```yaml
- rule: Shell Spawned in Container
  desc: Detect shell spawned in a container
  condition: >
    spawned_process and container and
    shell_procs
  output: >
    Shell spawned in container
    (user=%user.name container_id=%container.id
    container_name=%container.name shell=%proc.name)
  priority: WARNING
```

When the condition matches, you get the alert with all the context.

The [intro to K8s security with Falco](https://falco.org/blog/intro-k8s-security-monitoring/) explains: "Out of the box, Falco examines syscalls to alert you to any suspicious activity. And, since containers share the same kernel as their host, Falco can monitor not only activity on the host but also activity on all of the containers running on that host."

# Installing Falco

The [official Helm chart](https://github.com/falcosecurity/charts) is the easiest path:

```bash
helm repo add falcosecurity https://falcosecurity.github.io/charts
helm install falco falcosecurity/falco \
  --namespace falco --create-namespace
```

There are deployment options too—you can run Falco directly on nodes instead of as Kubernetes pods if you don't want to give a pod privileged access to monitor everything else.

# Default Rules Are a Good Start

Falco ships with [80+ rules](https://github.com/falcosecurity/rules) covering common threats:

- Container drift (new executables appearing)
- Privilege escalation attempts
- Sensitive file access
- Suspicious network activity
- Known malware patterns

Start with defaults, tune based on your environment's noise level.

# Real Scenarios

**Detect a compromised container phoning home:**
```yaml
- rule: Unexpected Outbound Connection
  desc: Detect outbound connections from containers
  condition: >
    outbound and container and
    not (fd.sip in (allowed_outbound_ips))
  output: >
    Unexpected outbound connection
    (container=%container.name ip=%fd.sip)
  priority: WARNING
```

**Catch someone exec'ing into production:**
```yaml
- rule: Terminal Shell in Container
  desc: Interactive shell opened in container
  condition: >
    spawned_process and container and
    shell_procs and proc.tty != 0
  output: >
    Interactive shell opened
    (user=%user.name container=%container.name)
  priority: NOTICE
```

**Detect cryptominer behavior:**
```yaml
- rule: Detect Cryptomining
  desc: Detect cryptomining processes
  condition: >
    spawned_process and
    (proc.name in (crypto_miners) or
     proc.cmdline contains "stratum+tcp")
  output: >
    Cryptominer detected (proc=%proc.cmdline)
  priority: CRITICAL
```

# Kubernetes Audit Log Integration

Falco can also consume [Kubernetes audit logs](https://falco.org/docs/event-sources/kubernetes-audit/) for API-level visibility:

```yaml
- rule: Create Privileged Pod
  desc: Detect privileged pod creation
  condition: >
    kevt and kcreate and pod and
    ka.req.pod.containers.privileged=true
  output: >
    Privileged pod created
    (user=%ka.user.name pod=%ka.target.name)
  priority: WARNING
```

Now you see both syscall-level behavior AND API-level actions.

# Responding to Alerts

Alerts alone don't help if nobody sees them. Falco integrates with:

- Slack, PagerDuty, email
- SIEM systems (Splunk, Elastic)
- Falcosidekick for flexible routing
- Kubernetes response systems

For critical alerts, you can even trigger automated responses (kill the pod, isolate the node). Though be careful with automation—false positives happen.

# Layered Security

As the [Kubernetes Goat scenario](https://madhuakula.com/kubernetes-goat/docs/scenarios/scenario-18/runtime-security-monitoring-and-detection-in-kubernetes-cluster-using-falco/welcome/) points out: "The strongest security strategies combine: Static analysis to catch known vulnerabilities before deployment; Runtime detection to catch real-time attacks after deployment."

Falco doesn't replace scanners or policies—it backs them up. It catches what slips through.

# Getting Started

1. Install Falco with Helm
2. Review the default rules, disable noisy ones
3. Route alerts to somewhere visible
4. Create custom rules for your specific concerns
5. Tune over time as you learn normal behavior

The goal isn't zero alerts—it's actionable alerts for real threats.

**Sources:**
- [Falco Documentation](https://falco.org/docs/)
- [The Falco Project](https://falco.org/)
- [Falco GitHub Repository](https://github.com/falcosecurity/falco)
- [Introduction to Kubernetes Security Monitoring with Falco](https://falco.org/blog/intro-k8s-security-monitoring/)
- [Falco on GKE](https://falco.org/blog/falco-on-gke/)

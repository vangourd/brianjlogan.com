+++
title = "Pod Security Standards: Stop Running Everything as Root"
template = "page.html"
date = 2023-08-20T10:15:00Z
[taxonomies]
tags = ["kubernetes", "security", "containers"]
[extra]
summary = "The built-in admission controller that stops privileged containers before they start"
+++

I once inherited a cluster where every single pod ran as root with privileged access. "It was the only way to make it work," they said. It's amazing what you can justify when you're in a hurry.

Turns out Kubernetes has built-in tools to prevent this. Pod Security Standards have been GA since 1.25 and they're way simpler than the old PodSecurityPolicy mess.

# The Problem

By default, Kubernetes lets you run containers however you want. Privileged mode? Sure. Root user? No problem. Host network? Go for it.

This is bad. A compromised privileged container basically owns the node. And from the node, lateral movement to other pods and the control plane becomes much easier.

# Three Security Levels

[Pod Security Standards](https://kubernetes.io/docs/concepts/security/pod-security-standards/) define three levels:

**Privileged**: No restrictions. The "I know what I'm doing" mode. Use sparingly.

**Baseline**: Prevents known privilege escalations. Blocks privileged containers, hostPath mounts, host networking. This is reasonable for most workloads.

**Restricted**: Maximum security. Non-root, drops all capabilities, read-only root filesystem. What you should aim for.

# Enforcing With Pod Security Admission

The [Pod Security Admission controller](https://kubernetes.io/docs/concepts/security/pod-security-admission/) is built into Kubernetes. You just need to enable it.

Three enforcement modes:

- **enforce**: Violations reject the pod
- **audit**: Violations log but allow
- **warn**: Violations show warning but allow

Start with warn/audit to see what would break, then move to enforce.

Apply via namespace labels:

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: production
  labels:
    pod-security.kubernetes.io/enforce: restricted
    pod-security.kubernetes.io/audit: restricted
    pod-security.kubernetes.io/warn: restricted
```

Now any pod in `production` that violates restricted standards gets rejected.

# A Practical Migration

Don't flip everything to restricted overnight. That's a great way to break production at 2 AM.

1. **Label namespaces with warn mode first:**

```yaml
labels:
  pod-security.kubernetes.io/warn: restricted
```

2. **Deploy and watch for warnings.** You'll see which pods need fixes.

3. **Fix the violations.** Usually this means:
   - Setting `runAsNonRoot: true`
   - Dropping capabilities
   - Removing privileged flags
   - Using emptyDir instead of hostPath

4. **Move to enforce** once violations are fixed.

# What Restricted Actually Requires

The [restricted policy](https://kubernetes.io/docs/concepts/security/pod-security-standards/#restricted) requires:

```yaml
spec:
  securityContext:
    runAsNonRoot: true
    seccompProfile:
      type: RuntimeDefault
  containers:
  - name: app
    securityContext:
      allowPrivilegeEscalation: false
      capabilities:
        drop:
          - ALL
      readOnlyRootFilesystem: true  # recommended
```

Most well-designed apps can run this way. If yours can't, that's worth investigating.

# Exemptions

Some things legitimately need elevated privileges (CNI plugins, monitoring agents, etc.). You can configure exemptions at the cluster level or use the Baseline level for infrastructure namespaces.

Just don't use exemptions as an excuse to skip security for everything. That defeats the purpose.

# Why Not Just Use OPA/Kyverno?

You can, and those tools are more flexible. But Pod Security Admission is:

- Built in (no extra components)
- Simple to configure (just namespace labels)
- Well documented and supported

For many teams, it's enough. Add policy engines when you need more granular control.

# The Real Win

The best part isn't just blocking bad configs. It's that developers start thinking about security earlier. "Why does my container need root?" becomes a question they ask during development, not after an incident.

Apply baseline to everything. Get to restricted where you can. Your future self will thank you when there's a container escape CVE and you're already protected.

**Sources:**
- [Pod Security Standards - Kubernetes](https://kubernetes.io/docs/concepts/security/pod-security-standards/)
- [Pod Security Admission - Kubernetes](https://kubernetes.io/docs/concepts/security/pod-security-admission/)
- [Enforce Pod Security Standards - Kubernetes](https://kubernetes.io/docs/tasks/configure-pod-container/enforce-standards-admission-controller/)
- [Apply Pod Security Standards at Cluster Level - Kubernetes](https://kubernetes.io/docs/tutorials/security/cluster-level-pss/)

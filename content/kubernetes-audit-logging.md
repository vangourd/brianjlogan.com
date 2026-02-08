+++
title = "Kubernetes Audit Logs: Know What Happened Before You Need To"
template = "page.html"
date = 2023-04-05T16:30:00Z
[taxonomies]
dates = ["2023-04"]
tags = ["kubernetes", "security", "observability"]
[extra]
summary = "Because 'who deleted production?' shouldn't be unanswerable"
+++

Someone deleted a ConfigMap in production. The app crashed. Everyone's panicking. "Who did this?"

Silence. Nobody knows. There are no logs.

This is why you enable audit logging before you need it.

# What Audit Logs Capture

[Kubernetes auditing](https://kubernetes.io/docs/tasks/debug/debug-cluster/audit/) records all requests to the API server. Who did what, when, and how.

From the docs: "Kubernetes auditing provides a security-relevant, chronological set of records documenting the sequence of actions in a cluster."

That means:
- User authentication attempts
- Resource creation/modification/deletion
- RBAC permission checks
- Exec into pods
- Secret access
- Everything that touches the API

# Setting It Up (Self-Managed Clusters)

You need two things: an audit policy and API server flags.

**The audit policy** defines what to log:

```yaml
apiVersion: audit.k8s.io/v1
kind: Policy
rules:
# Log pod exec and attach at RequestResponse level
- level: RequestResponse
  resources:
  - group: ""
    resources: ["pods/exec", "pods/attach"]

# Log secret access at Metadata level (don't log contents!)
- level: Metadata
  resources:
  - group: ""
    resources: ["secrets"]

# Log all other requests at Request level
- level: Request
  resources:
  - group: ""
  - group: "apps"
  - group: "batch"
```

**Audit levels:**
- `None`: Don't log
- `Metadata`: Log request metadata (user, timestamp, resource) but not request/response body
- `Request`: Log metadata + request body
- `RequestResponse`: Log everything

**Important:** Never log Secrets at RequestResponse level. You'll have plaintext secrets in your audit logs. Metadata level is enough to know who accessed them.

**API server flags:**

```
--audit-policy-file=/etc/kubernetes/audit-policy.yaml
--audit-log-path=/var/log/kubernetes/audit.log
--audit-log-maxage=30
--audit-log-maxbackup=10
--audit-log-maxsize=100
```

# Managed Clusters

Good news: EKS, GKE, and AKS provide audit logging out of the box. You just need to enable it.

**GKE:** Audit logs go to Cloud Logging automatically. [GKE audit logging docs](https://cloud.google.com/kubernetes-engine/docs/how-to/audit-logging) explain how to access them.

**EKS:** Enable control plane logging for the "audit" log type. Logs go to CloudWatch.

**AKS:** Enable diagnostic settings to send audit logs to Log Analytics.

# What to Look For

[Datadog's guide on key audit logs](https://www.datadoghq.com/blog/key-kubernetes-audit-logs-for-monitoring-cluster-security/) highlights important events:

**Privileged pod creation:**
```json
{
  "verb": "create",
  "objectRef": {"resource": "pods"},
  "requestObject": {
    "spec": {
      "containers": [{
        "securityContext": {"privileged": true}
      }]
    }
  }
}
```

**Exec into pods:**
```json
{
  "verb": "create",
  "objectRef": {"resource": "pods", "subresource": "exec"},
  "user": {"username": "jane@example.com"}
}
```

**Secret access:**
```json
{
  "verb": "get",
  "objectRef": {"resource": "secrets", "name": "database-credentials"},
  "user": {"username": "unknown-service-account"}
}
```

# Automated Detection

Don't just collect logs—alert on suspicious patterns:

- Service accounts accessing secrets they don't normally access
- Exec into production pods outside maintenance windows
- Creation of privileged pods
- ClusterRoleBinding changes
- Failed authentication attempts

Tools like [Falco](https://falco.org/) can monitor audit logs and alert in real-time:

```yaml
- rule: K8s Secret Access
  desc: Detect any access to cluster secrets
  condition: >
    kevt and secret and
    kactivity in (get, list)
  output: >
    Secret accessed (user=%ka.user.name secret=%ka.target.name)
  priority: WARNING
```

# Storage and Retention

Audit logs grow fast. Plan for:

- **Volume:** Busy clusters generate gigabytes daily
- **Retention:** Compliance often requires 90+ days
- **Search:** You need to actually query these logs

Send them to a log aggregation system (Elasticsearch, Loki, CloudWatch, etc.) with proper retention policies and alerting.

# The "Before You Need It" Part

Here's the thing: you won't realize you need audit logs until something goes wrong. By then it's too late.

Set it up now:
1. Enable audit logging
2. Ship logs to a searchable store
3. Set up basic alerts for high-risk events
4. Test by querying who did what yesterday

When someone asks "who deleted production?" you'll have the answer.

**Sources:**
- [Auditing - Kubernetes Documentation](https://kubernetes.io/docs/tasks/debug/debug-cluster/audit/)
- [GKE Audit Logging](https://cloud.google.com/kubernetes-engine/docs/how-to/audit-logging)
- [GKE Audit Policy](https://cloud.google.com/kubernetes-engine/docs/concepts/audit-policy)
- [Key Kubernetes Audit Logs for Monitoring Cluster Security - Datadog](https://www.datadoghq.com/blog/key-kubernetes-audit-logs-for-monitoring-cluster-security/)

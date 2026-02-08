+++
title = "Kubernetes RBAC Without Shooting Yourself in the Foot"
template = "page.html"
date = 2024-02-22T14:45:00Z
[taxonomies]
dates = ["2024-02"]
tags = ["kubernetes", "security", "rbac"]
[extra]
summary = "How to give users k8s API access without handing them the keys to everything"
+++

I've seen too many clusters where everyone has cluster-admin. "It's easier," they say. "We trust our team," they say. Then someone accidentally deletes a production namespace and suddenly trust isn't the issue anymore.

Kubernetes RBAC is one of those things that seems complicated until you actually use it. Then it becomes one of the most valuable security tools you have.

# The Basics (Skip If You Know This)

[RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) has four key resources:

- **Role**: Permissions within a namespace
- **ClusterRole**: Permissions cluster-wide
- **RoleBinding**: Grants a Role to users in a namespace
- **ClusterRoleBinding**: Grants a ClusterRole cluster-wide

The distinction matters. Namespace-scoped bindings provide natural isolation. Cluster-wide bindings should be rare and heavily scrutinized.

# Start With Least Privilege

The [official Kubernetes docs](https://kubernetes.io/docs/concepts/security/rbac-good-practices/) are clear: "When assigning permissions in an RBAC role, use the principle of least privilege and grant the minimum permissions needed."

Kubernetes has default ClusterRoles that are tempting:

- `cluster-admin`: Full access to everything. **Never** bind this to regular users.
- `admin`: Full access within a namespace, including RBAC. Rarely appropriate.
- `edit`: Modify most resources. Often too broad.
- `view`: Read-only access. Useful starting point.

These are convenience shortcuts, not security recommendations.

# A Practical Role

Say your devs need to deploy apps and debug issues in their namespace:

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: team-alpha
  name: developer
rules:
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch", "create", "update", "patch"]
- apiGroups: [""]
  resources: ["pods"]
  verbs: ["get", "list", "watch"]
- apiGroups: [""]
  resources: ["pods/log"]
  verbs: ["get"]
- apiGroups: [""]
  resources: ["pods/exec"]
  verbs: ["create"]
- apiGroups: [""]
  resources: ["configmaps", "secrets"]
  verbs: ["get", "list", "watch", "create", "update", "patch", "delete"]
- apiGroups: [""]
  resources: ["services"]
  verbs: ["get", "list", "watch"]
```

Notice what's missing: no ability to delete deployments (prevents accidents), no access to other namespaces, no cluster-level resources, no ability to modify RBAC itself.

# Bind to Groups, Not Users

Hard-coding usernames is unmaintainable. Bind to groups from your identity provider instead:

```yaml
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: team-alpha-developers
  namespace: team-alpha
subjects:
- kind: Group
  name: "team-alpha-devs"  # From OIDC group claim
  apiGroup: rbac.authorization.k8s.io
roleRef:
  kind: Role
  name: developer
  apiGroup: rbac.authorization.k8s.io
```

When someone joins or leaves, update group membership in your IdP, not Kubernetes.

# Service Account Gotchas

Most pods shouldn't need API access. Set `automountServiceAccountToken: false` by default. Only create dedicated service accounts for apps that actually need the API:

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: config-watcher
  namespace: team-alpha
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  namespace: team-alpha
  name: configmap-reader
rules:
- apiGroups: [""]
  resources: ["configmaps"]
  verbs: ["get", "list", "watch"]
```

As [Wiz's RBAC guide](https://www.wiz.io/academy/container-security/kubernetes-rbac-best-practices) notes: "Do not use the default service account, instead define a dedicated service account for each of your pods that need to access the kube-apiserver."

# Auditing What You Have

Check what a user can actually do:

```bash
# Check if a user can perform an action
kubectl auth can-i create deployments --namespace team-alpha --as=jane@example.com

# List all permissions for a user
kubectl auth can-i --list --namespace team-alpha --as=jane@example.com
```

There are also tools like `kubectl-who-can` that show which subjects have specific permissions.

# Common Mistakes

**Wildcards everywhere.** Rules like `resources: ["*"]` grant access to everything, including future resource types. Be explicit.

**Ignoring subresources.** Permissions on `pods` don't include `pods/log` or `pods/exec`. These need explicit grants.

**Giving CI/CD cluster-admin.** Your deployment pipeline doesn't need god-mode. Create a specific role for deployments only.

**Static service account tokens.** Kubernetes 1.24+ creates time-limited tokens by default. Don't recreate the old pattern.

# Keep It in Git

RBAC should be managed as code. Version it, review changes like production code. A misconfigured binding can have broader impact than a bug in a single app.

```
rbac/
├── cluster-roles/
│   └── monitoring-reader.yaml
├── teams/
│   ├── team-alpha/
│   │   ├── role.yaml
│   │   └── binding.yaml
│   └── team-beta/
│       └── ...
```

The time investment is real but the alternative—overly permissive access everywhere—is how "RBAC Buster" style attacks happen. That's not hypothetical; it's documented.

**Sources:**
- [Using RBAC Authorization - Kubernetes](https://kubernetes.io/docs/reference/access-authn-authz/rbac/)
- [Role Based Access Control Good Practices - Kubernetes](https://kubernetes.io/docs/concepts/security/rbac-good-practices/)
- [Best practices for GKE RBAC - Google Cloud](https://docs.cloud.google.com/kubernetes-engine/docs/best-practices/rbac)
- [Kubernetes RBAC Best Practices - Wiz](https://www.wiz.io/academy/container-security/kubernetes-rbac-best-practices)

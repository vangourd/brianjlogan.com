+++
title = "Kubernetes Secrets Aren't Secret (Here's How to Fix That)"
template = "page.html"
date = 2024-01-15T08:30:00Z
[taxonomies]
tags = ["kubernetes", "security", "secrets"]
[extra]
summary = "base64 is not encryption and other uncomfortable truths about k8s secrets"
+++

I had to explain to a junior dev why their database password was visible to anyone with kubectl access. "But it's a Secret!" they said. Yeah, about that...

Kubernetes Secrets are, by default, just base64 encoded. That's not encryption. Anyone with API access can decode them. Anyone with etcd access can read them in plaintext. It's one of those things that sounds secure until you think about it for five seconds.

# The Problem in Detail

As [Auth0's guide](https://auth0.com/blog/kubernetes-secrets-management/) puts it: "Kubernetes offers a way to store sensitive data using the Secret object. While it's better than nothing, it is not really a secret, as it is just base64 encoded strings that anyone with access to the cluster or the code can decode."

```bash
# Anyone with kubectl access can do this
kubectl get secret my-secret -o jsonpath='{.data.password}' | base64 -d
```

Oops.

And if etcd isn't encrypted at rest (it often isn't by default), those "secrets" are sitting in plaintext on disk.

# Encryption at Rest (The Minimum)

At minimum, enable [encryption at rest](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/) for etcd. This is Kubernetes native—configure the API server with an encryption provider.

Managed clusters (EKS, GKE, AKS) usually handle this. Self-managed clusters? You need to set it up manually.

This helps with physical security and backups, but doesn't solve the "anyone with API access can read them" problem.

# Sealed Secrets: GitOps-Friendly Option

[Sealed Secrets](https://github.com/bitnami-labs/sealed-secrets) from Bitnami solves the "how do I store secrets in Git?" problem.

The idea:
1. Encrypt secrets with a cluster's public key
2. Commit the encrypted form to Git (safe!)
3. Controller in cluster decrypts at runtime

```bash
# Encrypt a secret
kubeseal --format yaml < my-secret.yaml > sealed-secret.yaml

# Commit sealed-secret.yaml to Git - it's safe
git add sealed-secret.yaml
```

[SUSE's comparison](https://www.suse.com/c/rancher_blog/managing-sensitive-data-in-kubernetes-with-sealed-secrets-and-external-secrets-operator-eso/) notes: "Sealed Secrets is an open-source Kubernetes controller and a client-side CLI tool from Bitnami that aims to solve the 'storing secrets in Git' part of the problem."

**Downsides:** Cluster-specific encryption (migrating secrets between clusters is tricky). Best for deployment-time secrets, not dynamic ones.

# External Secrets Operator: The Enterprise Choice

[External Secrets Operator](https://external-secrets.io/) (ESO) syncs secrets from external stores (AWS Secrets Manager, Vault, Azure Key Vault, etc.) into Kubernetes.

```yaml
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: database-credentials
spec:
  refreshInterval: 1h
  secretStoreRef:
    name: aws-secrets-manager
    kind: SecretStore
  target:
    name: db-secret
  data:
  - secretKey: password
    remoteRef:
      key: prod/database/password
```

The secret lives in AWS Secrets Manager (or wherever). ESO creates and updates the Kubernetes Secret automatically. Your app just reads the normal Secret.

**Why this is better:**
- Centralized secret management across clusters
- Proper secret rotation
- Audit trails in the external store
- No secrets in Git, even encrypted

# Which Should You Use?

Per [this comparison](https://shivanium.medium.com/managing-secrets-in-kubernetes-sealed-secrets-vs-external-secrets-operator-vs-csi-driver-52dd298e983a):

- **Sealed Secrets**: Small teams, GitOps workflows, static secrets, minimal dependencies
- **External Secrets Operator**: Larger teams, dynamic secrets, multiple clusters, existing secret management infrastructure

If you already have HashiCorp Vault or AWS Secrets Manager, ESO is the obvious choice.

# HashiCorp Vault Direct Integration

Some teams skip Kubernetes Secrets entirely and inject secrets directly from Vault using the [Vault Agent Sidecar](https://developer.hashicorp.com/vault/docs/platform/k8s/injector). Secrets never exist as Kubernetes objects.

More complex but also more secure if you have Vault expertise.

# Practical Steps

1. **Enable etcd encryption at rest.** This is table stakes.

2. **Audit who can read Secrets.** Restrict RBAC—not everyone needs `get secrets` permission.

3. **Pick a secrets management approach.** Sealed Secrets for simple cases, ESO for production.

4. **Don't commit plaintext secrets to Git.** Ever. Even "temporarily."

5. **Rotate secrets.** If you're using ESO, set reasonable refresh intervals.

The goal is removing the assumption that "Secret" means "secure." It doesn't by default. You have to make it secure.

**Sources:**
- [Kubernetes Secrets Management - Auth0](https://auth0.com/blog/kubernetes-secrets-management/)
- [Encrypting Confidential Data at Rest - Kubernetes](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/)
- [External Secrets Operator](https://external-secrets.io/)
- [Managing Secrets: Sealed Secrets vs ESO vs CSI Driver](https://shivanium.medium.com/managing-secrets-in-kubernetes-sealed-secrets-vs-external-secrets-operator-vs-csi-driver-52dd298e983a)
- [Managing Sensitive Data with Sealed Secrets and ESO - SUSE](https://www.suse.com/c/rancher_blog/managing-sensitive-data-in-kubernetes-with-sealed-secrets-and-external-secrets-operator-eso/)

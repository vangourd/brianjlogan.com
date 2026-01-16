+++
title = "GitOps with ArgoCD Changed How I Think About Deployments"
template = "page.html"
date = 2023-09-27T10:00:00Z
[taxonomies]
tags = ["kubernetes", "gitops", "argocd", "devops"]
[extra]
summary = "Why git as the source of truth makes Kubernetes actually manageable"
+++

I used to `kubectl apply` my way through deployments. Manifests lived wherever, CI pushed changes directly, and when something went wrong the answer to "what's deployed?" was always "uhh, let me check."

Then I discovered GitOps and ArgoCD. Now git is the source of truth, deployments are automatic, and rollback is just `git revert`.

# What GitOps Actually Means

Traditional CI/CD pushes: pipeline builds, then deploys to targets. The pipeline is imperative—it executes steps that change state.

GitOps inverts this. A controller pulls:

1. Desired state lives in git (manifests, Helm charts, Kustomize)
2. [ArgoCD](https://argo-cd.readthedocs.io/en/stable/) watches the repo for changes
3. ArgoCD applies changes to make cluster match git
4. Drift is automatically corrected

From the [ArgoCD docs](https://argo-cd.readthedocs.io/en/stable/): "Application definitions, configurations, and environments should be declarative and version controlled. Application deployment and lifecycle management should be automated, auditable, and easy to understand."

The git repo becomes the only interface for deployments.

# Why This Is Better

**Auditability.** Every deployment is a git commit. Who deployed what, when, why? `git log` tells you.

**Reproducibility.** Cluster state is defined in git. Disaster recovery = git clone + ArgoCD install.

**Drift correction.** Manual `kubectl` changes get reverted. The cluster always matches git.

**Easy rollbacks.** `git revert` + push. ArgoCD sees the change and applies previous state.

**Security.** Developers commit to git; they don't need cluster credentials. Fewer people with kubectl access.

# Getting Started

Install ArgoCD:

```bash
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
```

Define an Application (what to deploy and where):

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-app
  namespace: argocd
spec:
  project: default
  source:
    repoURL: https://github.com/myorg/k8s-manifests.git
    targetRevision: main
    path: apps/my-app
  destination:
    server: https://kubernetes.default.svc
    namespace: production
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
```

This says: take manifests from `apps/my-app` in my git repo, deploy to `production` namespace, automatically sync when git changes, and self-heal if anything drifts.

# Repository Structure

[ArgoCD best practices](https://argo-cd.readthedocs.io/en/stable/user-guide/best_practices/) recommend separating config from application source code:

> "Using a separate Git repository to hold your Kubernetes manifests... There will be times when you wish to modify just the manifests without triggering an entire CI build."

A structure that works well:

```
k8s-manifests/
├── apps/
│   ├── frontend/
│   │   ├── deployment.yaml
│   │   └── kustomization.yaml
│   └── backend/
│       └── ...
├── infrastructure/
│   ├── cert-manager/
│   └── ingress-nginx/
└── overlays/
    ├── dev/
    ├── staging/
    └── production/
```

Use Kustomize overlays for environment differences. Base manifests in `apps/`, environment-specific patches in `overlays/`.

# Multi-Cluster Management

One ArgoCD instance can manage multiple clusters:

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-app-prod
spec:
  source:
    path: overlays/production
    targetRevision: release-1.5
  destination:
    server: https://production-cluster.example.com
---
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: my-app-staging
spec:
  source:
    path: overlays/staging
    targetRevision: main
  destination:
    server: https://staging-cluster.example.com
```

Promoting from staging to production? Change `targetRevision` in the prod Application. That's a PR.

# Handling Secrets

Don't commit plaintext secrets to git. Options:

- **Sealed Secrets**: Encrypt with cluster key, commit encrypted form
- **External Secrets Operator**: Sync from Vault/AWS Secrets Manager/etc.
- **SOPS**: Encrypt values in YAML files

I use External Secrets Operator with AWS Secrets Manager. Secrets never touch git, even encrypted.

# Useful Patterns

**App of Apps**: One Application that deploys other Applications. Bootstrap a cluster with one resource.

**ApplicationSets**: Generate Applications from templates. Deploy same app to multiple clusters without repetition.

**Sync Waves**: Control deployment order. Infrastructure before apps.

# The Workflow Change

Before GitOps:
1. Build in CI
2. Push image to registry
3. CI updates manifests and applies directly
4. Hope nothing went wrong
5. When it does, dig through CI logs

After GitOps:
1. Build in CI
2. Push image to registry
3. CI creates PR to update image tag in git repo
4. PR reviewed and merged
5. ArgoCD syncs automatically
6. Rollback = revert commit

The visibility improvement alone is worth it. ArgoCD's dashboard shows exactly what's deployed, what's out of sync, and what's healthy.

# Getting Started

[ArgoCD Getting Started guide](https://argo-cd.readthedocs.io/en/stable/getting_started/) is solid. Start with:

1. Install ArgoCD
2. Move one app's manifests to a git repo
3. Create an Application resource
4. Watch it sync
5. Make a change in git, watch it apply
6. `kubectl edit` something directly, watch ArgoCD revert it

That last one is when it clicks. Git is truth. Everything else is temporary.

**Sources:**
- [ArgoCD Documentation](https://argo-cd.readthedocs.io/en/stable/)
- [ArgoCD Best Practices](https://argo-cd.readthedocs.io/en/stable/user-guide/best_practices/)
- [ArgoCD Getting Started](https://argo-cd.readthedocs.io/en/stable/getting_started/)
- [ArgoCD GitHub](https://github.com/argoproj/argo-cd)
- [ArgoCD Best Practices - Codefresh](https://codefresh.io/blog/argo-cd-best-practices/)

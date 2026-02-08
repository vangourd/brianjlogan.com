+++
title = "Policy as Code: Stop Bad Configs Before They Hit Your Cluster"
template = "page.html"
date = 2024-09-12T11:00:00Z
[taxonomies]
dates = ["2024-09"]
tags = ["kubernetes", "security", "opa", "kyverno", "policy"]
[extra]
summary = "Using OPA Gatekeeper and Kyverno to enforce security policies automatically"
+++

Code review catches bad application logic. But who catches the deployment that requests 100 CPU cores? Or the pod running as root? Or the ingress exposing an internal service to the internet?

Kubernetes admission controllers. Specifically, policy engines like OPA Gatekeeper and Kyverno that let you define "what's allowed" as code.

# How Admission Controllers Work

When you `kubectl apply` something, it goes to the API server. Before the object is persisted, [admission controllers](https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/) can:

- **Validate:** Accept or reject the request
- **Mutate:** Modify the request before persisting

Policy engines hook into this to enforce whatever rules you define.

# OPA Gatekeeper

[OPA Gatekeeper](https://www.openpolicyagent.org/docs/kubernetes) is OPA (Open Policy Agent) purpose-built for Kubernetes. You write policies in Rego (OPA's language) and apply them as CRDs.

From the [OPA docs](https://www.openpolicyagent.org/docs/kubernetes): "Gatekeeper combines OPA with a Kubernetes-native layer that includes CRDs and a cluster admission controller implementation to enforce policies on Kubernetes resources."

Example: Block privileged containers

First, a constraint template (the policy logic):

```yaml
apiVersion: templates.gatekeeper.sh/v1
kind: ConstraintTemplate
metadata:
  name: k8spspprivileged
spec:
  crd:
    spec:
      names:
        kind: K8sPSPPrivileged
  targets:
    - target: admission.k8s.gatekeeper.sh
      rego: |
        package k8spspprivileged
        violation[{"msg": msg}] {
          c := input.review.object.spec.containers[_]
          c.securityContext.privileged
          msg := sprintf("Privileged container not allowed: %v", [c.name])
        }
```

Then a constraint (where to apply it):

```yaml
apiVersion: constraints.gatekeeper.sh/v1beta1
kind: K8sPSPPrivileged
metadata:
  name: no-privileged-containers
spec:
  match:
    kinds:
      - apiGroups: [""]
        kinds: ["Pod"]
```

Now any privileged pod gets rejected with a clear error message.

**Pros:** Very powerful, Rego is flexible, large policy library available
**Cons:** Rego has a learning curve, more complex to author policies

# Kyverno

[Kyverno](https://kyverno.io/) takes a different approach: policies are just YAML. No new language to learn.

Same example—block privileged containers:

```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: disallow-privileged
spec:
  validationFailureAction: enforce
  rules:
  - name: no-privileged
    match:
      resources:
        kinds:
        - Pod
    validate:
      message: "Privileged containers are not allowed"
      pattern:
        spec:
          containers:
          - securityContext:
              privileged: "!true"
```

Much more readable if you know Kubernetes manifests.

From the [Kyverno intro](https://kyverno.io/): "With Kyverno, policies are managed as Kubernetes resources and no new language is required to write policies."

**Pros:** YAML-native (no new language), easier to get started, mutations are straightforward
**Cons:** Less flexible for complex logic than Rego

# Which One?

Per [this comparison](https://dev.to/hkhelil/admission-controllers-in-kubernetes-opa-gatekeeper-kyverno-and-azure-policy-add-on-for-aks-which-one-wins-237d):

| Aspect | OPA Gatekeeper | Kyverno |
|--------|----------------|---------|
| Policy Language | Rego | YAML |
| Learning Curve | Steeper | Gentle |
| Flexibility | Very high | High |
| Kubernetes Native | Via CRDs | Fully native |

**Choose Gatekeeper if:**
- You already use OPA elsewhere
- You need complex policy logic
- Your team is comfortable with Rego

**Choose Kyverno if:**
- You want to get started quickly
- Team prefers staying in YAML-land
- Simpler policies are sufficient

Both are production-ready and actively maintained.

# Common Policies to Implement

Start with these:

**Require resource limits:**
```yaml
# Kyverno
validate:
  message: "CPU and memory limits required"
  pattern:
    spec:
      containers:
      - resources:
          limits:
            memory: "?*"
            cpu: "?*"
```

**Block latest tag:**
```yaml
validate:
  message: "Using 'latest' tag is not allowed"
  pattern:
    spec:
      containers:
      - image: "!*:latest"
```

**Require labels:**
```yaml
validate:
  message: "Label 'app.kubernetes.io/name' required"
  pattern:
    metadata:
      labels:
        app.kubernetes.io/name: "?*"
```

**Restrict registries:**
```yaml
validate:
  message: "Images must come from approved registries"
  pattern:
    spec:
      containers:
      - image: "myregistry.com/*"
```

# Mutations Are Powerful Too

Policies can also modify resources automatically:

```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: add-default-labels
spec:
  rules:
  - name: add-team-label
    match:
      resources:
        kinds:
        - Pod
    mutate:
      patchStrategicMerge:
        metadata:
          labels:
            managed-by: kubernetes
```

Now every pod gets the label automatically. Great for standardization without bothering developers.

# Enforcement Modes

Don't go to full enforcement immediately:

1. **Audit mode:** Log violations but don't block
2. **Warn mode:** Show warnings but allow
3. **Enforce mode:** Block violations

Start in audit, fix existing violations, then move to enforce.

# The Bigger Picture

Admission controllers are your last line of defense before bad configs hit the cluster. Combined with:
- RBAC (who can do what)
- Network policies (what can talk to what)
- Pod security standards (how pods can run)

You get defense in depth. Any one layer might have gaps, but together they make misconfiguration much harder.

**Sources:**
- [OPA for Kubernetes - Open Policy Agent](https://www.openpolicyagent.org/docs/kubernetes)
- [Kyverno Documentation](https://kyverno.io/)
- [Admission Controllers Comparison - DEV](https://dev.to/hkhelil/admission-controllers-in-kubernetes-opa-gatekeeper-kyverno-and-azure-policy-add-on-for-aks-which-one-wins-237d)
- [Kubernetes Policy Enforcement - StackGenie](https://www.stackgenie.io/analysing-kubernetes-policy-enforcement-using-opa-gatekeeper-and-kyverno/)

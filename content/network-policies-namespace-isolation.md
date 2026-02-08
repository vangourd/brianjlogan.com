+++
title = "Your Kubernetes Namespaces Aren't Isolated (Fix That)"
template = "page.html"
date = 2023-06-14T09:22:00Z
[taxonomies]
dates = ["2023-06"]
tags = ["kubernetes", "security", "networking"]
[extra]
summary = "Why network policies are the thing standing between you and lateral movement nightmares"
+++

I learned this one the hard way. Spent weeks setting up nice clean namespaces for different teams - `team-alpha`, `team-beta`, `production`, `staging`. Felt good about the organization. Then a security audit happened and someone pointed out that any pod in any namespace could talk to any other pod in any other namespace.

Wait, what?

# The Uncomfortable Truth

Kubernetes namespaces are an organizational tool. That's it. They're not a security boundary. By default, every single pod in your cluster can reach every other pod. Your staging database can talk to your production payment service. That random test workload someone deployed last month? It can probe your entire infrastructure.

I always assumed there was *some* isolation happening. Nope. It's a [flat network](https://kubernetes.io/docs/concepts/services-networking/network-policies/). The Kubernetes docs don't exactly advertise this fact prominently.

# Why This Matters

Picture this: an attacker exploits a vulnerability in one of your less-maintained services. Maybe it's that internal tool nobody's updated in 18 months. Once they're in that pod, they can scan your entire cluster. Every service, every database, every secret-holding workload is reachable.

Lateral movement becomes trivial. Your carefully segmented microservices architecture provides zero defensive value because the network doesn't enforce any boundaries.

As [Snyk's network policy guide](https://snyk.io/blog/kubernetes-network-policy-best-practices/) puts it: "Start with a deny-all policy and gradually allow only the necessary traffic as you identify it."

# Network Policies to the Rescue

NetworkPolicy resources let you define firewall rules at the pod level. They're enforced by your CNI plugin (Calico, Cilium, etc.).

Here's the simplest policy that makes a huge difference - deny all ingress by default:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-ingress
  namespace: production
spec:
  podSelector: {}
  policyTypes:
  - Ingress
```

Apply that and suddenly nothing can reach your production pods unless you explicitly allow it. The security model flips from "allow everything" to "deny by default."

# A Real Example

Say your frontend needs to talk to your API, and that's it:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-from-frontend-only
  namespace: production
spec:
  podSelector:
    matchLabels:
      app: api-server
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

Now only pods labeled `app: frontend` can reach your API on port 8080. That compromised test pod? Can't touch your API anymore.

# Cross-Namespace Isolation

This one actually provides namespace isolation:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: deny-from-other-namespaces
  namespace: production
spec:
  podSelector: {}
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: production
```

Traffic only allowed from within the same namespace. Now your namespaces actually mean something from a security perspective.

# Don't Forget Egress

Everyone focuses on ingress but egress matters too. If an attacker compromises a pod, they'll try to exfiltrate data or set up command-and-control channels.

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-egress
  namespace: production
spec:
  podSelector:
    matchLabels:
      app: api-server
  policyTypes:
  - Egress
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: database
    ports:
    - protocol: TCP
      port: 5432
  - to:
    - namespaceSelector:
        matchLabels:
          name: kube-system
      podSelector:
        matchLabels:
          k8s-app: kube-dns
    ports:
    - protocol: UDP
      port: 53
```

API can only talk to the database and DNS. Can't phone home to attacker infrastructure.

# The Gotchas

A few things that bit me:

**DNS is easy to forget.** Your pods need to resolve service names. If you lock down egress without allowing DNS to kube-dns, everything breaks in confusing ways. Spent two hours on that one.

**Not all CNIs support this.** Flannel doesn't implement NetworkPolicy. If you're using Flannel and apply these policies, nothing happens. [Calico, Cilium, and Antrea](https://www.armosec.io/blog/kubernetes-network-policies-best-practices/) are solid choices that do support them.

**Test before production.** I applied an overly aggressive policy once and took down service communication for an hour.

# Getting Started

If you want practical examples, [Ahmet Alp Balkan's kubernetes-network-policy-recipes](https://github.com/ahmetb/kubernetes-network-policy-recipes) on GitHub is gold. Just copy-paste and adapt.

Start with observability. If you're using Cilium, Hubble shows you actual traffic flows. See what's talking to what before you start blocking things.

Then apply default-deny policies to your most sensitive namespaces. Add allow rules based on actual requirements. Tighten over time.

It's not glamorous work but it transforms your cluster from an open network into something defensible.

**Sources:**
- [Kubernetes Network Policies Documentation](https://kubernetes.io/docs/concepts/services-networking/network-policies/)
- [kubernetes-network-policy-recipes - GitHub](https://github.com/ahmetb/kubernetes-network-policy-recipes)
- [Kubernetes Network Policy Best Practices - Snyk](https://snyk.io/blog/kubernetes-network-policy-best-practices/)
- [Kubernetes Network Policies Best Practices - ARMO](https://www.armosec.io/blog/kubernetes-network-policies-best-practices/)

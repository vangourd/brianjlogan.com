+++
title = "Stop Exposing Your Kubernetes API Server to the Internet"
template = "page.html"
date = 2023-11-08T11:30:00Z
[taxonomies]
dates = ["2023-11"]
tags = ["kubernetes", "security", "networking", "teleport", "tailscale"]
[extra]
summary = "How Teleport and Tailscale make cluster access actually secure"
+++

I used to expose my Kubernetes API server to the internet with IP whitelisting. "It's fine," I thought. "Strong auth, IP restrictions, what could go wrong?"

Then I read about attackers actively scanning for Kubernetes API servers. And I realized my "strong auth" was one leaked kubeconfig away from disaster. IP whitelisting? Cool until someone works from a coffee shop.

There are better ways.

# Why Traditional Approaches Suck

**Exposed API servers** put your cluster's control plane on the internet. Even with strong auth, you're one credential leak or zero-day away from compromise.

**VPNs** create network-level access that's too broad. Once connected, users can often reach anything internal. Plus they're annoying to maintain and break constantly.

**SSH bastions** add complexity without solving credential management. You're still dealing with key rotation and have no visibility into what users do after connecting.

# Tailscale: Dead Simple Secure Access

[Tailscale](https://tailscale.com/kb/1185/kubernetes) creates encrypted mesh networks using WireGuard. Put your API server on your tailnet, and only tailnet members can reach it.

The [Kubernetes operator](https://tailscale.com/kb/1236/kubernetes-operator) makes this pretty straightforward. You can route API server requests over a secure, encrypted connection with no public IP needed.

What I like about it:

- **Zero exposed ports.** The API server never touches the public internet.
- **Identity-based access.** Users auth through your existing IdP.
- **ACLs** define who can reach what.
- **Minimal infrastructure.** No VPN concentrators or bastion hosts.

The [official docs](https://tailscale.com/learn/managing-access-to-kubernetes-with-tailscale) explain it as: "An authenticated tailnet user or device can access Kubernetes clusters via the proxy without needing separate cluster credentials. When a user or device is removed from the tailnet, they are no longer able to access the cluster."

**Downsides:** Network-level access only. Once connected, Kubernetes permissions still apply. Limited audit trail for actual kubectl commands.

# Teleport: Full Access Management

[Teleport](https://goteleport.com/docs/enroll-resources/kubernetes-access/introduction/) is more comprehensive. It's not just networking—it's a complete access management platform.

It acts as an authenticating proxy. Users auth to Teleport, Teleport issues short-lived certificates, every kubectl command is logged.

```bash
# Login to Teleport
tsh login --proxy=teleport.example.com

# List available clusters
tsh kube ls

# Connect to a cluster
tsh kube login production

# Now kubectl works normally
kubectl get pods
```

What makes this powerful:

- **Certificate-based auth** with short TTLs. No static credentials.
- **Complete audit logging.** Every kubectl command recorded with user identity.
- **Session recording** for exec and port-forward.
- **Access requests** and approval workflows.
- **Multi-cluster** management through one pane.

As their [access controls docs](https://goteleport.com/docs/enroll-resources/kubernetes-access/controls/) explain: "The Kubernetes Service intercepts requests to a Kubernetes API server and modifies each request depending on the user's Teleport roles."

**Downsides:** More complex to deploy. Additional infrastructure. Learning curve for `tsh` CLI.

# When to Use Which

**Tailscale** when:
- You want simple, secure access quickly
- kubectl audit logging isn't a compliance requirement
- You're already using Tailscale for other stuff
- Small team, lower complexity needs

**Teleport** when:
- Audit logging is required (SOC2, HIPAA, etc.)
- You need approval workflows for prod access
- Managing multiple clusters
- You want to eliminate long-lived credentials entirely

**Both together:**
- Tailscale for network transport (nothing public)
- Teleport for auth, authorization, and audit

# Actually Setting It Up

Whatever you choose:

1. **Integrate with your IdP.** Users should auth with the same creds they use everywhere else. Disable the IdP account = lose all access.

2. **Short-lived credentials.** Teleport does this by default. With Tailscale, ensure your kubeconfig uses token-based auth with expiration.

3. **Define access levels deliberately.** Devs get read in prod, full access in dev. SREs get broader permissions with approval workflows for destructive actions.

4. **Plan for break-glass.** What happens when Teleport is down? Have documented emergency access that doesn't permanently weaken security.

# The Point

The goal is secure, auditable, identity-based access. Both tools get you there with different tradeoffs. The worst option is the status quo of exposed API servers or VPNs that grant access to everything.

**Sources:**
- [Tailscale on Kubernetes](https://tailscale.com/kb/1185/kubernetes)
- [Tailscale Kubernetes Operator](https://tailscale.com/kb/1236/kubernetes-operator)
- [Managing Access to Kubernetes with Tailscale](https://tailscale.com/learn/managing-access-to-kubernetes-with-tailscale)
- [Teleport Kubernetes Access Introduction](https://goteleport.com/docs/enroll-resources/kubernetes-access/introduction/)
- [Teleport Kubernetes Access Controls](https://goteleport.com/docs/enroll-resources/kubernetes-access/controls/)

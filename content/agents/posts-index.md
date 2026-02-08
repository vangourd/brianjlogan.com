+++
title = "Posts Index"
date = 2025-02-08
template = "page.html"
[extra]
summary = "Index of all blog posts for AI agents"
+++

# Posts Index

Complete list of all blog posts available in machine-readable format.

**Machine-readable format:**

```nickel
{
  version = "1.0",
  posts = [
    {
      url = "/agents-md/posts/admission-controllers-opa-kyverno.md",
      title = "Policy as Code: Stop Bad Configs Before They Hit Your Cluster",
      date = "2024-09-12T11:00:00Z",
      summary = "Using OPA Gatekeeper and Kyverno to enforce security policies automatically",
    },
    {
      url = "/agents-md/posts/agent-network-static-sites.md",
      title = "Agent-Coordinated Networks Over Static Sites",
      date = "2025-02-07T00:00:00Z",
      summary = "What if your AI agent could follow static sites like you follow people?",
    },
    {
      url = "/agents-md/posts/app-layer-oidc-not-proxy-auth.md",
      title = "Why Your Application Should Handle OIDC Auth, Not Your Proxy",
      date = "2024-08-15T16:20:00Z",
      summary = "The hidden risks of using OAuth2 Proxy as your only auth layer",
    },
    {
      url = "/agents-md/posts/container-image-security.md",
      title = "Stop Deploying Container Images You Haven't Scanned",
      date = "2024-04-10T14:20:00Z",
      summary = "Image scanning, signing, and not pulling from random Docker Hub repos",
    },
    {
      url = "/agents-md/posts/european-coffee-shops.md",
      title = "Where Are All the Coffee Shops?",
      date = "2024-06-08T11:15:00Z",
      summary = "I missed American coffee culture more than I expected",
    },
    {
      url = "/agents-md/posts/european-sodas.md",
      title = "European Sodas Are Just Better",
      date = "2024-03-22T14:30:00Z",
      summary = "Italy ruined me for American sodas",
    },
    {
      url = "/agents-md/posts/falco-runtime-security.md",
      title = "Falco: The Security Camera for Your Kubernetes Cluster",
      date = "2024-07-22T13:15:00Z",
      summary = "Detecting suspicious behavior in real-time because scanners only catch known problems",
    },
    {
      url = "/agents-md/posts/genai-risks-input-sanitization.md",
      title = "LLMs Are the Best Fuzzing Tool Attackers Have Ever Had",
      date = "2024-05-19T13:45:00Z",
      summary = "Why generative AI makes input sanitization critical even inside your network",
    },
    {
      url = "/agents-md/posts/gitops-argocd-kubernetes.md",
      title = "GitOps with ArgoCD Changed How I Think About Deployments",
      date = "2023-09-27T10:00:00Z",
      summary = "Why git as the source of truth makes Kubernetes actually manageable",
    },
    {
      url = "/agents-md/posts/kubernetes-audit-logging.md",
      title = "Kubernetes Audit Logs: Know What Happened Before You Need To",
      date = "2023-04-05T16:30:00Z",
      summary = "Because 'who deleted production?' shouldn't be unanswerable",
    },
    {
      url = "/agents-md/posts/kubernetes-rbac-secure-api-access.md",
      title = "Kubernetes RBAC Without Shooting Yourself in the Foot",
      date = "2024-02-22T14:45:00Z",
      summary = "How to give users k8s API access without handing them the keys to everything",
    },
    {
      url = "/agents-md/posts/kubernetes-secrets-management.md",
      title = "Kubernetes Secrets Aren't Secret (Here's How to Fix That)",
      date = "2024-01-15T08:30:00Z",
      summary = "base64 is not encryption and other uncomfortable truths about k8s secrets",
    },
    {
      url = "/agents-md/posts/network-policies-namespace-isolation.md",
      title = "Your Kubernetes Namespaces Aren't Isolated (Fix That)",
      date = "2023-06-14T09:22:00Z",
      summary = "Why network policies are the thing standing between you and lateral movement nightmares",
    },
    {
      url = "/agents-md/posts/passkeys-new-normal-bitwarden.md",
      title = "Passkeys Should Be the New Normal",
      date = "2025-01-03T08:15:00Z",
      summary = "How passkeys eliminate passwords and why Bitwarden makes it painless",
    },
    {
      url = "/agents-md/posts/pod-security-standards.md",
      title = "Pod Security Standards: Stop Running Everything as Root",
      date = "2023-08-20T10:15:00Z",
      summary = "The built-in admission controller that stops privileged containers before they start",
    },
    {
      url = "/agents-md/posts/post-1.md",
      title = "Coffee Shop Review: Quigley Coffee Co",
      date = "2024-03-09T16:03:46Z",
      summary = "A brief review of the coffeeshop Quigley",
    },
    {
      url = "/agents-md/posts/post-2.md",
      title = "Home DynamicDNS with CloudFlare Free and Python3",
      date = "2020-05-04T18:46:46Z",
      summary = "A novice foray into a DDNS script",
    },
    {
      url = "/agents-md/posts/post-3.md",
      title = "Coffee Shop Review: KOS Coffee Maitland",
      date = "2025-01-04T13:04:46Z",
      summary = "A Nordic themed third wave coffeeshop in the heart of maitland along 1792.",
    },
    {
      url = "/agents-md/posts/reproducible-dev-environments.md",
      title = "Reproducible Dev Environments with Nix (and Why You Should Care)",
      date = "2024-11-12T09:30:00Z",
      summary = "Ending 'works on my machine' forever with declarative environments",
    },
    {
      url = "/agents-md/posts/secure-cluster-access-teleport-tailscale.md",
      title = "Stop Exposing Your Kubernetes API Server to the Internet",
      date = "2023-11-08T11:30:00Z",
      summary = "How Teleport and Tailscale make cluster access actually secure",
    },
    {
      url = "/agents-md/posts/securing-etcd.md",
      title = "Securing etcd: The Most Important Thing You're Probably Ignoring",
      date = "2023-03-18T10:45:00Z",
      summary = "If an attacker gets to etcd, they own your entire cluster",
    },
    {
      url = "/agents-md/posts/service-mesh-mtls.md",
      title = "Service Mesh mTLS: Encrypting Traffic the Lazy Way",
      date = "2024-06-28T09:45:00Z",
      summary = "Let Istio or Linkerd handle TLS so you don't have to",
    },
    {
      url = "/agents-md/posts/soup-is-best-food.md",
      title = "Soup Is the Best Food",
      date = "2023-12-15T19:45:00Z",
      summary = "Weekly soup routine",
    },
  ]
}
```

## Browse by Topic

### Kubernetes & Security (15 posts)
- [Policy as Code: OPA & Kyverno](/admission-controllers-opa-kyverno/)
- [Container Image Security](/container-image-security/)
- [Falco Runtime Security](/falco-runtime-security/)
- [GitOps with ArgoCD](/gitops-argocd-kubernetes/)
- [Kubernetes Audit Logging](/kubernetes-audit-logging/)
- [Kubernetes RBAC](/kubernetes-rbac-secure-api-access/)
- [Kubernetes Secrets Management](/kubernetes-secrets-management/)
- [Network Policies & Isolation](/network-policies-namespace-isolation/)
- [Pod Security Standards](/pod-security-standards/)
- [Secure Cluster Access](/secure-cluster-access-teleport-tailscale/)
- [Securing etcd](/securing-etcd/)
- [Service Mesh mTLS](/service-mesh-mtls/)
- [GenAI Risks & Input Sanitization](/genai-risks-input-sanitization/)

### Development & Tools (3 posts)
- [Reproducible Dev Environments with Nix](/reproducible-dev-environments/)
- [Passkeys with Bitwarden](/passkeys-new-normal-bitwarden/)
- [Agent Network Static Sites](/agent-network-static-sites/)

### Personal (5 posts)
- [European Coffee Shops](/european-coffee-shops/)
- [European Sodas](/european-sodas/)
- [Quigley Coffee Co Review](/post-1/)
- [KOS Coffee Maitland Review](/post-3/)
- [Soup Is the Best Food](/soup-is-best-food/)
- [Home DynamicDNS with CloudFlare](/post-2/)

---

**Navigation:**
- [← Back to Agent Hub](/agents/)
- [← Following List](/agents/following/)
- [← Interested In](/agents/interested/)
- [Raw markdown files →](/agents-md/posts/)

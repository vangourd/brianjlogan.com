+++
title = "Stop Deploying Container Images You Haven't Scanned"
template = "page.html"
date = 2024-04-10T14:20:00Z
[taxonomies]
dates = ["2024-04"]
tags = ["kubernetes", "security", "containers", "supply-chain"]
[extra]
summary = "Image scanning, signing, and not pulling from random Docker Hub repos"
+++

Found a great image on Docker Hub. Pulled it. Deployed it. Three weeks later, security finds a cryptominer running in production.

We've all been there. Or we know someone who has. Container images are the new attack vector, and most teams don't treat them with the paranoia they deserve.

# The Supply Chain Problem

You're not just deploying your code. You're deploying:
- A base image (who maintains that?)
- System packages (when were they updated?)
- Dependencies (npm, pip, cargo... how many CVEs are hiding in there?)
- Build artifacts (was the CI compromised?)

Any link in that chain can be compromised. And as the [OWASP Kubernetes Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Kubernetes_Security_Cheat_Sheet.html) notes, securing your container images during build is critical.

# Scan Everything

Image scanning tools analyze layers for known vulnerabilities. The good news: there are solid open-source options.

**Trivy** is my go-to. Fast, comprehensive, easy to integrate:

```bash
# Scan an image
trivy image myapp:latest

# Scan and fail on high/critical
trivy image --severity HIGH,CRITICAL --exit-code 1 myapp:latest
```

Other solid options: [Clair](https://github.com/quay/clair), [Grype](https://github.com/anchore/grype), [Snyk Container](https://snyk.io/product/container-vulnerability-management/).

As [Sysdig's best practices](https://www.sysdig.com/learn-cloud-native/12-container-image-scanning-best-practices) recommends: "Healthy security practices include regular scans of container images before deployment."

# When to Scan

1. **At build time** - Catch issues before they hit your registry
2. **In your registry** - Many registries (Harbor, ECR, GCR) scan automatically
3. **At deploy time** - Admission controllers can block vulnerable images
4. **Continuously** - New CVEs are discovered daily; rescan regularly

# Sign Your Images

Scanning tells you about known vulnerabilities. Signing tells you the image hasn't been tampered with.

[Cosign](https://docs.sigstore.dev/cosign/overview/) (part of Sigstore) makes this straightforward:

```bash
# Sign an image
cosign sign myregistry.com/myapp:v1.0.0

# Verify before deploying
cosign verify myregistry.com/myapp:v1.0.0
```

Combined with admission controllers, you can enforce that only signed images run in your cluster.

# Use Minimal Base Images

Every package in your image is potential attack surface. The [distroless images](https://github.com/GoogleContainerTools/distroless) concept is powerful: no shell, no package manager, just your app and its dependencies.

```dockerfile
# Instead of this
FROM ubuntu:22.04

# Try this
FROM gcr.io/distroless/static-debian12
```

As the [ARMO guide](https://www.armosec.io/glossary/container-image-scanning/) notes: "Distroless images allow you to package only your application and its dependencies in a lightweight container image, which minimizes the attack surface."

Can't go full distroless? At least use slim variants and remove unnecessary packages.

# Pin Your Images

```yaml
# Bad - this changes constantly
image: nginx:latest

# Better - specific version
image: nginx:1.25.3

# Best - immutable digest
image: nginx@sha256:abc123...
```

Using `latest` or mutable tags means the image you scanned might not be the one you deploy. Digests guarantee immutability.

# Private Registries

Stop pulling directly from Docker Hub in production. Set up a private registry and only push vetted images:

- All images scanned before entering
- Signature verification
- Access controls
- Audit logging

[Harbor](https://goharbor.io/) is a solid open-source choice. All major cloud providers offer managed registries too.

# Admission Control

Block bad images before they run. Kubernetes admission controllers can:

- Require images from trusted registries only
- Block images with critical vulnerabilities
- Enforce image signatures
- Prevent `latest` tags

Tools like [Kyverno](https://kyverno.io/) or [OPA Gatekeeper](https://open-policy-agent.github.io/gatekeeper/) make this policy-driven:

```yaml
apiVersion: kyverno.io/v1
kind: ClusterPolicy
metadata:
  name: require-signed-images
spec:
  validationFailureAction: enforce
  rules:
  - name: check-signature
    match:
      resources:
        kinds:
        - Pod
    verifyImages:
    - image: "myregistry.com/*"
      key: |-
        -----BEGIN PUBLIC KEY-----
        ...
        -----END PUBLIC KEY-----
```

# Don't Run as Root

Even with a good image, running as root is asking for trouble:

```dockerfile
# Create non-root user
RUN adduser --disabled-password --gecos "" appuser
USER appuser
```

And in your Kubernetes manifests:

```yaml
securityContext:
  runAsNonRoot: true
  runAsUser: 1000
```

# The Pipeline

Put it together:

1. Build with minimal base image
2. Scan at build time, fail on critical vulns
3. Sign the image
4. Push to private registry (which scans again)
5. Admission controller verifies signature and scan status
6. Deploy

It sounds like a lot, but most of this can be automated in CI. The alternative—discovering a compromised image in production—is much more expensive.

**Sources:**
- [OWASP Kubernetes Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Kubernetes_Security_Cheat_Sheet.html)
- [12 Container Image Scanning Best Practices - Sysdig](https://www.sysdig.com/learn-cloud-native/12-container-image-scanning-best-practices)
- [Container Image Scanning - ARMO](https://www.armosec.io/glossary/container-image-scanning/)
- [Cosign - Sigstore](https://docs.sigstore.dev/cosign/overview/)

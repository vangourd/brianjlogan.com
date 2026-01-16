+++
title = "Securing etcd: The Most Important Thing You're Probably Ignoring"
template = "page.html"
date = 2023-03-18T10:45:00Z
[taxonomies]
tags = ["kubernetes", "security", "etcd"]
[extra]
summary = "If an attacker gets to etcd, they own your entire cluster"
+++

etcd stores everything. Every secret, every RBAC policy, every deployment, every config. It's the single source of truth for your entire Kubernetes cluster.

If someone gets read access to etcd, they have all your secrets. If they get write access, they control your cluster. Yet I've seen etcd running without encryption, without TLS, sometimes even exposed to the network.

# Why This Matters

From [k8s-security.geek-kb.com](https://k8s-security.geek-kb.com/docs/best_practices/cluster_setup_and_hardening/control_plane_security/etcd_security_mitigation/): "etcd is the backbone of a Kubernetes cluster, storing all cluster configuration data, including secrets, RBAC policies, and workload definitions. If etcd is left unsecured, attackers can extract sensitive data or modify cluster settings to gain full control over Kubernetes."

Full control. Not "access to some pods." Full control.

# Encryption at Rest

By default, etcd stores data unencrypted. Your Secrets are sitting in plaintext on disk. Anyone with disk access (backup systems, compromised nodes, stolen drives) can read them.

[Kubernetes supports encryption at rest](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/) via encryption providers configured on the API server:

```yaml
apiVersion: apiserver.config.k8s.io/v1
kind: EncryptionConfiguration
resources:
  - resources:
      - secrets
    providers:
      - aescbc:
          keys:
            - name: key1
              secret: <base64-encoded-key>
      - identity: {}
```

Then configure the API server:
```
--encryption-provider-config=/path/to/encryption-config.yaml
```

For managed clusters (EKS, GKE, AKS), encryption at rest is usually available—sometimes on by default, sometimes you need to enable it.

As [this guide](https://medium.com/@tamerbenhassan/keeping-your-data-safe-etcd-encryption-at-rest-in-kubernetes-a297250e83e7) notes: "Encryption at rest is crucial for: Security (protects sensitive data from unauthorized users), Compliance (helps meet GDPR, HIPAA, PCI-DSS requirements), and Peace of Mind (even if storage media is compromised, data remains secure)."

# Use a KMS Provider

Local key storage protects against etcd compromise but not host compromise—the keys are on the same machine.

Better approach: use envelope encryption with a cloud KMS:

- **AWS:** KMS integration with EKS
- **GCP:** Cloud KMS with GKE
- **Azure:** Key Vault KMS with AKS

From [Google's docs](https://cloud.google.com/kubernetes-engine/docs/how-to/encrypting-secrets): "Application-layer secrets encryption provides an additional layer of security for sensitive data. You use a key managed with Cloud KMS to encrypt data at rest at the application layer."

The encryption key (DEK) is encrypted by your KMS key (KEK). Compromise of etcd alone doesn't give access to the data.

# TLS for etcd Communication

etcd communication should be encrypted. Client-to-server and server-to-server (peer) traffic both need TLS.

From the [CIS Kubernetes Benchmark](https://www.cisecurity.org/benchmark/kubernetes): "Use TLS-based authentication to allow (or not) access to etcd."

For self-managed clusters, configure etcd with:
```
--cert-file=/path/to/server.crt
--key-file=/path/to/server.key
--client-cert-auth=true
--trusted-ca-file=/path/to/ca.crt
--peer-cert-file=/path/to/peer.crt
--peer-key-file=/path/to/peer.key
--peer-client-cert-auth=true
--peer-trusted-ca-file=/path/to/ca.crt
```

Managed clusters handle this for you, but verify it's enabled.

# Network Isolation

etcd should only be reachable from the control plane. Period.

From [k8s-security.geek-kb.com](https://k8s-security.geek-kb.com/docs/best_practices/cluster_setup_and_hardening/control_plane_security/etcd_security_mitigation/): "If etcd is exposed externally, it becomes a high-value target. Use firewall rules to block external access."

In practice:
- Firewall rules restricting access to control plane nodes only
- No public IP on etcd nodes
- Network policies (if etcd runs in the cluster) restricting ingress

# Backup Security

Your etcd backups contain all your secrets. If backups are unencrypted or stored insecurely, you've undermined your encryption at rest.

Encrypt backups. Store them with proper access controls. Test restoration regularly.

# Key Rotation

Encryption keys should rotate. For KMS-backed encryption, this is often automatic. For local keys, you need a rotation process.

[Monitor certificate expiration](https://medium.com/@tamerbenhassan/keeping-your-data-safe-etcd-encryption-at-rest-in-kubernetes-a297250e83e7). Expired certificates break cluster communication and cause downtime. Set up alerts before expiration.

# Managed Clusters

If you're on EKS, GKE, or AKS, you don't manage etcd directly. But you should still:

1. **Enable application-layer encryption** if not default
2. **Use customer-managed keys** for compliance requirements
3. **Verify audit logging** captures etcd access
4. **Understand the shared responsibility model**

The cloud provider secures the infrastructure; you secure the configuration.

# Self-Managed Clusters

More work, but you have full control:

1. Enable encryption at rest with KMS if possible
2. Configure TLS for all etcd communication
3. Restrict network access to control plane only
4. Encrypt and secure backups
5. Monitor certificate expiration
6. Set up audit logging

The [kubeadm documentation](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/) covers most of this for self-managed setups.

# The Bottom Line

etcd compromise = cluster compromise. It's that simple.

If you do nothing else:
1. Enable encryption at rest
2. Ensure TLS is configured
3. Verify network isolation

The time to secure etcd is before you have an incident, not after.

**Sources:**
- [Encrypting Confidential Data at Rest - Kubernetes](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/)
- [Securing etcd - k8s-security.geek-kb.com](https://k8s-security.geek-kb.com/docs/best_practices/cluster_setup_and_hardening/control_plane_security/etcd_security_mitigation/)
- [etcd Encryption at Rest - Medium](https://medium.com/@tamerbenhassan/keeping-your-data-safe-etcd-encryption-at-rest-in-kubernetes-a297250e83e7)
- [Encrypt secrets at the application layer - Google Cloud](https://cloud.google.com/kubernetes-engine/docs/how-to/encrypting-secrets)
- [Encryption and etcd - Infosec Institute](https://resources.infosecinstitute.com/topic/encryption-and-etcd-the-key-to-securing-kubernetes/)

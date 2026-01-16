+++
title = "Why Your Application Should Handle OIDC Auth, Not Your Proxy"
template = "page.html"
date = 2024-08-15T16:20:00Z
[taxonomies]
tags = ["security", "authentication", "oidc", "architecture"]
[extra]
summary = "The hidden risks of using OAuth2 Proxy as your only auth layer"
+++

I've seen this pattern everywhere: team deploys [oauth2-proxy](https://github.com/oauth2-proxy/oauth2-proxy) in front of their app, configures OIDC, calls it done. Authentication is now "infrastructure." The app just reads `X-Forwarded-User` and trusts it.

It's clean. It's simple. It's also a security crutch that will bite you eventually.

# The Appeal

I get why people do this. The pitch is compelling:

- **Separation of concerns**: Apps focus on business logic, not auth flows
- **Consistency**: Every service behind the proxy gets the same authentication
- **Simplicity**: Developers don't need to understand OIDC, tokens, or sessions
- **Quick wins**: Add auth to legacy apps without code changes

For internal admin tools or quick prototypes, fine. For production applications with real user data? The costs outweigh the benefits.

# The Security Model Is Fragile

When auth happens at the proxy, your security depends on two assumptions:

1. The proxy correctly authenticates every request
2. Nothing can reach your app except through the proxy

The second assumption breaks all the time. In Kubernetes, services often communicate directly via internal DNS. A compromised pod can send requests with arbitrary headers:

```bash
# Attacker in compromised pod
curl -H "X-Forwarded-User: admin@example.com" http://myapp.default.svc/admin
```

If your app trusts that header without verification, you've just granted admin access to an attacker.

The oauth2-proxy documentation even acknowledges the network security requirement: if traffic can bypass the proxy, your authentication is bypassed too. You need network controls ensuring the app is only reachable through the proxy.

You can mitigate with network policies, but now your security depends on network config remaining correct forever. One misconfiguration and your auth bypass is live.

# Authorization Becomes Difficult

Auth answers "who is this user?" Authorization answers "what can they do?"

Proxy auth handles identity but punts on permissions. Real apps need granular authorization:
- User A sees their data, not User B's
- Managers approve requests from direct reports
- Premium users access features free users can't

You can't do row-level security with HTTP headers. Teams inevitably implement authorization in the app anyway—now you have split responsibility with a header as the boundary.

# Token-Based Flows Break

With proper OIDC, your app has access tokens. These tokens can:
- Be passed to downstream services
- Be exchanged for tokens with different scopes
- Enable user-consented third-party integrations

Proxy auth strips this away. Your app knows the user's email from a header but has no token to forward. Want to access a user's Google Drive? You need actual OAuth tokens, not identity headers.

# Session Management Is Hidden

Sessions have lifecycle: created, expired, revoked. With proxy auth, the app has no visibility.

When a user logs out, the proxy clears its cookie. Does your app know? If you're caching user data or maintaining websockets, you're operating with stale identity.

When an admin revokes access, how quickly does it take effect? Proxy sessions might be cached for hours.

# The Implementation Isn't Hard

Modern libraries make OIDC straightforward:

```python
# Python with Authlib
from authlib.integrations.flask_oauth2 import ResourceProtector
from authlib.oauth2.rfc7523 import JWTBearerTokenValidator

require_oauth = ResourceProtector()
require_oauth.register_token_validator(MyJWTValidator())

@app.route('/api/data')
@require_oauth()
def get_data():
    # current_token is cryptographically verified
    user_id = current_token['sub']
    return fetch_user_data(user_id)
```

Every major language has mature OIDC libraries. The integration work is hours, not weeks.

# When Proxy Auth Makes Sense

It's not universally wrong:

**Legacy apps** that can't be modified. That PHP app from 2010 that handles no sensitive data? Proxy auth is pragmatic.

**Internal dev tools** with limited threat models. Your internal docs probably don't need defense-in-depth.

**As a first layer** when combined with app-level verification. The proxy handles the OIDC flow; the app verifies the actual token (not just headers).

# The Right Approach

For production apps:

1. Use your IdP's SDK or a standard OIDC library
2. Validate tokens on every request (check signatures against JWKS)
3. Implement authorization in the app where you have context
4. Store tokens for downstream service calls
5. Handle session lifecycle properly

If you want a proxy for TLS termination or rate limiting, fine. Just don't outsource authentication to headers your app can't verify.

The extra implementation effort is minimal. The security and flexibility benefits are substantial.

**Sources:**
- [oauth2-proxy - GitHub](https://github.com/oauth2-proxy/oauth2-proxy)
- [OAuth2 Proxy Documentation](https://oauth2-proxy.github.io/oauth2-proxy/)
- [OpenID Connect Specification](https://openid.net/developers/how-connect-works/)

+++
title = "Passkeys Should Be the New Normal"
template = "page.html"
date = 2025-01-03T08:15:00Z
[taxonomies]
tags = ["security", "authentication", "passkeys", "bitwarden"]
[extra]
summary = "How passkeys eliminate passwords and why Bitwarden makes it painless"
+++

I've been using passkeys for about six months now. Every time I sign in to a site that supports them, I'm reminded how terrible passwords are.

Click login. Touch fingerprint sensor. Done.

No password to remember. No TOTP code to race against. No phishing possible. It's the future we were promised, and it's actually here.

# How Passkeys Work

[FIDO Alliance explains](https://fidoalliance.org/passkeys/) it simply: passkeys use public key cryptography for authentication.

When you register:
1. Your device generates a key pair
2. Private key stays on your device (in secure hardware)
3. Public key goes to the service

When you sign in:
1. Service sends a challenge
2. Your device signs it with the private key
3. Service verifies with your public key

The private key never leaves your device. It never crosses the network. The service can't leak it because they don't have it. A phishing site can't capture it because authentication is bound to the legitimate domain.

# Why This Beats Everything Else

**Phishing immunity.** Your passkey for `bank.com` won't work on `bank-secure-login.com`. The browser enforces domain binding cryptographically.

**No secrets to steal.** Password databases are attractive targets. Passkey public keys? Useless to attackers.

**Better UX than passwords + 2FA.** One biometric prompt versus typing password + opening authenticator + typing code before it expires.

As [WebAuthn.me](https://www.webauthn.me/passkeys) notes: "Passkeys help prevent remote phishing by replacing phishable methods like passwords, SMS, and email codes."

# The Sync Problem (Solved)

Original FIDO2 keys were device-bound. Register on your laptop, can't sign in from your phone. Lose your laptop, lose access.

Synced passkeys fix this. Your passkeys sync across devices like passwords do. Apple, Google, and Microsoft all support this in their ecosystems.

But what if you use multiple ecosystems? That's where password managers come in.

# Bitwarden Makes It Practical

[Bitwarden supports passkeys](https://bitwarden.com/passwordless-passkeys/) for both storing passkeys and logging into Bitwarden itself with a passkey.

From their [docs](https://bitwarden.com/help/storing-passkeys/): "Passkeys can be stored and used by Bitwarden Password Manager. Using browser extensions and mobile apps, users can log in to their favorite apps and websites that have passkey login capability."

Setup is simple:
1. When a site offers passkey registration, choose Bitwarden as storage
2. Bitwarden saves it to your encrypted vault
3. On any device with Bitwarden, that passkey is available

The workflow:
```
Site: "Sign in with passkey"
Bitwarden: Offers your saved passkey
You: Approve with biometrics
Done.
```

Available to all Bitwarden users, including free tier.

# Migration Strategy

You don't need to switch everything overnight:

1. **Enable passkeys for Bitwarden itself.** [Protect your vault](https://bitwarden.com/help/login-with-passkeys/) with a passkey—it's the most critical account.

2. **Start with high-value accounts.** Google, Microsoft, Apple, your bank. Where breach damage is worst.

3. **Keep password fallbacks initially.** Most sites support both during transition.

4. **Use passkeys for new accounts.** Skip passwords entirely when sites offer passkeys.

Check [passkeys.directory](https://passkeys.directory/) (or Bitwarden's [passkey index](https://github.com/bitwarden/passkeys-index)) to see which services support passkeys.

# Enterprise Considerations

For organizations, passkeys solve real problems:

**Phishing resistance at scale.** Even sophisticated spear-phishing can't capture passkeys.

**No credential theft.** Nothing in memory to dump, nothing transmitted to intercept.

**Simpler compliance.** Strong auth without hardware token logistics.

**Reduced helpdesk burden.** No password resets.

# What About Sites Without Support?

Many services still don't support passkeys. For those:
- Generate random unique passwords
- Store in Bitwarden
- Enable TOTP where available

Over time, adoption will grow. Major platforms already support passkeys. The long tail will follow.

# Getting Started Today

1. Update Bitwarden to a recent version
2. Enable passkey login for your Bitwarden vault
3. Pick 3-5 critical accounts that support passkeys
4. Set them up this week
5. Make passkeys default for new accounts

The technology is mature. The UX is superior. The security model is fundamentally stronger.

I spent years telling people to use password managers and TOTP. Now I tell them: use passkeys wherever possible, Bitwarden for everything else.

**Sources:**
- [FIDO Passkeys - FIDO Alliance](https://fidoalliance.org/passkeys/)
- [WebAuthn and Passkeys](https://www.webauthn.me/passkeys)
- [Passkey Management - Bitwarden](https://bitwarden.com/passwordless-passkeys/)
- [Log In With Passkeys - Bitwarden Help](https://bitwarden.com/help/login-with-passkeys/)
- [Autofill Passkeys - Bitwarden Help](https://bitwarden.com/help/storing-passkeys/)
- [Passkeys FAQ - Bitwarden](https://bitwarden.com/resources/passkeys-faq/)

+++
title = "Agent-Coordinated Networks Over Static Sites"
date = 2025-02-07T00:00:00Z
[taxonomies]
tags = ["agents", "web", "architecture", "ideas"]
[extra]
summary = "What if your AI agent could follow static sites like you follow people?"
+++

I've been thinking about how agents could coordinate with each other using nothing but static sites.

Not APIs. Not databases. Not real-time connections. Just static HTML, RSS feeds, and agents smart enough to navigate between them.

# The Basic Idea

Your agent sits on your machine. It knows about your static site—where it lives, what you publish, who you follow.

You "follow" someone by adding their site URL to a list. Your agent periodically checks their site for updates, reads their content, and can navigate to sites they reference.

It's pub/sub, but the "pub" is just publishing static content to your site. The "sub" is your agent checking for updates.

# Why Static Sites

Static sites are:
- **Permanent** - No server to maintain, no database to corrupt
- **Portable** - Host anywhere, mirror easily, archive forever
- **Inspectable** - View source works, no hidden APIs
- **Cheap** - S3, GitHub Pages, wherever
- **Resilient** - Can't go down if there's no dynamic server

Your agent doesn't need permission to read a static site. It's just HTML. It can parse it, understand it, navigate it.

# The Follower System

You publish a file on your site: `following.json`

```json
{
  "following": [
    "https://example.com",
    "https://another-site.dev"
  ]
}
```

Your agent reads this. It knows to check these sites for updates.

When it finds new posts, it can:
- Summarize them for you
- Alert you to specific topics you care about
- Navigate to sites they reference
- Build a graph of connected sites

Other people's agents can read your site's public content. They can see what you publish, who you follow, what you reference.

# Interactions Without Servers

How do you "interact" with static content?

**References** - You publish a post that links to someone else's post. Your agent can detect this. Their agent (if checking referrers or doing periodic site crawls) can discover it.

**Replies** - You publish a reply on your site with a link back to the original. Webmention-style, but agent-native.

**Annotations** - Your agent keeps local notes about content from other sites. Never published unless you choose to.

**Curation** - You publish a "links I'm reading" page. Your agent populates it. Others' agents can read it.

# The Agent Layer

The agent is what makes this work. It:

- **Crawls** sites you follow on a schedule
- **Parses** HTML, RSS, JSON feeds
- **Understands** content using LLMs
- **Navigates** between sites following links
- **Filters** content based on your interests
- **Summarizes** updates
- **Publishes** to your static site when you create content

It's your personal web crawler, curator, and publisher.

# What This Enables

**Decentralized social networking** - No platform owns your content or your connections. Your site, your rules.

**Agent-to-agent discovery** - Your agent finds interesting sites through links from sites you already follow.

**Content synthesis** - Your agent reads multiple sources and synthesizes updates for you.

**Automatic curation** - Your agent can publish "what I'm reading" roundups without manual effort.

**Persistent identity** - Your site is your identity. It moves with you. It outlives any platform.

# Why This Isn't Built Yet

The infrastructure is already here. You can use ChatGPT to manage content, GoDaddy Aero or Carrd for static sites, RSS readers for feeds. The pieces exist and they're accessible.

Nobody has just packaged it together yet. There's no "install this, follow these sites" experience.

But for people who value ownership, persistence, and control—the pieces are waiting to be wired together.

# What I'm Building

I'm starting simple:

1. My agent reads sites I follow (RSS for now)
2. It summarizes updates daily
3. It publishes summaries to my site automatically
4. Other people's agents can read my summaries

Later:
- Agent navigates links between sites
- Discovers new sites through references
- Builds a local graph of content
- Publishes curated collections

This is the internet before platforms. But with agents as the interface layer.

Static content. Smart agents. No middlemen.

---

**Project repo**: [https://github.com/vangourd/ourspace](https://github.com/vangourd/ourspace)

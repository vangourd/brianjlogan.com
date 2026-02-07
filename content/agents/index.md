+++
title = "Agent Network Hub"
date = 2025-02-07
template = "page.html"
[extra]
summary = "My agent-coordinated network implementation and endpoints"
+++

# Agent Network Hub

This is my implementation of the agent-coordinated network concept. See the [main article](/agent-network-static-sites/) for the full concept.

## My Use Cases

### Decentralized Bookmarking
Agent-managed bookmarks that live on my site, not someone else's platform. My agent can:
- Collect links I save from anywhere
- Organize and tag them automatically
- Make them discoverable to other agents
- Archive the content for permanence

### Link Aggregation
Curated collections of links from sites I follow:
- Agent reads my follows daily
- Surfaces interesting links based on my interests
- Publishes aggregated collections to my site
- Others can subscribe to my curated feeds

### Long-Form Writing (Letters Revival)
Bringing back the lost art of letter-writing, but async and agent-assisted:
- Write longer-form responses to others' posts
- Publish on my site with proper threading
- Agent notifies when someone responds
- Build conversations across sites without platforms

### Private Circles
Public and private content mixing (like Google+ wanted):
- Public content: available to all agents
- Circle content: encrypted or gated by authentication
- Agent manages access control
- Maintain different audiences on the same site

## Standard Endpoints

### `/agents/following.yaml`
Sites my agent follows.

### `/agents/bookmarks/`
My agent-managed bookmark collection.

### `/agents/links/`
Aggregated links from sites I follow.

### `/agents/letters/`
Long-form responses and threaded conversations.

## Implementation Status

**Current**: Early concept, documenting use cases

**Next steps**:
1. Set up basic `following.yaml`
2. Implement bookmark collection
3. Build link aggregation
4. Create letter threading system

---

**Project documentation**: [https://github.com/vangourd/ourspace](https://github.com/vangourd/ourspace)

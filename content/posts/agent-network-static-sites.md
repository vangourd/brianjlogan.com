+++
title = "ANTS: Agent Network Traversal System"
date = 2025-02-07T00:00:00Z
template = "page.html"
[taxonomies]
dates = ["2025-02"]
tags = ["agents", "web", "architecture", "ideas", "ants"]
[extra]
summary = "What if your AI agent could follow static sites the way you follow people - no Facebook, no Google, no middlemen?"
+++

## The Basic Concept

**ANTS (Agent Network Traversal System)** is a simple idea: agents can do a lot of interaction on behalf of users by just publishing static content that can be scraped cheaply through CDNs and high-performance proxy hosts.

You publish a static site. I publish a static site. Our agents read each other's sites, summarize what's new, and surface what we care about. No platform sits in the middle.

## Why Static Content

Static files served from a CDN are about as cheap and fast as the web gets. There is no API to rate-limit, no login wall, no algorithmic feed deciding what gets shown. A crawler with a modest budget can keep up with thousands of personal sites without breaking a sweat.

That changes the economics of discovery. Building an index of "the people I follow and the people they follow" stops being an infrastructure project and starts being a weekend script.

## Building Your Own Index

The point of ANTS is that anyone can build a crawler and their own index to find things on the web without relying on incumbent providers like Facebook and Google.

A site that wants to participate publishes a small, predictable hub - a `/agents/` page that lists who they follow, what they post, and how an agent should interact with the site. From there, an agent can:

- Walk the follow graph the same way a human follows links.
- Pull raw markdown for cheap parsing instead of fighting rendered HTML.
- Cache aggressively at the edge, since the content rarely changes.
- Build a personal index scoped to whatever the user actually cares about.

No single index has to be authoritative. Everyone gets to build their own.

## Why This Matters

The current web routes most discovery through a handful of incumbents. Their incentives are not your incentives. They optimize for engagement, ad inventory, and lock-in.

Static sites plus cheap crawlers route around that. You get back something closer to the early web - sites linking to sites, with agents doing the tedious work of keeping up. The infrastructure is boring on purpose. Boring is what makes it durable.

## Privacy Through Keys, Not Walls

One tricky problem is privacy to specific audiences. It's worth being honest about the baseline: even in walled-garden ecosystems like Facebook, anything you post can be extracted by whoever is in the audience you set. The wall doesn't actually keep your content private - it just gates who shows up on the inside.

If that's the real bar, you can clear it without a platform. Encrypt each post for the public keys of the people you want to reach, and publish the ciphertext as static content like everything else. Anyone can fetch it; only the intended recipients can read it. The CDN doesn't need to know who you're sharing with, and you don't need a server enforcing access rules.

This needs a spec before it's useful - key discovery, rotation, group membership, how an agent figures out which blobs are addressed to its user. None of that is solved here. But it's a neat thing to experiment with and think about, and it slots cleanly into the ANTS model: one more file on a static site, just one that most readers can't decrypt.

## Status

This site is the experiment. The [/agents/](/agents/) hub lists what I follow and how an agent should read me. It's an early implementation - rough edges expected.

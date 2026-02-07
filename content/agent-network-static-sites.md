+++
title = "Agent-Coordinated Networks Over Static Sites"
date = 2025-02-07T00:00:00Z
template = "post-with-tabs.html"
[taxonomies]
tags = ["agents", "web", "architecture", "ideas", "ai-assisted"]
[extra]
summary = "What if your AI agent could follow static sites like you follow people?"
+++

<div class="authorship-notice">

**Authorship Transparency**: I want to be openly transparent about my own written words versus genai influenced content as I think drawing a line is important to establishing your own trust in an AI driven world. The rest of this blog post was generated through my own personal review, considerations, and multiple rounds of evolving and tweaking its content. That doesn't mean it's not prone to errors and I believe some aspect of "move quickly and break things" is warranted here. I have three tags for articles: `ai-assisted`, `ai-only`, `human-only` which will make it clear the authorship of the article.

</div>

<div class="tech-level-prompt">Who are you?</div>

<div class="tech-level-tabs">
  <button data-level="everyday">Everyday Joe</button>
  <button data-level="technical">Technical</button>
</div>

<div class="tech-level-content" data-level="everyday">

## The Big Idea

Use some simple techniques to publish and subscribe to other people's content online without any reliance on middlemen like Facebook or Instagram. Revive the web.

Here's the basic idea: Use AI to read other people's websites, helping you summarize and explore their content without having to read everyone verbatim. What does that mean? It means you can have AI read multiple websites at once.

You can publish certain kinds of information—looking for a tool to borrow, selling some furniture, stuff that reminds me of old message boards and forums. Real community stuff, not algorithm-driven feeds.

You own your website. Others own theirs. AI helps you keep up with what people are posting without drowning in content.

### How to Get Started

**Use a tool like bolt.new to create a simple website** - You can use bolt.new or similar tools to create a basic website. Your AI (ChatGPT, Claude, etc.) can help you do this. You don't need to be a programmer.

**Create `/agents/index.md` on YOUR site** - This is your hub. It has:
- Links to the people/sites you follow
- What you're sharing
- What AI should know about your site

Your AI can help you create this file in whatever format works for you.

**Important: Everything is public** - Don't put anything on your site you wouldn't put in a public Facebook post. This is all publicly readable. It's not private. Think of it like a public bulletin board, not a locked diary.

**Tell AI to read YOUR OWN site** - This is the key: You tell AI "Read MY site at https://mysite.com/agents/index.md and check what's new from the people I follow."

AI starts at YOUR index.md, sees who YOU follow, then reads those sites and summarizes what's new. Everything starts from your own site.

As the idea evolves, AI gets smarter about what to look for and how to organize it. But it always starts by reading your own index.md.

**Publish by writing** - Want to say something? Write it on your site. Looking for a tool to borrow? Post it. Selling furniture? Post it. Responding to someone else's post? Write on your site and link back to theirs.

No comment boxes. No platform. Just websites linking to each other like the old web.

### Why Use Simple Websites

**Easy to set up** - A static site is just files sitting on the internet. No WordPress. No complicated backend. You can use bolt.new, GitHub Pages, or lots of other tools. Many are free.

**It lasts** - Once you publish a file, it just sits there. No server to crash. No database to get hacked. No company going out of business and taking your data with them.

**You own it** - It's your website. Your files. You can move them anywhere. Copy them. Back them up. No one can ban you or change the rules on you.

**AI can read it** - Because it's just a public website, any AI tool with web access can read it. No special permissions needed. Works with ChatGPT, Claude, whatever.

**Cheap or free** - Many options are free or cost a few dollars a month. Way cheaper than paying a social media company with your data and attention.

### How People Connect

**It all starts with YOUR index.md** - You maintain a list of people you follow in YOUR `/agents/index.md`. When you tell AI "check what's new," it reads YOUR site first, sees who YOU follow, then goes and reads those sites.

**Following someone** - Add their website URL to your index.md:

```
# People I Follow
- https://brianjlogan.com (tech stuff, added 2025-02-07)
- https://example.com
```

Don't worry about the file format. Make it however you want. AI figures it out.

**Using it** - Tell your AI: "Read https://mysite.com/agents/index.md and summarize what's new from the people I follow."

AI reads YOUR index, finds the links, visits those sites, and tells you what's happening. As it learns, it gets better at knowing what you care about.

**Replying** - Write a post on your site that links back to theirs. Like writing a letter but posting it publicly. If they tell their AI to check for backlinks, it might discover your reply.

**Sharing lists** - AI can help you publish collections of interesting links. Put them on your site. Others can tell their AI to read your collections.

Put this in an `/agents/` folder on your site. That's the standard place AI looks.

### How AI Tools Help

Whether you're using ChatGPT manually or running something automatic, the AI handles the boring stuff:

**Reading your files** - AI reads your `following.yaml` file to know which sites to check. Just a simple text file with a list.

**Fetching updates** - Manual way: "ChatGPT, check these sites for updates." Automatic way: a program runs on a schedule. Either way, AI grabs the new content.

**Understanding what it means** - This is where AI shines. It reads posts and figures out what they're about, what's interesting to you.

**Showing you summaries** - Instead of visiting 50 websites yourself, AI gives you "Here's what's new and why it matters." You decide what to actually read.

**Helping you publish** - AI can help draft responses, create the files for your website, even rebuild your site. Or you write manually and AI just handles the tedious publishing steps.

**Mapping connections (optional)** - If you eventually build something that runs automatically, it can track how sites link to each other, map the network, discover new sites through these connections.

This works with both approaches: manually using ChatGPT/Claude, or building automated tools later. Start simple, expand when you're ready.

### Trust & Safety

**How do you know who's who?** - Same way you know a website is real: HTTPS certificates. When you visit `https://brianjlogan.com`, your browser checks that it's really my domain. AI does the same—no extra login needed.

**Trust levels based on how close you are**:

**Level 0 (you)**: Full control. It's your site.

**Level 1 (people you follow)**: High trust. AI reads their content, summarizes it, can help you interact with it. You explicitly chose to follow them.

**Level 2 (friend of friends)**: Discovery mode. AI can show you "Alice follows Bob" but won't act on Bob's content. Just suggestions. You have to approve to promote them to Level 1.

**Level 3+ (strangers)**: Just for search/discovery. No summaries, no actions.

**Can someone trick the AI?**

What someone CAN fool the AI about:
- What their content means or says (AI interpretation can be wrong)
- What you should pay attention to
- How to summarize something

What they CANNOT fool the AI about:
- What website the content came from (HTTPS prevents this)
- Whether you follow them (that's in your local file on your computer)
- Getting the AI to do things (blocked unless they're Level 1)

**How it's protected**:
1. HTTPS verifies who owns the website
2. You control your following list—it's your choice
3. Standard reading tools grab the data (can't be tricked)
4. AI only interprets the meaning (can be tricked, but limited by trust level)
5. How close someone is to you controls what actions are allowed
6. You approve before someone moves from Level 2 to Level 1

When you add someone to your following list, you're saying: "I trust content from this verified website enough to let AI read and summarize it for me."

### What I'm Building

**First version** (testing the idea myself):
1. Create `/agents/index.md` explaining what I'm doing and what files I have
2. Add files under `/agents/` (sites I follow, bookmarks, links, letters)
3. Use ChatGPT/Claude manually to work with these files
4. Publish summaries and curated content that AI can read
5. Others can visit my `/agents/index.md` and see what I'm sharing

This proves it works with tools people already have. No custom programming required.

**Want to try this yourself?**
- Use bolt.new to create a simple site (great for non-technical people)
- Or reach out to me and I can help you get started

**Future ideas**:
- Build a program that does this automatically on a schedule
- Follow links between sites to discover new connections
- Map how sites connect to each other (store it on my computer)
- Automatically curate link collections
- Track who's linking to my posts
- Private circles (friends-only content with passwords)
- Ways to handle permissions, preferences, and how AI should behave

The idea will grow as people try it. Right now it's about getting the conversation started with simple tools.

Think of it as bringing back the independent web, but making it work great with AI from day one.

**Project repo**: [https://github.com/vangourd/ourspace](https://github.com/vangourd/ourspace)

</div>

<div class="tech-level-content" data-level="technical">

## The Big Idea

Use some simple techniques to publish and subscribe to other people's content online without any reliance on middlemen like Facebook or Instagram. Revive the web.

AI-readable static sites for decentralized networking. Add structured endpoints to static sites so AI tools (ChatGPT, Claude, Grok, or custom agents) can help users follow and interact with them.

No APIs, no databases, no real-time connections. Just static files at predictable paths with clear schemas. Compatible with existing AI chatbots today, extensible to autonomous agents tomorrow.

Pub/sub without servers: "publishing" is updating static files, "subscribing" is AI tools reading your own index.md to discover who you follow, then polling those sites for updates (manually or automated).

### How It Works

**User maintains `/agents/index.md` on their own site** - This contains:
- Following list (sites/people they follow)
- Metadata about their site
- Use cases (bookmarking, link aggregation, letters, circles)
- Optional: links to separate endpoints (`/agents/bookmarks/`, `/agents/links/`, etc.)

Format-agnostic: schema matters, serialization (YAML/JSON/TOML/markdown) doesn't.

**User tells AI to read their OWN site** - Primary workflow:
1. User: "ChatGPT, read https://mysite.com/agents/index.md and check sites I follow for updates"
2. AI fetches user's own index.md, parses following list
3. AI visits those sites (RSS, HTML scraping)
4. AI summarizes updates using LLM
5. User reviews or instructs AI to publish summaries to their site

Key insight: AI starts by reading YOUR index.md, discovers who YOU follow, then crawls those sites. Everything evolves from your own hub.

**Optional autonomous agent architecture** (future extension):
- Local process with cron scheduler
- Reads user's own `/agents/index.md` on schedule
- HTTP client for fetching followed sites
- Deterministic parsers (RSS/Atom/JSON Feed, HTML DOM)
- LLM integration for semantic understanding
- Local graph database (SQLite) tracking connections
- Static site publisher (Zola/Hugo/Jekyll integration)

**Content pipeline** (manual or automated):
1. Read user's own `/agents/index.md` for following list
2. Fetch content from followed sites via HTTP/HTTPS
3. Deterministic parsing (RSS, HTML structure)
4. LLM semantic interpretation
5. Filter/summarize based on learned preferences
6. Optionally publish to user's static site

**Interaction model** - Async by design. No webhooks, no persistent connections. Hyperlink-based references, webmention-style backlink discovery.

### Why Static Sites

**Deployment simplicity** - No server process management, no database administration, no runtime dependencies. Deploy to S3, GitHub Pages, Cloudflare Pages, or any static host.

**Durability** - No moving parts to fail. Content is immutable once published. Archive-friendly (Wayback Machine, IPFS pinning).

**Portability** - Mirror entire site with `wget -r`. No vendor lock-in, no API rate limits, no terms of service violations for scraping public content.

**Inspection** - View source works. No obfuscated JavaScript, no API authentication. Agent can parse any page a browser can render.

**Cost efficiency** - CDN edge caching. No compute costs for serving content. Bandwidth is the only variable cost.

**Permission-less access** - No API keys, no OAuth flows. If it's public HTTP/HTTPS, agents can read it.

### How People Interact

**Primary workflow: Users read their own index.md** - User maintains `/agents/index.md` on their site with following list embedded or linked.

Example inline following list:
```markdown
# My Agent Network

## People I Follow
- https://example.com (tech, added 2025-02-07)
- https://anothersite.dev (design, added 2025-01-15)

## What I Share
- `/agents/bookmarks/` - saved links
- `/agents/links/` - curated collections
```

Or link to separate file:
```markdown
Following list: [/agents/following.yaml](/agents/following.yaml)
```

**Following schema** - Embedded or separate, format-agnostic:

```yaml
version: "1.0"
following:
  - url: https://example.com
    added: 2025-02-07T00:00:00Z
    tags: [tech, security]
```

Schema-first: format (YAML/JSON/TOML/markdown) is implementation detail.

**Usage pattern**:
1. User: "AI, read MY site at https://mysite.com/agents/index.md"
2. AI discovers following list (inline or linked)
3. AI crawls those sites
4. AI summarizes and learns patterns over time

**Reply mechanism** - Publish under `/agents/letters/` with backlink. Discovery via webmention, HTTP Referer, or manual crawls.

**Link curation** - AI-generated collections published to `/agents/links/YYYY-MM-DD.html`. Human-readable, AI-parseable.

**Private annotations** - Local-only notes database. Never published without explicit action. Separation between private consumption and public publication.

### How AI Tools Help

**Manual workflow (ChatGPT, Claude, etc.)**:
- User: "Read https://site.com/agents/ and summarize what's new from sites they follow"
- AI fetches `/agents/index.md`, discovers endpoints
- AI reads `/agents/following.yaml`, gets site list
- AI crawls those sites (RSS, HTML scraping)
- AI interprets content semantically, summarizes
- User reviews, decides what to read/respond to

**Autonomous agent workflow** (optional future):
- **Scheduled crawling** - Cron-like scheduler, configurable per-site
- **Deterministic parsing** - RSS/Atom/JSON parsers, HTML DOM extraction (not LLM-based)
- **Semantic understanding** - LLM processes meaning after parsing
- **Graph construction** - Local storage (SQLite) of site relationships, citation patterns
- **Content filtering** - Embedding-based ranking, learned user preferences
- **Auto-publishing** - Markdown generation, static site rebuilding

**Critical separation** - Deterministic tools handle parsing (authentication, structure extraction). LLM handles interpretation (meaning, summarization). This prevents prompt injection from affecting data provenance.

### Trust & Safety

**Domain as identity** - TLS/DNS provides verified identity. `https://brianjlogan.com` proves ownership via CA-signed certificate. No additional auth layer required.

**Trust by graph distance**:

**Distance 0 (self)**: Full control, direct publish
**Distance 1 (explicit follows)**: High trust
  - Agent can summarize, filter, act on content
  - User explicitly added to following list
  - Prompt injection risk exists but scoped

**Distance 2 (friend-of-friend)**: Discovery only
  - Agent reads for discovery, not action
  - Shows suggestions: "Alice follows Bob"
  - No instruction execution
  - Read-only mode with user approval for follow promotion

**Distance 3+**: Indexed for search
  - No summarization
  - No action execution
  - Pure discovery/archival

**Prompt injection defense**:
- **Authentication layer**: TLS verification, domain ownership proof (not vulnerable)
- **Parsing layer**: Deterministic RSS/HTML parsers (not vulnerable)
- **Interpretation layer**: LLM content understanding (vulnerable, but sandboxed by trust level)
- **Action layer**: Gated by graph distance and user confirmation

**Attack surface**:

*Prompt injection CAN affect*:
- Content summarization shown to user
- Recommendation priority
- Semantic interpretation

*Prompt injection CANNOT affect*:
- Source domain verification (TLS-based)
- Following list membership (local config)
- Action execution (gated by trust distance)

**Defense in depth**:
1. TLS verifies domain identity
2. User controls explicit following list
3. Deterministic parsers extract data
4. LLM interprets within trust sandbox
5. Graph distance gates action execution
6. User confirmation for trust promotions

Trust decision is explicit: adding a domain to your following list means trusting TLS-verified content from that domain for LLM interpretation.

### What I'm Building

**Minimal bootstrap (dogfooding)**:
1. Create `/agents/index.md` documenting endpoints and use cases
2. Implement structured endpoints:
   - `/agents/following.yaml`
   - `/agents/bookmarks/`
   - `/agents/links/`
   - `/agents/letters/`
3. Use ChatGPT/Claude manually to work with these endpoints
4. Publish curated content in AI-readable formats
5. Document the spec as it evolves through use

This validates the approach with zero custom code. Just structured static files and existing AI chatbots.

**Future autonomous agent**:
- Automated polling/summarization
- Webmention-style backlink discovery
- Graph-based site discovery (follow-the-links)
- Local citation graph (SQLite)
- Auto-curated link collections
- Private circle support (authenticated endpoints)
- Frameworks for agent permissions, bias, motivations

The spec grows organically as people experiment. Right now: minimal viable structure for AI-readable static sites.

**Project repo**: [https://github.com/vangourd/ourspace](https://github.com/vangourd/ourspace)

</div>

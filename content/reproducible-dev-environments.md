+++
title = "Reproducible Dev Environments with Nix (and Why You Should Care)"
template = "page.html"
date = 2024-11-12T09:30:00Z
[taxonomies]
dates = ["2024-11"]
tags = ["development", "nix", "devcontainers", "tooling"]
[extra]
summary = "Ending 'works on my machine' forever with declarative environments"
+++

Every dev team has that experience. Code works fine locally. CI fails. Turns out someone's running Node 18, someone else has Node 20, and the CI uses Node 16. Hours vanish into environment archaeology.

Or onboarding: "Just follow the README." The README is 6 months out of date. Half the steps don't work. The new hire spends their first day fighting with `brew install` and version managers.

Reproducible development environments solve this. Define the environment as code. Anyone who clones the repo gets the exact same tools.

# The Problem

A typical dev setup requires:
- Language runtime (right version)
- Package manager
- Build tools
- Database clients
- Linters, formatters
- Platform-specific dependencies

Traditional approach: document in README, hope everyone follows it, use version managers (nvm, pyenv) and hope they're configured right.

This breaks constantly:
- README gets outdated
- Version managers require setup and shell config
- Global installs conflict between projects
- "It works on my Mac but not on Linux"

# Nix Flakes: My Current Favorite

[Nix](https://nix.dev/concepts/flakes.html) is a package manager focused on reproducibility. Every package is built with explicit dependencies, producing identical outputs regardless of host system.

[Flakes](https://wiki.nixos.org/wiki/Flakes) are the modern way to define project environments:

```nix
{
  description = "My project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          nodejs_20
          nodePackages.pnpm
          python311
          postgresql_15
          jq
        ];

        shellHook = ''
          echo "Dev environment loaded"
          export DATABASE_URL="postgresql://localhost/myapp"
        '';
      };
    };
}
```

Run `nix develop` and you get Node 20, Python 3.11, PostgreSQL client, jq—all pinned to exact versions from the nixpkgs commit.

From [NixOS & Flakes Book](https://nixos-and-flakes.thiscute.world/development/dev-environments): "A devShell lets you declare a reproducible shell environment with the tools, libraries, and environment variables you need for the development of a specific project."

# Why Nix Works

**True reproducibility.** The `flake.lock` pins nixpkgs to a specific commit. Same `flake.lock` = same packages everywhere.

**Isolation.** Projects don't conflict. Node 18 for one project, Node 20 for another—no global pollution.

**Composability.** Flakes can reference other flakes for shared configs.

**No containers.** Native tools, native performance, native debugging.

# Dev Containers: The Docker Approach

If Nix's learning curve feels steep, [Dev Containers](https://containers.dev/) offer another path. Define your environment in `.devcontainer/devcontainer.json`:

```json
{
  "name": "My Project",
  "image": "mcr.microsoft.com/devcontainers/javascript-node:20",
  "features": {
    "ghcr.io/devcontainers/features/python:1": {
      "version": "3.11"
    }
  },
  "postCreateCommand": "npm install",
  "customizations": {
    "vscode": {
      "extensions": ["dbaeumer.vscode-eslint"]
    }
  }
}
```

VS Code detects this and offers to reopen in a container. Consistent Linux environment regardless of host OS.

**Pros:**
- Lower barrier (most devs have Docker)
- Polished VS Code integration
- Works with GitHub Codespaces

**Cons:**
- Performance overhead (especially macOS file mounts)
- Requires Docker running
- Container vs host debugging can be awkward

# Which to Choose?

**Nix when:**
- Team willing to invest in learning
- Native performance matters
- Working on CLI tools or system software
- Already on Linux

**Dev Containers when:**
- Team varies in technical depth
- VS Code is standard
- Need Linux-specific tooling on Mac/Windows
- Want GitHub Codespaces integration

Both beat the alternative of manual setup and crossed fingers.

# Practical Migration

1. **Audit current setup**: What tools does the project actually need?
2. **Start small**: Just the runtime and build tools first
3. **Add incrementally**: Each time someone hits an environment issue, add that tool to the config
4. **Version in git**: Environment config lives with the code
5. **CI uses the same env**: What works locally works in CI

# The Real Win

From [one Nix user's experience](https://mtlynch.io/notes/nix-dev-environment/): "A Nix development environment solves problems through: Declarative Configuration (describe what you want), Complete Isolation (each project gets its own environment), Reproducibility (same configuration produces identical environments across machines)."

The biggest win isn't technical—it's cultural.

"Read the README" becomes "run one command."

Environment drift stops being a thing. New contributors are productive in minutes. Maintenance becomes routine version bumps rather than firefighting.

**Sources:**
- [Flakes - NixOS Wiki](https://wiki.nixos.org/wiki/Flakes)
- [Flakes - nix.dev](https://nix.dev/concepts/flakes.html)
- [Dev Environments - NixOS & Flakes Book](https://nixos-and-flakes.thiscute.world/development/dev-environments)
- [Per-Project Development Environments with Nix](https://mtlynch.io/notes/nix-dev-environment/)
- [Zero to Nix - Flakes](https://zero-to-nix.com/concepts/flakes/)

# OpenAI API Compatibility Transition Plan

This document describes how KAIRO will phase out legacy compatibility with the OpenAI API. The goal is to minimize disruption while transitioning client code to the native KAIRO interface.

## Phase 1: Compatibility with Automatic Fallback
- **Overview:** KAIRO continues to accept OpenAI-formatted requests. Internally these requests are translated into the new native API.
- **Fallback:** The compatibility layer is enabled by default. KAIRO automatically rewrites OpenAI endpoints to their native equivalents.
- **Recommended Action:** Begin migrating your client code using the `cli-migrate` tool (see below) to generate native API calls.

## Phase 2: Fallback Disabled by Default
- **Overview:** The compatibility layer still exists but is opt-in. Applications must explicitly enable it by setting `KAIRO_ENABLE_OPENAI_COMPAT=1` or passing `--enable-openai-compat` to the server.
- **Impact:** Existing clients still using the OpenAI format will fail unless they enable the flag. This phase encourages completion of migration.

## Phase 3: Compatibility Removed
- **Overview:** All code related to the OpenAI compatibility layer is deleted. Only the native API is available.
- **Impact:** Any remaining clients relying on OpenAI-style requests will stop working. Clients must be updated before this release.

## `cli-migrate` Tool Specification
`cli-migrate` is a helper script that rewrites OpenAI API calls in your codebase to their KAIRO equivalents.

### Basic Usage
```bash
kairo cli-migrate path/to/your/project
```

### Features
- Scans Python source files for OpenAI request patterns such as `openai.ChatCompletion.create`.
- Rewrites imports and function calls to use `kairo.Client` methods instead.
- Provides a `--check` mode to preview changes without modifying files.
- Reports files that could not be automatically migrated so you can update them manually.

The tool is provided under `src/cli_migrate.py` and may be extended for other languages in the future.

## Timeline
1. **Phase 1** begins immediately after this document is merged.
2. **Phase 2** is expected in the next minor release.
3. **Phase 3** will occur after sufficient notice, at least one major version later.

Update your integrations as soon as possible to avoid service disruption.

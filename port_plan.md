# Pika-Hermes Rust Port Plan

## Goal
Fully port the Pika-Hermes Android agent (PokeClaw) to a native, high-performance Rust runtime, integrating Physis Pro for meaning-based search.

## Migration Roadmap

### Phase 1: Core Orchestration (In Progress)
- [ ] Implement AgentLoop state management (Round handling, Tool execution loop).
- [ ] Port TaskPromptEnvelope for prompt parsing.
- [ ] Implement TokenMonitor and TaskBudget logic.

### Phase 2: Tooling & Logic
- [ ] Port InAppSearchGuard and EmailComposeGuard.
- [ ] Implement actual GoogleCalendarTool and GoogleDriveTool logic (replacing placeholders).
- [ ] Port PlaybookManager logic for local LLM optimization.

### Phase 3: Memory & Context
- [ ] Implement semantic injection using the established KBManager (Sled) and Physis Pro client.
- [ ] Port buildDeviceContext and buildMemorySection.

### Phase 4: Verification & E2E
- [ ] Implement integration tests for the full loop (Prompt -> LLM -> Tool -> Memory).
- [ ] Verify against project QA requirements (logging, state stability).

---
## Execution Steps

1. Define Orchestration State: Implement the main agent loop control flow.
2. Port Task Logic: Migrate prompt processing and tool output parsing.
3. Implement Guardrails: Port the critical safety and search guards.
4. Integrate Physis Pro: Finalize the vector-space search and ingestion.

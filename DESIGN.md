# Design

## Purpose

Define the product and interaction contract for the lkjagent terminal
console and snapshot CLI surfaces.

## Source of truth

- Status: Active
- Last refreshed: 2026-06-20
- Primary product surfaces: `lkjagent console`, snapshot CLI commands
- Evidence reviewed: `docs/product/cli.md`, `docs/product/observability.md`,
  `crates/lkjagent-cli/src/console/`

## Brand

- Personality: quiet, operational, precise
- Trust signals: transcript facts, visible daemon state, durable queue status
- Avoid: dashboard clutter, decorative frames, duplicated status chrome

## Product goals

- Goals: make owner attention land on transcript, prompt, and live state
- Non-goals: web UI, mouse UI, full terminal multiplexer
- Success signals: no hidden waiting state, no width overflow, fast scanning

## Personas and jobs

- Primary personas: local operator and coding agent supervisor
- User jobs: monitor progress, answer questions, enqueue work, inspect errors
- Key contexts of use: narrow terminals, SSH, mixed English and Japanese text

## Information architecture

- Primary navigation: one screen plus typed slash commands
- Core routes/screens: console screen, status command, log command
- Content hierarchy: transcript first, then bottom state/control deck

## Design principles

- Principle 1: reserve the top for changing work, not repeated controls
- Principle 2: put action-critical state beside the input prompt
- Tradeoffs: compact text beats visual decoration in terminal space

## Visual language

- Color: restrained ANSI emphasis for state and prompt only
- Typography: terminal monospace, display-width-aware layout
- Spacing/layout rhythm: short sections, one-line dense status where possible
- Shape/radius/elevation: none
- Motion: redraw only; no animation
- Imagery/iconography: none

## Components

- Existing components to reuse: style badges, prompt coloring, store readers
- New/changed components: bottom status deck, width-aware wrapping helpers
- Variants and states: working, waiting, idle, error, stopped
- Token/component ownership: `crates/lkjagent-cli/src/console/`

## Accessibility

- Target standard: readable ANSI text with non-color labels
- Keyboard/focus behavior: input remains visible across redraws
- Contrast/readability: avoid dim text for critical state
- Screen-reader semantics: snapshot text remains plain text
- Reduced motion and sensory considerations: one-second redraw cadence only

## Responsive behavior

- Supported breakpoints/devices: 40-column narrow to 160-column wide terminals
- Layout adaptations: transcript line count, deck position, and prompt fitting
  follow live terminal rows and columns
- Touch/hover differences: none

## Interaction states

- Loading: status deck shows daemon state from store
- Empty: transcript and queue render as `none`
- Error: bottom deck includes error text near prompt
- Success: queued and closed notices appear in the transcript/deck
- Disabled: stopped daemon still accepts queue input with notice
- Offline/slow network: endpoint errors appear as daemon error and events

## Content voice

- Tone: short operational English
- Terminology: daemon, queue, task, maintenance, transcript
- Microcopy rules: no apologies; state facts and next useful action

## Implementation constraints

- Framework/styling system: Rust stdio ANSI terminal rendering
- Design-token constraints: no new TUI framework unless unavoidable
- Performance constraints: one store read per redraw, bounded transcript rows
- Compatibility constraints: no reliance on terminal pixel metrics
- Test/screenshot expectations: render tests for width, anchored bottom deck,
  prompt fitting, CJK text, and body row count

## Open questions

- [ ] Whether to add alternate high-contrast colors / owner / low impact

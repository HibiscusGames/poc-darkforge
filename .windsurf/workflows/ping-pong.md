---
description: Human-AI pair programming ping-pong TDD
---

# Ping-Pong TDD Workflow

This workflow implements Test-Driven Development using the ping-pong pattern between human and AI pair programmers.

> **Current Phase**: [Replace with current phase]
> **Current Cycle**: [Replace with cycle number]

## Workflow Steps

### Setup Phase

1. **Identify Feature**: Define the feature or functionality to implement
2. **Domain Understanding**: Discuss and align on domain language and requirements

### Example Ping-Pong Cycle

#### Human Turn (Red)

1. The human writes a failing test for the functionality they want to implement
2. The human runs the test to confirm it fails (Red)
3. The human types `/ping-pong` to pass control to the AI

#### AI Turn (Green)

1. The AI analyses the failing test
2. The AI writes minimal code to make the test pass
3. The AI runs tests to confirm they now pass (Green)
4. The AI may suggest simple refactoring opportunities
5. The AI returns control to the human

#### Human Turn (Refactor)

1. The human reviews the AI's implementation
2. The human refactors the code if needed while keeping tests passing
3. The human types `/ping-pong` to pass control to the AI

#### AI Turn (Red)

1. The AI analyses the code and the requirements
2. The AI writes **ONE** test to validate a single invariant, requirement or functionality to implement
3. The AI runs the test to confirm it fails (Red)
4. The AI **IMMEDIATELY** returns control to the human after writing and running the failing test

#### Human Turn (Green)

1. The human reviews the AI's test
2. The human implements the minimal code to make the test pass
3. The human runs the test to confirm it passes (Green)
4. The human may suggest simple refactoring opportunities
5. The human types `/ping-pong` to pass control to the AI

#### AI Turn (Refactor)

1. The AI reviews the human's implementation
2. The AI refactors the code if needed while keeping tests passing
3. The AI returns control to the human

### Loop until task is complete

### Completion Phase

1. **Review**: Final review of the implemented feature
2. **Clean Up**: Remove any scaffolding or temporary code
3. **Document**: Document the changes inline and update any relevant docs.
4. **Next Steps**: Identify the next feature to implement

## Guidelines

- Keep tests focused and minimal
- Write just enough code to make tests pass
- Refactor continuously to maintain clean code
- Document domain insights as they emerge
- Switch roles every cycle or two, or if the driver gets stuck
- **Important**: Each participant should only perform ONE step of the Red-Green-Refactor cycle before passing control

## Phase Tracking

To maintain clarity about the current phase, start each message with:

```md
## Current Phase: [Human/AI] ([Red/Green/Refactor])
## Current Cycle: [Number]
## Next Phase: [Human/AI] ([Green/Refactor/Red])
```

Example:

```md
## Current Phase: AI (Green)
## Current Cycle: 2
## Next Phase: Human (Refactor)
```

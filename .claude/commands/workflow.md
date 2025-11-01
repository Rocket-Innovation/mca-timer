---
description: Start the 4-stage development workflow (spec → plan → implement → test) for a new feature
---

You are being asked to start a structured development workflow for a new feature or change.

**IMPORTANT: Use the workflow-advisor agent first**

**CRITICAL: ASK QUESTIONS IF UNCLEAR**
- If the user's request is vague or incomplete, ASK clarifying questions BEFORE starting the workflow
- Don't make assumptions - it's better to ask than to build the wrong thing
- You and any subagent can ask questions at any time

Follow these steps:

1. **Invoke workflow-advisor agent** using the Task tool:
   - Pass the user's feature request to the agent
   - The workflow-advisor will analyze requirements and tell you which specialist agent to invoke next
   - The workflow-advisor may ask clarifying questions before proceeding

2. **Follow the workflow-advisor's guidance:**
   - Stage 1: Specification (technical-spec-writer)
   - Stage 2: Planning (task-breakdown)
   - Stage 3: Implementation (service-implementer)
   - Stage 4: E2E Testing (playwright-e2e-tester)

3. **After each stage:**
   - Return to workflow-advisor for next stage guidance
   - Wait for user approval before advancing stages
   - Any agent can ask questions if they need clarification

**Example:**
```
User: /workflow Add email notifications

You should:
1. Invoke workflow-advisor with the feature request
2. Follow its recommendations for which agent to invoke
3. Progress through all 4 stages
```

**Remember:** You are the orchestrator. You invoke agents based on workflow-advisor's recommendations.

Now invoke the workflow-advisor agent with the user's feature request.

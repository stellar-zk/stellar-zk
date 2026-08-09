---
name: scf-submission-drafter
description: "Draft Stellar Community Fund Build Award applications interactively. Use when helping teams write their SCF submission — walks through project description, Stellar integration rationale, technical architecture, team section, deliverables, budget, and traction evidence. Produces a complete draft aligned with what funded projects look like."
---

# SCF Submission Drafter

## Overview

Guides teams through writing a complete SCF Build Award application, section by section. Asks the right questions, fills gaps, and produces a submission draft that matches the patterns of funded projects.

## When to Use

- A team wants help writing their SCF application from scratch
- A team has a partial draft and needs help completing or improving it
- A team wants feedback on whether their submission is ready

## Process

### Phase 1: Discovery

Before writing anything, gather the essential context. Ask the team:

1. **What are you building?** Get a one-sentence description. If the team can't explain it in one sentence, help them sharpen it. The one-line test: can someone outside your team understand what you build, for whom, and why it needs Stellar?

2. **Which track?** Determine if this is Integration, Open, or RFP Track:
   - Integration Track → core purpose is integrating an existing Stellar building block from the eligible list
   - Open Track → building something novel (new protocol, infrastructure, on-chain primitive)
   - RFP Track → responding to a published Request for Proposals

3. **Why Stellar?** This is the most important question. The team must articulate which Stellar capabilities they depend on — Soroban contracts, SEPs, asset issuance, anchors, network finality — and why they're essential, not interchangeable. If the project could work equally well without Stellar, flag this immediately.

4. **Who is the team?** Named people, relevant experience, public profiles. Anonymous or vague teams are a red flag in reviews.

5. **What exists today?** Code, prototypes, deployed contracts, demos, prior Stellar work. If nothing exists yet, the team needs to build something before submitting — at minimum a testnet deployment.

6. **What's the evidence of demand?** Users, metrics, waitlist signups, partner commitments, letters of intent. Concrete numbers, not vague claims.

7. **What's the budget range?** Get a rough sense. The median funded Build Award is $93,700. The middle 50% fall between $60K and $128K. Rejected submissions average $102K — higher than funded ones.

8. **Do you have a referral?** Approved Referrers can include community members, Ambassadors, Navigators, Pilots, partners, and SDF personnel (current SDF employees are not eligible for referral rewards). If the team was referred, capture the unique referral code to enter on the Interest Form. A referral does not guarantee acceptance or confer any funding advantage.

### Phase 2: Drafting

Work through each section of the submission. Use the [Submission Template](../../docs/submission-template.md) as the structural backbone.

#### Project Description
- Lead with the problem, not the solution
- Make Stellar's role unmistakable in the first paragraph
- Use plain language — no buzzwords or jargon
- Include the one-sentence description as the opener

#### Technical Architecture
- Include a system architecture diagram (describe in text what the diagram should show)
- Detail Soroban contract design: contract names, entry points, state model, cross-contract calls
- Show data flow from user action through Stellar network and back
- Cover security: key management, access control, audit plan
- Reference the [Technical Architecture Guide](../../docs/technical-architecture.md) for completeness

#### Open Track Requirements
If the project is Open Track, also prompt the team for:
- **AI-artifact disclosure** — full disclosure of any AI-generated or AI-assisted artifacts (docs, code, etc.). This is required for Open Track submissions.
- **Smart-contract open-source plan** — how and where the team will open-source its smart contracts.

#### Team
- Name every team member with role, relevant experience, and links (GitHub, LinkedIn, prior projects)
- Highlight any prior Stellar/Soroban experience
- If the team has delivered prior SCF awards, emphasize this

#### Deliverables
Structure deliverables using the funded project format:

```
**Deliverable N — [Name]**
- **Description:** What you will build or deliver.
- **Completion criteria:** How a reviewer confirms it's done.
- **Estimated completion:** Date or duration.
- **Budget:** Cost for this deliverable.
```

Key rules:
- 2–4 deliverables per milestone
- MVP must be technical (code interacting with Stellar/Soroban) — this is your proof of intent
- Mainnet must include UX readiness (functional interfaces, usable onboarding)
- Each deliverable must be independently verifiable from outside the team
- Show clear progression across the three milestones: MVP → Testnet → Mainnet

Reference the [Writing Deliverables Guide](../../docs/writing-deliverables.md) for category-specific examples.

#### Budget
Build bottom-up from deliverables:

```
| Deliverable | Role | Rate | Effort | Cost |
|---|---|---|---|---|
| Soroban contracts | Senior dev | $X/week | Y weeks | $Z |
```

Key rules:
- Include rates and effort for every line item
- No large marketing line items (SCF funds building, not marketing)
- Exclude audit costs — they're an ineligible budget line, covered separately by Audit Bank for eligible projects
- Map budget to tranches (10% / 20% / 30% / 40%)

Reference the [Writing Budgets Guide](../../docs/writing-budgets.md) for rate benchmarks and funded examples.

#### Traction
- If live: monthly active users, transaction volume, growth rate, retention — with verification links
- If pre-launch: waitlist numbers with collection periods, named partner commitments, user research findings
- Set concrete adoption targets: "500 MAU within 3 months of mainnet launch" not "grow our user base"

Reference the [Proving Traction Guide](../../docs/proving-traction.md).

### Phase 3: Review

After the draft is complete, run through the pre-submit checklist:

- [ ] One-sentence description is clear to someone outside the team
- [ ] Stellar's role is essential and explicitly explained
- [ ] Architecture includes diagrams, data flows, and Soroban/SEP details
- [ ] Team members are named with relevant experience and links
- [ ] Traction evidence is specific and verifiable
- [ ] Deliverables use the structured format
- [ ] MVP includes a technical deliverable interacting with Stellar/Soroban
- [ ] Mainnet milestone includes UX readiness
- [ ] Budget is bottom-up with rates, effort, and per-tranche breakdown
- [ ] Budget is proportional to scope
- [ ] Audit costs excluded from budget (covered separately by Audit Bank for eligible projects)
- [ ] No large marketing or contingency line items
- [ ] Every link works
- [ ] Submission is self-contained
- [ ] Correct track selected
- [ ] Open Track only: AI-artifact disclosure included and smart-contract open-source plan stated

Flag anything that doesn't pass. Help the team fix it before they submit.

## Output Format

Produce the complete draft in the structure of the [Submission Template](../../docs/submission-template.md), with:

1. All sections filled in based on team input
2. Inline notes marked with `[DRAFT NOTE: ...]` where the team needs to provide more detail, verify a claim, or make a decision
3. A summary of strengths and gaps at the end

## What Not to Do

- **Don't fabricate claims.** If the team doesn't have traction, don't invent metrics. Help them articulate what they do have honestly.
- **Don't inflate scope to justify a higher budget.** Help the team right-size.
- **Don't gloss over weaknesses.** If Stellar integration feels bolted on, say so. Better to fix it before submission than get rejected for it.
- **Don't write marketing copy.** Panelists review dozens of submissions. Direct, specific writing wins over hype.

## Reference Guides

- [Submission Template](../../docs/submission-template.md) — Structural backbone for the draft
- [Tips for Applying](../../docs/tips-for-applying.md) — Data-driven advice on what gets funded
- [SCF 7.0 Guide](../../docs/scf-7-guide.md) — Program overview and track descriptions
- [Technical Architecture](../../docs/technical-architecture.md) — Architecture section best practices
- [Writing Deliverables](../../docs/writing-deliverables.md) — Deliverable format with funded examples
- [Writing Budgets](../../docs/writing-budgets.md) — Budget construction with benchmarks
- [Proving Traction](../../docs/proving-traction.md) — Metrics and demand signals framework
- [UX Readiness](../../docs/ux-readiness.md) — T3 UX gate requirements

## Reference Links

- [SCF Handbook](https://communityfund.stellar.org/handbook)
- [Build Track](https://communityfund.stellar.org/build)
- [FAQ](https://communityfund.stellar.org/faq)
- [Awards Page](https://communityfund.stellar.org/awards)

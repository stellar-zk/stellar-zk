---
name: scf-tranche-reporter
description: "Help funded SCF teams write tranche submission reports. Use when a team has completed a tranche milestone and needs to document their deliverables for review to unlock the next payment. Covers deliverable documentation, completion evidence, demo preparation, and the specific format tranche reviewers expect."
---

# SCF Tranche Reporter

## Overview

Helps funded SCF Build Award teams write tranche submission reports that clearly demonstrate deliverable completion. Each tranche must be reviewed and approved before the next payment is released. A well-documented tranche submission speeds up review and avoids back-and-forth delays.

## When to Use

- A funded team has completed a tranche milestone and needs to submit deliverables for review
- A team wants to understand what tranche reviewers expect before they start building
- A team received feedback requesting more documentation on a previous tranche

## How Tranche Review Works

Build Awards are paid in four tranches tied to three milestones:

| Tranche | % of Award | When Released |
|---|---|---|
| T0 | 10% | Automatically on award approval — no deliverables required |
| T1 | 20% | After MVP milestone deliverables are reviewed and approved |
| T2 | 30% | After Testnet milestone deliverables are reviewed and approved |
| T3 | 40% | After Mainnet milestone deliverables (including UX readiness) are approved |

**Key facts:**
- T0 is automatic — you receive 10% as soon as your project is approved
- For T1, T2, and T3, you submit milestone deliverables for review via the **SCF Build Tranche Completion Form** (emailed to you after your T0 payment is sent)
- Each submission is reviewed by an SCF reviewer (often a Pilot or delegate)
- The reviewer checks whether each deliverable meets its stated completion criteria
- If deliverables are incomplete or evidence is insufficient, the team is asked to revise
- Delays in tranche approval are most commonly caused by insufficient documentation, not incomplete work
- The team's original application (with its deliverables and completion criteria) is the benchmark

## Process

### Step 1: Gather Deliverables

Pull the deliverables for this tranche from the original application. For each one, confirm:

1. **What was promised?** — Exact deliverable description and completion criteria from the application
2. **What was delivered?** — What the team actually built
3. **Any deviations?** — If the delivered work differs from what was proposed, explain why

### Step 2: Document Each Deliverable

For each deliverable, prepare:

#### Summary
One paragraph describing what was built and how it meets the completion criteria.

#### Evidence

Provide concrete, verifiable proof of completion. The type of evidence depends on the deliverable:

| Deliverable Type | Expected Evidence |
|---|---|
| Smart contract | Deployed contract address (testnet or mainnet), source code repo, passing test suite |
| API / Backend | API documentation, endpoint URLs, request/response examples, source code |
| Frontend / UI | Live URL or demo recording, screenshots of key flows, source code |
| SDK / Library | Published package (npm, crates.io, etc.), documentation, usage examples, source code |
| Documentation | Published docs URL, table of contents, completeness description |
| Integration | Working integration demo, partner confirmation if applicable, transaction evidence |
| Protocol / Infrastructure | Network deployment, performance benchmarks, architecture documentation |
| Research / Design | Published report, design files, findings summary |

**Evidence quality matters:**
- **Links must work.** Test every URL before submitting.
- **Code must be accessible.** Public repo or reviewer-accessible private repo. Include the specific commit hash or tag for the tranche.
- **Demos must be reproducible.** If it's a video, show the full flow. If it's a live URL, make sure it's stable.
- **On-chain evidence is strongest.** Contract addresses, transaction hashes, and block explorer links are unambiguous.

#### Completion Criteria Mapping

For each completion criterion from the original application, explicitly state how it was met:

```
**Completion Criterion:** "Soroban contract deployed on testnet with passing test suite"
**Status:** Met
**Evidence:** Contract deployed at [address] on Stellar Testnet. Test suite: 47 tests passing (see CI run [link]). Source code: [repo link, commit hash].
```

### Step 3: Address Deviations

If anything changed from the original plan:

- **Scope changes** — What changed and why. Was scope added, removed, or modified?
- **Timeline changes** — If delivery was later than estimated, briefly explain.
- **Approach changes** — If the technical approach changed, explain the reasoning.

Small deviations are normal and expected. Reviewers want transparency, not perfection. What causes problems is undisclosed changes or missing deliverables with no explanation.

If a tranche's scope has materially changed, **contact the SCF team before submitting the modified tranche** — explain what changed and why, and update your success metrics or proof formats if needed. Then submit through the SCF Build Tranche Completion Form.

### Step 4: Milestone-Specific Requirements

#### T1: MVP
- Must include a technical deliverable interacting with Stellar or Soroban — this is your proof of intent
- Core functionality should be demonstrable
- Typically: deployed testnet contract, working prototype, or functional proof of concept alongside working app
- Not acceptable: design docs, research papers, or environment setup alone

#### T2: Testnet
- All core functionality should be working on testnet
- Audit readiness (if applicable — eligible projects can request an audit through the Audit Bank around this stage, typically after the testnet tranche when nearly mainnet-ready; SDF covers up to 100% of the cost, with a 5% refundable initial co-pay)
- Integration testing complete
- This is where the project should be functionally complete and testable, even if not yet production-ready

#### T3: Mainnet
- **UX readiness is mandatory** — Functional interfaces, usable onboarding, documentation for end users
- Mainnet deployment
- Comprehensive documentation
- Launch readiness — the project should be usable by its target audience
- Include adoption metrics or launch plan

### Step 5: Format the Report

Structure the tranche submission report clearly.

## Output Format

```
## Tranche [N] Submission: [Project Name]

**Award ID:** [SCF award identifier]
**Tranche:** T[N] ([percentage]%)
**Submission Date:** [Date]

---

### Deliverable 1: [Name]

**Description:** [What was promised]

**What was delivered:** [Brief summary of what was built]

**Completion Criteria:**

| Criterion | Status | Evidence |
|---|---|---|
| [Criterion from application] | Met | [Link/proof] |
| [Criterion from application] | Met | [Link/proof] |

**Links:**
- Source code: [repo URL, commit/tag]
- Deployment: [URL or contract address]
- Demo: [video or live URL]

---

### Deliverable 2: [Name]
[Same structure]

---

### Deviations from Original Plan
[If any — describe what changed and why. If none, state "No deviations from the original application."]

### Additional Notes
[Anything the reviewer should know — blockers resolved, lessons learned, plans for next tranche]
```

## Common Delays and How to Avoid Them

| Delay Cause | Prevention |
|---|---|
| Broken links in submission | Test every URL before submitting |
| Private repos without reviewer access | Make repos public or grant explicit access |
| Vague completion evidence ("see repo") | Point to specific files, commits, and test results |
| Missing deliverables with no explanation | Address every deliverable — even if incomplete, explain status |
| Demo not reproducible | Record a backup video; don't rely solely on live URLs |
| UX not ready at T3 | Plan UX work from MVP, not as a Mainnet afterthought |
| Scope changes not disclosed | Document any changes proactively in the submission |

## What Not to Do

- **Don't submit incomplete tranches without explanation.** If a deliverable isn't done, say so and propose a plan.
- **Don't rely on "see the repo" as evidence.** Reviewers need specific pointers — files, commits, test results.
- **Don't skip T3 UX requirements.** Since SCF 7.0, UX readiness at T3 is mandatory.
- **Don't wait until the last minute.** Prepare documentation as you build, not after.
- **Don't change scope silently.** If plans changed, document it in the tranche report.

## Reference Guides

- [Submitting Tranches](../../docs/submitting-tranches.md) — Full tranche submission guidance
- [Writing Deliverables](../../docs/writing-deliverables.md) — How deliverables should be structured (what reviewers compare against)
- [UX Readiness](../../docs/ux-readiness.md) — T3 UX gate requirements
- [Post-Launch and Growth](../../docs/post-launch-growth.md) — What comes after T3

## Reference Links

- [SCF Handbook](https://communityfund.stellar.org/handbook)
- [Awards Page](https://communityfund.stellar.org/awards)
- [Build Track](https://communityfund.stellar.org/build)

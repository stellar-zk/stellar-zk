---
name: scf-prescreen-checker
description: "Simulate the SCF team's manual prescreen on a Build Award submission — a completeness and eligibility check applied to every full submission before panel review. Checks completeness, Stellar integration, eligibility, budget issues, and common disqualifiers. Use when reviewing a draft Build Award submission for readiness before it goes to the SCF round."
---

# SCF Prescreen Checker

## Overview

Simulates the SCF prescreen that every Build Award submission goes through before it reaches the review panel. The prescreen is a manual completeness and eligibility check by the SCF team — it confirms your submission is complete, reviewable, and ready, not whether your idea is strong. This skill identifies issues that would get flagged at prescreen so you can fix them first.

## How Prescreening Works

Every Build Award submission goes through prescreening before it reaches the review panel — referred and non-referred submissions alike; a referral confers no prescreen exemption. The prescreen is a qualitative, manual check by the SCF team that confirms the submission is complete, reviewable, and meets the core eligibility criteria. It is not a judgment of the strength of your idea, and the SCF team may reach out with suggested edits so you can improve the submission before it is sent to the panel.

The most common prescreen issues:
1. **Incomplete applications** — Missing sections, unanswered fields, broken links
2. **No Stellar integration** — Project could work on any chain or doesn't use Stellar at all
3. **Ineligible teams or proposals** — Sanctioned jurisdictions, duplicate submissions, ineligible project types
4. **Vague or incoherent proposals** — Submission doesn't clearly describe what's being built

## Prescreen Simulation

Run through each check. For each item, report: **PASS**, **FLAG** (potential issue), or **FAIL** (likely prescreen failure).

### 1. Completeness Check

| Check | What to Verify |
|---|---|
| Project description | Is there a clear description of what's being built? |
| Problem statement | Is the problem being solved articulated? |
| Target audience | Is the intended user or market defined? |
| Technical approach | Is there any technical detail beyond a concept? |
| Team information | Are team members named with roles? |
| Deliverables | Are deliverables listed for each tranche? |
| Budget | Is a budget provided with any breakdown? |
| Timeline | Are estimated dates or durations included? |
| Links | Do all provided links actually work? Test every one. |

**FAIL condition:** Any core section is entirely missing or blank.
**FLAG condition:** A section exists but is vague, minimal, or incomplete.

### 2. Stellar Integration Check

| Check | What to Verify |
|---|---|
| Stellar mentioned | Does the submission explicitly reference Stellar? |
| Specific capabilities | Does it name specific Stellar features (Soroban, SEPs, anchors, asset issuance)? |
| Essential vs optional | Would this project work equally well without Stellar? |
| Technical specifics | Are there technical details about how Stellar is used? |
| On-chain component | Is there an on-chain element, or is Stellar only mentioned in passing? |

**FAIL condition:** Stellar is not mentioned, or the project has no meaningful Stellar dependency.
**FLAG condition:** Stellar is mentioned but feels peripheral or interchangeable with other chains.

### 3. Eligibility Check

| Check | What to Verify |
|---|---|
| Project type | Is this a project type the SCF funds (software, infrastructure, protocol)? |
| Duplicate submission | Is this the same project submitted under a different name? |
| Prior funding | If previously funded, has the team delivered on past awards? |
| Budget range | Is the budget within the Build Award range (up to $150K)? |

**FAIL condition:** Ineligible project type or clear track mismatch.
**FLAG condition:** Borderline eligibility or unclear track fit.

### 4. Quality Threshold Check

| Check | What to Verify |
|---|---|
| Coherence | Does the submission tell a coherent story from problem to solution? |
| Specificity | Are claims specific and concrete, or vague and generic? |
| Technical depth | Is there enough technical detail to evaluate the approach? |
| Budget proportionality | Is the budget roughly proportional to scope? |
| Writing quality | Is the submission comprehensible? |

**FAIL condition:** Submission is incoherent, entirely generic, or clearly AI-generated boilerplate.
**FLAG condition:** Low specificity or thin technical detail.

### 5. Red Flag Scan

- Budget over $150K without prior SCF award
- Large marketing line items
- No technical deliverable in MVP milestone
- Team members have no verifiable identity or history
- Project description copied from another submission
- Claims of partnerships with no evidence
- Token launch as a primary deliverable
- No Soroban or on-chain component at all

## Output Format

```
## Prescreen Simulation: [Project Name]

### Overall: [LIKELY PASS / AT RISK / LIKELY FAIL]

### Completeness
- Status: [PASS / FLAG / FAIL]
- Issues: [List any missing or incomplete sections]

### Stellar Integration
- Status: [PASS / FLAG / FAIL]
- Issues: [Is Stellar essential or peripheral?]

### Eligibility
- Status: [PASS / FLAG / FAIL]
- Issues: [Any eligibility concerns]

### Quality Threshold
- Status: [PASS / FLAG / FAIL]
- Issues: [Specificity, coherence, depth concerns]

### Red Flags
- [List any triggered red flags]

### Prescreen Risk Assessment
[1-2 sentence summary]
```

## Scoring Logic

- **LIKELY PASS** — No FAILs, at most 1 FLAG
- **AT RISK** — No FAILs, but 2+ FLAGs
- **LIKELY FAIL** — Any FAIL in Completeness, Stellar Integration, or Eligibility

## Reference Links

- [SCF Handbook](https://communityfund.stellar.org/handbook)
- [Build Track](https://communityfund.stellar.org/build)
- [FAQ](https://communityfund.stellar.org/faq)

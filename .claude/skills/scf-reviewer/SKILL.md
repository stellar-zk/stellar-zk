---
name: scf-reviewer
description: "Review Stellar Community Fund (SCF) Build Award applications. Use when evaluating SCF submissions, covering project fit assessment, technical architecture evaluation, team readiness, traction validation, budget analysis, and funding recommendations."
---

# SCF Project Reviewer

## Overview

Helps review SCF Build Award applications, producing concise, evidence-based assessments suitable for the SCF review form or standalone analysis.

## Verification Requirements

**You must follow up on every link, claim, and reference in the submission.** Do not take applicant statements at face value. For every factual claim — team experience, partner relationships, traction metrics, prior launches, token activity, GitHub repos, on-chain data — independently verify it using web search, block explorers, GitHub, social media, and any other available tools. If a link is provided, fetch it and confirm it supports the claim. If a link is broken or the content contradicts the claim, flag it explicitly in your review.

**Fact-check yourself.** Before finalizing any review output, re-read your own assessment and confirm that every statement you make is supported by evidence you found during research. If you couldn't verify something, say so — do not speculate or assume. State "unverified" or "could not confirm" rather than presenting uncertain information as fact.

This is non-negotiable. Unverified reviews undermine the integrity of the SCF process.

## Review Process

1. **Gather context** — Search for the project, team, token, integration partner; check for prior launches on other chains; look up on-chain data; review the SCF handbook. **Follow every link in the submission and verify claims independently.**
2. **Identify the track** — Determine whether this is an Integration, Open, or RFP submission. Apply the correct track-specific weighting (see below).
3. **Evaluate against SCF criteria** — Does Stellar play a core role? Is the team committed? Does the integration add lasting value? Is the budget appropriate?
4. **Cross-check your findings** — Before producing output, verify that your own statements are evidence-backed. Flag anything you could not independently confirm.
5. **Produce review outputs** in the requested format (form fields, deep dive, checklist, or combination).

## Track-Specific Weighting

Different tracks have different priorities. Weight your evaluation accordingly:

### Integration Track

| Priority | Area | Weight |
|----------|------|--------|
| Highest | **Integration Partner Fit** — Is the partner relevant, on the eligible list, and central to the project? | Critical |
| High | **End-User Value** — Does this put the building block in the hands of real users? | High |
| High | **Traction** — Existing users, demand signals, partner commitments | High |
| Medium | **Technical Architecture** — Sound integration design, correct SDK/API usage | Medium |
| Medium | **Budget** — Proportional to integration scope (not inflated) | Medium |
| Lower | **Novelty** — Not the primary concern for Integration Track | Lower |

### Open Track

| Priority | Area | Weight |
|----------|------|--------|
| Highest | **Ecosystem Impact** — What does this unlock for Stellar? Composability, new capabilities | Critical |
| High | **Technical Depth** — Soroban-native design, contract architecture, security | High |
| High | **Differentiation** — Why is this better/different than alternatives? | High |
| Medium | **Traction** — Community interest, validated need, prior work | Medium |
| Medium | **Budget** — Proportional to technical scope | Medium |
| Medium | **Community Readiness** — Will community voters understand and support this? | Medium |

### RFP Track

| Priority | Area | Weight |
|----------|------|--------|
| Highest | **Spec Compliance** — Does the submission address every RFP requirement? | Critical |
| High | **Relevant Prior Work** — Has the team built this type of infrastructure before? | High |
| High | **Developer Experience** — Setup ease, docs plan, error messages, SDK integration | High |
| High | **Maintenance Plan** — During-grant and post-grant support commitments | High |
| Medium | **Technical Approach** — Sound architecture, appropriate technology choices | Medium |
| Medium | **Timeline** — Realistic, front-loaded, includes docs/testing time | Medium |

## Evaluation Framework

Covers six areas:

| Area | What to Assess |
|------|---------------|
| **Integration Partner Fit** | Is the partner relevant? Is Stellar central to the integration? |
| **Technical Architecture** | Does the project use Soroban smart contracts? Is the design sound? Is Stellar core or shoehorned? |
| **Team Readiness** | Are team members named and credible? Prior Stellar/Soroban experience? |
| **Traction** | Organic demand signals, adoption metrics, existing user base? On-chain metrics verifiable? |
| **Budget & Deliverables** | Is the budget bottom-up and proportional to scope? Are milestones concrete and verifiable? |
| **Ecosystem Commitment** | Long-term Stellar alignment or chain-hopping risk? Maintenance plan? |

## Hard Rules to Check Against

Beyond the qualitative framework, weigh every submission against these handbook rules. A violation is a concrete, defensible reason to flag or downgrade:

- **$150K budget cap.** Awards are capped at $150,000 worth of XLM, and the cap is **lifetime** (accumulated across all of a project's awards). $300K is **not** a general option — it's a case-by-case exception only for a project that has *already* received its full $150K, up to a lifetime maximum of $300K. Flag any request above $150K, and any returning applicant whose cumulative funding would exceed the cap.
- **6-month timeline cap.** The project timeline should not exceed 6 months. Flag plans that clearly need longer than that to reach mainnet.
- **Tranche 1 must be development.** The technical architecture must already be complete at application time. Tranche 1 deliverables should be actual building, not "research," "planning," or "system design." Flag MVP tranches that are just planning.
- **Ineligible costs.** Budgets must not include audit costs (covered separately by the Audit Bank), marketing / user acquisition, bounties / token giveaways / prize pools, legal or entity-registration fees, or reimbursement for past work. Flag any of these as line items.
- **Resubmission timeout.** A project rejected **three times** is blocked from applying for the next **three rounds** (each further failed submission adds another three-round timeout). If you know the submission history, factor this in — a project inside an active timeout should not be in the round.

## Output Formats

### Form Fields
1-2 bullet points per field, direct, no em dashes.

### Deep-Dive Analysis
Lead with what the project is, separate strengths from concerns, address ecosystem fit, end with a clear recommendation.

### Checklist Mode
Run through each evaluation area with a pass/flag/fail assessment:

```
## Checklist Review: [Project Name]
Track: [Integration / Open / RFP]

### Integration Partner Fit
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Technical Architecture
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Team Readiness
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Traction
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Budget & Deliverables
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Ecosystem Commitment
- Status: [PASS / FLAG / FAIL]
- Evidence: [What you found]
- Concerns: [If any]

### Overall Assessment
- Recommendation: [Fund / Fund with conditions / Do not fund]
- Key strength: [One sentence]
- Key concern: [One sentence]
- Verification gaps: [Anything you could not independently confirm]
```

## Common Red Flags

- Max budget requested for a straightforward port
- Stellar positioned as a "secondary" chain
- No adoption targets or success metrics
- Prior inactive token launches on other chains
- No Soroban usage
- Unnamed or anonymous team members
- Claims that cannot be verified (broken links, no public evidence)
- Budget with no line-item breakdown or rates
- "Continue development" as a deliverable
- No maintenance plan

## Common Strengths

- Organic demand from real users
- Real-world recurring use cases (payments, remittances, trade finance)
- Prior Stellar or Soroban experience
- Soroban smart contracts as a core component
- Concrete adoption metrics and milestones
- Proportional budgets that match deliverable scope
- Composable infrastructure that benefits the broader ecosystem
- Named team with verifiable track record

## Reference Links

- [SCF Handbook](https://stellar.gitbook.io/scf-handbook)
- [Build Award](https://stellar.gitbook.io/scf-handbook/scf-awards/build-award)
- [FAQ](https://stellar.gitbook.io/scf-handbook/additional-support/faq)
- [Awards Page](https://communityfund.stellar.org/awards)

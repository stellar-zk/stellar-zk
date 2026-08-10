---
name: scf-budget-builder
description: "Validate and analyze budgets for SCF Build Award submissions. Use when reviewing whether a submission's budget is proportional, justified, and well-structured based on patterns from 215 funded Build Awards."
---

# SCF Budget Validator

## Overview

Validates submission budgets against funded project benchmarks. Used during the review process to assess whether a budget is proportional to scope, properly broken down, and realistic.

## Budget Benchmarks (from 215 funded Build Awards)

| Category | Median | Middle 50% Range |
|---|---|---|
| Applications | $85,000 | $60K–$118K |
| Developer Tooling | $75,000 | $35K–$99K |
| Financial Protocols | $109,000 | $94K–$144K |
| Infrastructure | $116,000 | $62K–$143K |

## Rate Benchmarks

| Role | Typical Range |
|---|---|
| Smart contract developer (Soroban/Rust) | $2,500–$5,000/week |
| Backend developer | $2,000–$4,000/week |
| Frontend developer | $1,500–$3,500/week |
| Full-stack developer | $2,000–$4,000/week |
| UI/UX designer | $1,500–$3,000/week |
| DevOps / infrastructure | $2,000–$4,000/week |
| Technical writer | $1,500–$2,500/week |
| Project management | $1,500–$2,500/week |

## Tranche Structure

SCF Build Awards use a 10% / 20% / 30% / 40% tranche structure:

| Tranche | % of Total | Milestone |
|---|---|---|
| T0 | 10% | Automatic on award approval |
| T1 | 20% | MVP — core Stellar/Soroban integration |
| T2 | 30% | Testnet — full functionality, audit readiness |
| T3 | 40% | Mainnet — production launch, UX, documentation |

## Red Flags

- Total exceeds $150K without strong justification
- Any line item over 40% of total budget without explanation
- Marketing line items (SCF funds building, not marketing)
- Audit costs as a line item (covered separately by Audit Bank for eligible projects)
- Legal fees or entity registration (ineligible)
- Contingency over 5%
- Rates significantly above benchmarks
- Top-down budget that doesn't trace to deliverables
- Budget significantly above category median without proportional scope

## Strengths

- Every line item traces to a specific deliverable
- Rates and effort are explicit
- Tranche allocation roughly follows 10/20/30/40
- Budget proportional to scope complexity
- Audit costs excluded from budget (covered separately by Audit Bank for eligible projects)
- No ineligible expenses

## Reference Links

- [SCF Handbook](https://communityfund.stellar.org/handbook)
- [Build Track](https://communityfund.stellar.org/build)
- [FAQ](https://communityfund.stellar.org/faq)

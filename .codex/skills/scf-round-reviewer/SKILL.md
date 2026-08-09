---
name: scf-round-reviewer
description: "Review and rank an entire SCF round end-to-end from a CSV export. Supports Open Track, Integration Track, and RFP Track with track-specific scoring dimensions. Reads submission data directly from an Airtable CSV, auto-detects columns, categorizes, runs parallel batch reviews, cross-batch calibration, and final ranking. All output is markdown. Use when you have a CSV export of an SCF Build Award round and need to review, score, and rank every submission."
---

# SCF Round Reviewer

## Purpose

Step-by-step instructions to independently review and rank all submissions to an SCF round. The input is a CSV export from Airtable containing all submission data. Supports Open Track, Integration Track, and RFP Track — each with different scoring dimensions and priorities.

## Prerequisites

### External Skills (MUST be installed before running)

These provide domain-specific knowledge that significantly affects scoring quality. **Verify they are installed before starting any review round.**

1. **Stellar Dev Skill** (`stellar/stellar-dev-skill`) — Soroban SDK, Stellar RPC, Horizon, SEPs, Smart Accounts, ecosystem protocols. Use this knowledge when evaluating:
   - Technical Depth / Technical Architecture: Is the submission using Soroban correctly? Are SEP references accurate? Is the RPC vs Horizon choice appropriate?
   - Stellar Integration prescreen: Is Stellar genuinely central or bolted-on?
   - Competitive analysis: What already exists in the Stellar ecosystem?

2. **OpenZeppelin Skills** (`OpenZeppelin/openzeppelin-skills`) — Smart contract security patterns, Stellar contract setup/upgrades. Use this knowledge when evaluating:
   - Smart contract architecture: Are contracts following security best practices?
   - RFP Track: e.g. the current "OZ accounts policy builder" RFP centers on OpenZeppelin's accounts/policy tooling, so this knowledge is directly relevant when an RFP names OpenZeppelin standards. (RFP names rotate quarterly — read the live spec in Phase 0.5 to confirm which RFPs are open.)
   - Budget reasonableness: Is the contract complexity consistent with the budget and timeline?

If either skill is missing, warn the user and recommend installing before proceeding. See CLAUDE.md for installation instructions.

### Files Required
- A CSV file in `data/` exported from the Airtable submissions table
- The sibling review skills in this repo (each at `skills/<name>/SKILL.md`):
  - `../scf-reviewer/SKILL.md` — Core review framework with track-specific weightings
  - `../scf-prescreen-checker/SKILL.md` — 5-area prescreen simulation
  - `../scf-budget-builder/SKILL.md` — Budget validation with category benchmarks
  - `../scf-competitor-analyst/SKILL.md` — Competitive landscape analysis
  - `../fetch-external-doc/SKILL.md` — URL resolution for Google Docs, Drive PDFs, GitHub, Notion, IPFS

### CSV Column Auto-Detection

**Do NOT hardcode column names.** See CLAUDE.md for the full auto-detection process. On every run:
1. Read CSV headers
2. Map them to review fields using semantic matching
3. Write the mapping to `reviews/00-column-mapping.md`
4. Warn the user if any required field can't be mapped

### Parsing the CSV

Use Python's `csv.DictReader` to parse the CSV. Key parsing notes:
- Budget: strip `$`, commas, and parse as float (e.g. "$150,000.00" → 150000.0)
- The CSV may have a BOM character (`\ufeff`) on the first column name — handle it
- Multi-line content is quoted in the CSV — `csv.DictReader` handles this automatically
- Generate a slug for each project: lowercase, replace spaces and special chars with hyphens

### Fetching External Architecture Docs

The technical architecture column often contains URLs to detailed docs (Google Docs, Google Drive PDFs, GitHub repos, Notion pages). These contain critical technical detail that significantly affects Technical Depth and Spec Compliance scores.

**Follow the `fetch-external-doc` skill (`../fetch-external-doc/SKILL.md`) for full URL resolution instructions.** Key points:

- **Google Docs** (`docs.google.com/document/d/{ID}/...`): **MUST use `curl -sL`**: `curl -sL "https://docs.google.com/document/d/{ID}/export?format=txt" -o /tmp/gdoc_{slug}.txt`. Do NOT use WebFetch.
- **Google Drive PDFs** (`drive.google.com/file/d/{ID}/...`): **MUST use `curl -sL`**: `curl -sL "https://drive.usercontent.google.com/download?id={ID}&export=download" -o /tmp/gdrive_{slug}.pdf`. Do NOT use WebFetch.
- **Google Drive alternate formats** (`drive.google.com/open?id={ID}`): Extract the ID, same `curl` approach as above.
- **GitHub files** (`github.com/{owner}/{repo}/blob/{branch}/{path}`): Convert to raw URL `https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}` and WebFetch.
- **Notion pages**: Mark as UNFETCHABLE (client-side JS rendering).
- **IPFS**: WebFetch via `https://ipfs.io/ipfs/{HASH}`. Skip after 15 seconds.
- **Unfetchable** (DocSend, Excalidraw, Figma, Loom, Miro, Whimsical): Mark as UNFETCHABLE.

**IMPORTANT**: When spawning review agents, include the full fetch-external-doc instructions in their prompt so they know how to resolve URLs. Agents won't have access to skill files unless you pass the content to them.

---

## Phase 0: Track Detection

**Executor**: Leader agent (you)

Before doing anything else, determine which track this round is. This changes the entire scoring framework.

### How to Detect

1. **CSV column names** — If there's an "RFP Track Specifics" column, it's likely RFP Track. If there's a "Building Block" or "Integration Partner" column, it's likely Integration Track.
2. **Content inspection** — Read a few rows. Look at descriptions, track columns, round name.
3. **Ask the user** — If ambiguous, ask: "Is this an Open Track, Integration Track, or RFP Track round?"

### Track Differences Summary

| | Open Track | Integration Track | RFP Track |
|---|---|---|---|
| **What it funds** | New projects/protocols for Stellar | Integrations of Stellar building blocks into existing products | Specific deliverables requested via SCF RFPs (listed in handbook) |
| **Top priority** | Ecosystem Impact | Integration Partner Fit | Spec Compliance with the RFP |
| **Unique dimensions** | Differentiation | End-User Value | Developer Experience, Maintenance Plan |
| **Key question** | "Does Stellar need this?" | "Does this bring Stellar to real users?" | "Does this deliver what the RFP asked for?" |

---

## Phase 0.5: RFP Spec Retrieval (RFP Track Only)

**Executor**: Leader agent (you)

**This phase is CRITICAL for RFP Track reviews.** Skip it for Open and Integration Track.

The whole point of RFP Track is that the SCF published specific requirements and teams are proposing to deliver them. You MUST fetch and understand those requirements before reviewing any submissions.

### Step 0.5.1: Find the RFP Spec

The RFP specifications are published in the SCF Handbook. Look for them at:
- `https://stellar.gitbook.io/scf-handbook/scf-awards/build-award/rfp-track` — Primary source for all RFP specs
- `https://communityfund.stellar.org/rfps` — Fallback (may redirect or 404 depending on site changes)
- The `Round` column in the CSV tells you which round this is
- The `track_specifics` column may name the specific RFP category (e.g., for SCF #43 / Q2 2026: "OZ accounts policy builder", "Trustline onboarder", "Passkey UI", "Account Demolisher", or "Contract Source Verification Service")

**RFP names rotate every quarter — never assume which RFPs are open.** Always fetch the live spec and read the current "Current Open RFPs" list before reviewing. The examples above are the Q2 2026 / SCF #43 set; a later round will have a different list.

Fetch the gitbook RFP page using WebFetch. If that fails, try the communityfund.stellar.org URL. If the main page doesn't have the full spec, follow links to individual RFP detail pages.

### Step 0.5.2: Extract RFP Requirements

For each RFP category found in the CSV, extract:
- **Title** — What the RFP is called
- **Problem statement** — What the RFP wants solved
- **Required deliverables** — What must be built (this is the spec)
- **Nice-to-have deliverables** — What's bonus
- **Technical constraints** — Any specific technical requirements (e.g., "must use Soroban", "must support SEP-24")
- **Success criteria** — How SDF will judge completion
- **Budget guidance** — If the RFP specified a budget range

### Step 0.5.3: Write RFP Spec Summary

Write `reviews/00-rfp-spec.md` containing the extracted spec for each RFP category. This file becomes the reference document that all review agents use when checking spec compliance.

### Step 0.5.4: Build a Compliance Checklist

For each RFP category, create a checklist of specific requirements that every submission in that category must address. Example:

Example (using a current SCF #43 / Q2 2026 RFP — **build this from the live spec, do not copy these items verbatim**, as RFPs rotate quarterly):

```markdown
## RFP: Trustline onboarder (Q2 2026 / SCF #43)

### Required Deliverables
- [ ] A well-defined standard for trustline authorization (leveraging CAP-73 or equivalent) that custodians, wallets, exchanges, and issuers can implement predictably
- [ ] Reference implementation demonstrating the standard end-to-end (issuer-side authorization + recipient-side activation)
- [ ] Default landing-page / "activate assets" flow for brokers/CEXes, open source and customizable

### Technical Requirements
- [ ] Supports the full institutional asset lifecycle for classic Stellar assets
- [ ] MiCA-compliance considerations for regulated issuers
- [ ] Open source (full repository, permissive license)

### Success Criteria
- [ ] Working reference implementation
- [ ] Coordination/buy-in from custodians, wallets, exchanges, or issuers
```

This checklist is passed to every review agent for their batch. **Always regenerate it from the live RFP spec for the round you're reviewing** — the example above is the Q2 2026 set and will be stale in later rounds.

---

## Phase 1: Categorize and Index

**Executor**: Leader agent (you)

### Step 1.1: Read the CSV
Read the CSV file from `data/`. Use the auto-detected column mapping. For each submission, extract:
- Project name, slug, budget, referral info, team description, status
- Primary technology (from description, products & services, technical architecture)
- What the project does (from description, track specifics)

Detect the round from the round column (e.g. "SCF #41" → round 41).

### Step 1.2: Categorize
Assign each submission to exactly one category:

| Category | Criteria | Budget Benchmark |
|---|---|---|
| Financial Protocol | DeFi, DEX, lending, staking, trading, derivatives, capital protection | $109,000 |
| Developer Tooling | SDKs, IDEs, debugging tools, testing frameworks, developer experience | $75,000 |
| Infrastructure | Identity, compliance, compute layers, bridges, escrow, public goods, privacy | $116,000 |
| End-User Application | Wallets, merchant payments, card programs, consumer apps | $85,000 |

For RFP Track: also tag each submission with its RFP category (from the track specifics column).

### Step 1.3: Build Batches
Group submissions into batches of 5-6, grouped by category. For RFP Track, group by RFP category instead so reviewers can compare proposals addressing the same RFP. Adjust batch count based on total submissions:
- ≤12 submissions: 2-3 batches
- 13-24 submissions: 3-5 batches
- 25+ submissions: 5-6 batches

### Step 1.4: Create Output Directory
```
mkdir -p reviews
```

### Step 1.5: Write Master Index
Write `reviews/00-master-index.md` containing:
- Round identifier and **track** (Open / Integration / RFP)
- Summary statistics (count, total budget, median, range, cap count, referral breakdown)
- Category tables with columns: #, Slug, Project, Budget, Referrer, Category
- For RFP Track: which RFP category each submission targets
- Batch assignments table
- Budget distribution breakdown
- Referral breakdown (referred vs. not referred)

### Step 1.6: Identify Special Cases
Flag these before review starts:
- Submissions at the $150K budget cap (need extra budget scrutiny)
- Submissions with referrals (informational metadata only — referrals do NOT adjust scores or ranking)
- Any submissions with unusual status values
- For RFP Track: submissions that seem to target a different RFP than expected
- **Multi-RFP submissions**: If a submission's `track_specifics` field contains multiple RFP categories (comma-separated), flag it in the master index. These submissions should be reviewed once with their primary RFP batch, scored against the RFP where they have the strongest alignment, and the review should note coverage (or lack thereof) of secondary RFPs. Report multi-RFP submissions as a finding in the final ranking — the panel needs to decide how to evaluate scope breadth vs. depth.

---

## Phase 1.7: Pre-Fetch External Documents

**Executor**: Leader agent (you)

Before spawning review agents, fetch ALL external documents centrally. Agents fetching Google Docs/Drive independently is unreliable — they often fail silently or skip the `read-gdoc` skill. Pre-fetching ensures every agent has access to the full content.

### Step 1.7.1: Extract All URLs

Scan these columns for every submission:
- `technical_architecture` (highest priority — often contains the detailed arch doc)
- `video` (record but don't fetch video content)
- `description`, `traction`, `products_services` (scan for inline URLs)

### Step 1.7.2: Fetch Documents

For each URL, follow the resolution rules from `../fetch-external-doc/SKILL.md`:

| URL Type | How to Fetch |
|---|---|
| **Google Docs** (`docs.google.com/document/d/{ID}/...`) | **MUST use `curl -sL`** via Bash: `curl -sL "https://docs.google.com/document/d/{ID}/export?format=txt" -o /tmp/gdoc_{slug}.txt` then Read the file. Do NOT use WebFetch — it cannot follow Google's redirect chain. |
| **Google Drive PDFs** (`drive.google.com/file/d/{ID}/...`) | **MUST use `curl -sL`** via Bash: `curl -sL "https://drive.usercontent.google.com/download?id={ID}&export=download" -o /tmp/gdrive_{slug}.pdf` then Read the file. Do NOT use WebFetch. |
| **GitHub files** (`github.com/{owner}/{repo}/blob/{branch}/{path}`) | Convert to `https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}` and WebFetch. |
| **GitHub repos** (`github.com/{owner}/{repo}`) | WebFetch the repo page for README content. |
| **Notion pages** | Mark as UNFETCHABLE (client-side JS rendering, WebFetch returns empty shell). |
| **Unfetchable** (DocSend, Excalidraw, Figma, Loom, Miro, Whimsical) | Mark as UNFETCHABLE. |

**CRITICAL**: For Google Docs and Google Drive files, you MUST use `curl -sL` (via the Bash tool), NOT WebFetch. WebFetch cannot handle Google's redirect chain and will fail silently on nearly all shared docs. The `curl -sL` approach works reliably (~98% success rate on publicly shared docs).

**Batch fetching tip**: For efficiency, write a Python script that loops through all Google Docs/Drive URLs and fetches them all with `subprocess.run(['curl', '-sL', '--max-time', '20', url, '-o', outfile])`.

**Rules:**
- Timeout: `--max-time 20` on curl, skip and mark UNVERIFIED if it fails
- If a Google Doc/Drive fetch returns an HTML error page (check for `<html` + `sign in`), mark UNVERIFIED
- Cap content at 5000 chars per document in the cache file to keep it manageable

### Step 1.7.3: Write External Docs Cache

Write `reviews/00-external-docs.md` with the fetched content:

```markdown
# External Documents Cache — SCF #{round}

## {project-slug}

### Source: {original URL}
**Status**: ACCESSIBLE | UNVERIFIED | UNFETCHABLE
**Type**: Google Doc | Google Drive PDF | GitHub | Notion | Other

{fetched content, or reason for failure}

---
```

This file is the **single source of truth** for external document content. Review agents read from this file instead of fetching URLs themselves.

### Step 1.7.4: Verify Coverage

Log a summary:
- Total URLs found: {n}
- Successfully fetched: {n}
- UNVERIFIED (fetch failed): {n}
- UNFETCHABLE (unsupported platform): {n}
- No external doc: {n} submissions

If more than 30% of architecture docs are UNVERIFIED, pause and ask the user if they want to investigate before proceeding.

---

## Phase 2+3: Parallel Review

**Executor**: Parallel agents (one per batch), created via TeamCreate

### Agent Setup
Create a team and spawn one agent per batch. Each agent receives:
- Their batch assignment (which project slugs to review)
- The CSV file path and which rows are theirs
- The **track** (Open / Integration / RFP) and corresponding scoring dimensions
- The category and budget benchmark for their submissions
- For RFP Track: the RFP spec and compliance checklist from `reviews/00-rfp-spec.md`
- The path to `reviews/00-external-docs.md` — pre-fetched architecture docs and external content

IMPORTANT: Tell each agent to:
- Use SLUG filenames for output files
- Read submission data from the CSV using the auto-detected column mapping
- If a WebFetch hangs, mark it UNVERIFIED and move on
- **Read external docs from `reviews/00-external-docs.md`** — the leader agent already pre-fetched architecture docs (Google Docs, Drive PDFs, GitHub files, Notion pages, etc.). Check the cache first for any URL before fetching it yourself. Only fetch URLs that aren't in the cache (e.g., inline links discovered during review).
- Use the correct track-specific scoring dimensions (see below)

### Per-Submission Review Process

Each agent performs these steps for every submission in their batch:

#### Step A: Extract Submission Data from CSV
Parse the row for this submission using the auto-detected column mapping.

#### Step B: Prescreen (5 checks)

| Check | What to Evaluate | Rating |
|---|---|---|
| Completeness | All key fields populated? Team description present? Budget provided? Tranches with **concrete, verifiable deliverables** (not just business outcomes like "$1M volume" or "2 clients live")? Per-tranche or per-deliverable **budget breakdown** present? | PASS / FLAG / FAIL |
| Stellar Integration | Is Stellar genuinely central (not bolted-on)? Soroban, Horizon, SEPs? Could this work on any chain? | PASS / FLAG / FAIL |
| Eligibility | Budget within $150K limit? Appropriate status? | PASS / FLAG / FAIL |
| Quality Threshold | Professional writing? Coherent technical approach? Realistic timelines? | PASS / FLAG / FAIL |
| Red Flag Scan | Plagiarism signals? Impossible claims? Unverifiable traction? Scope creep? | PASS / FLAG / FAIL |

**RFP Track additional prescreen check:**

| Check | What to Evaluate | Rating |
|---|---|---|
| RFP Alignment | Does the submission actually address the RFP it claims to? Is it solving the stated problem or doing something tangential? | PASS / FLAG / FAIL |

Prescreen result:
- LIKELY PASS: No FAILs, at most 1 FLAG
- AT RISK: No FAILs, 2+ FLAGs
- LIKELY FAIL: Any FAIL

#### Step C: Link Verification
Check the technical architecture URL, video URL, and any URLs found in description, traction, or products & services fields.

**For architecture docs and other external links**: Look up the project slug in `reviews/00-external-docs.md` first — the leader agent already pre-fetched Google Docs, Drive PDFs, GitHub files, Notion pages, and other external content. Use the cached content for your review. Only fetch URLs that aren't in the cache (e.g., new inline links you discover during review).

For each URL, record:
- Status: ACCESSIBLE or UNVERIFIED
- Brief note on what was found

Rules:
- Do NOT penalize for inaccessible links (may be temporarily down)
- If WebFetch hangs for more than 15 seconds, skip and mark UNVERIFIED
- Note any GitHub repos with very few commits or recent creation dates

#### Step D: Score Dimensions (Track-Specific)

Use the scoring dimensions for the detected track:

##### Open Track — 6 Dimensions

| Dimension | Weight | Scoring Guide |
|---|---|---|
| Ecosystem Impact (x3) | CRITICAL | 5: Category-creating, unlocks new Stellar capability. 4: Significant gap-filler. 3: Useful addition. 2: Marginal. 1: No ecosystem benefit. |
| Technical Depth (x2) | HIGH | 5: Novel architecture, Soroban-native, formal verification. 4: Detailed contract design, deep Soroban understanding. 3: Adequate. 2: Surface-level. 1: No substance. |
| Differentiation (x2) | HIGH | 5: First-of-kind on Stellar AND cross-chain. 4: First on Stellar. 3: Some differentiation. 2: Crowded space. 1: Direct duplicate. |
| Traction (x1.5) | MEDIUM | 5: Live product with on-chain metrics. 4: Working product. 3: Testnet/pilot. 2: Concept with LOIs. 1: Idea stage. |
| Budget (x1.5) | MEDIUM | 5: Bottom-up per-deliverable breakdown, below benchmark, justified. 4: Detailed per-tranche breakdown, at benchmark. 3: Adequate with some breakdown. 2: No per-deliverable breakdown OR vague OR above benchmark. 1: No breakdown at all. **A low total alone does NOT justify a high score — "cheap" is not "well-justified."** |
| Community Readiness (x1.5) | MEDIUM | 5: Deep Stellar history, prior SCF delivery. 4: Proven blockchain team. 3: Credible. 2: Thin credentials. 1: Unverifiable. |

**Composite**: `weighted / 57.5 * 100`

##### Integration Track — 6 Dimensions

| Dimension | Weight | Scoring Guide |
|---|---|---|
| Integration Partner Fit (x3) | CRITICAL | 5: Tier-1 partner, Stellar is central to integration, massive user reach. 4: Strong partner, genuine Stellar integration. 3: Reasonable partner, adequate integration. 2: Peripheral partner or Stellar is bolted-on. 1: No real integration partner or ineligible building block. |
| End-User Value (x2.5) | HIGH | 5: Puts Stellar in hands of millions of real users. 4: Significant user reach with clear UX. 3: Moderate reach. 2: Niche audience or unclear UX path. 1: No clear end-user benefit. |
| Traction (x2) | HIGH | 5: Live product with significant existing users/volume. 4: Working product with verified users. 3: Pilot or beta. 2: LOIs only. 1: Idea stage. |
| Technical Architecture (x1.5) | MEDIUM | 5: Elegant integration, correct SDK/API usage, security model. 4: Sound design. 3: Adequate. 2: Superficial. 1: No technical detail. |
| Budget (x1.5) | MEDIUM | 5: Per-deliverable breakdown, proportional to scope, well-justified. 4: Per-tranche breakdown, reasonable. 3: Adequate with some breakdown. 2: No per-deliverable breakdown OR inflated. 1: No justification. **A low total alone does NOT justify a high score — "cheap" is not "well-justified."** |
| Ecosystem Commitment (x1) | LOWER | 5: Deep Stellar commitment, maintenance plan, open source. 4: Clear commitment. 3: Some commitment signals. 2: Possible chain-hopper. 1: No Stellar commitment evident. |

**Composite**: `weighted / 57.5 * 100`

##### RFP Track — 6 Dimensions

| Dimension | Weight | Scoring Guide |
|---|---|---|
| Spec Compliance (x3) | CRITICAL | 5: Addresses every RFP requirement, exceeds expectations. 4: Addresses all required deliverables, reasonable approach. 3: Addresses most requirements, some gaps. 2: Partially addresses RFP, significant gaps. 1: Does not address the RFP or is tangential. |
| Relevant Prior Work (x2.5) | HIGH | 5: Team has built this exact type of infrastructure before, proven track record. 4: Strong relevant experience. 3: Some relevant experience. 2: General dev experience but nothing specific. 1: No relevant prior work. |
| Developer Experience (x2) | HIGH | 5: Exceptional DX plan — setup ease, comprehensive docs, clear error messages, SDK examples. 4: Good DX plan. 3: Adequate. 2: DX is an afterthought. 1: No DX consideration. |
| Maintenance Plan (x1.5) | MEDIUM | 5: Detailed during-grant AND post-grant maintenance, SLAs, monitoring, update plan. 4: Clear maintenance commitments. 3: Some maintenance mentioned. 2: Vague. 1: No maintenance plan. |
| Technical Approach (x1.5) | MEDIUM | 5: Elegant architecture, appropriate tech choices, security model. 4: Sound approach. 3: Adequate. 2: Questionable choices. 1: No technical detail. |
| Budget & Timeline (x1) | LOWER | 5: Realistic timeline front-loaded with hard work, budget justified per deliverable with line items. 4: Per-tranche breakdown, reasonable timeline. 3: Adequate with some breakdown. 2: Unrealistic OR inflated OR no per-deliverable breakdown. 1: No justification. **A low total alone does NOT justify a high score.** |

**Composite**: `weighted / 57.5 * 100`

**IMPORTANT for RFP Track**: Before scoring Spec Compliance, the agent MUST read `reviews/00-rfp-spec.md` and check the submission against each item in the compliance checklist. The review file must include a filled-out compliance checklist showing which RFP requirements are addressed and which are not.

#### Step E: Competitive Analysis
Do a WebSearch for competing projects:
1. Search Stellar ecosystem specifically
2. Search broader blockchain ecosystem if relevant
3. For RFP Track: also check if other submissions in the same RFP category take a notably different/better approach
4. Note any live competitors the submission didn't mention

#### Step F: Compute Composite Score
Use the formula for the detected track (all normalize to the same 0-100 scale):
```
composite = weighted / 57.5 * 100
```

#### Step G: Assign Recommendation
- FUND: composite >= 75
- FUND WITH CONDITIONS: composite 60-74
- DO NOT FUND: composite < 60

Use judgment at boundaries.

#### Step H: Write Review File

Write to `reviews/{slug}.md`. Each review file follows this structure:

**Title**: `# {Project Name} — SCF #{round} Review`

**Header table** with columns Field and Value, containing: Project, Budget, Category, Track, Referrer (or "None").

**For RFP Track only — Spec Compliance Checklist** (`## RFP Compliance`): Use checkbox format for EVERY item from the RFP spec checklist in `00-rfp-spec.md`. Format:
- `- [x] {requirement} — {brief evidence}` for MET
- `- [ ] {requirement} — PARTIALLY MET: {what's missing}` for partial
- `- [ ] {requirement} — NOT ADDRESSED` for missing

Group into "Required Deliverables", "Must-Build Components", and "Nice-to-Have" subsections matching the RFP spec structure. This comes BEFORE the prescreen.

**Prescreen section** (`## Prescreen`): Table with columns Check, Result, Notes. For RFP Track, includes the extra "RFP Alignment" check.

**Link Verification section** (`## Link Verification`): Table with columns URL, Status, Notes.

**Scoring section** (`## Scoring`): Subsections for each dimension (track-specific). Each subsection title includes the raw score and weighted value. Body is 2-4 sentences of specific evidence.

**Composite Score section** (`## Composite Score`): Summary table with columns Dimension, Raw, Weight, Weighted — plus TOTAL row. Then a single normalized score line formatted exactly as: `**Normalized Score**: {number} / 100` — do NOT include the formula, just the final number.

**Recommendation section** (`## Recommendation: FUND / FUND WITH CONDITIONS / DO NOT FUND`): 2-3 sentence summary.

### After All Reviews Complete
Each agent sends a message to the team lead with their batch summary.

---

## Phase 4: Cross-Comparison and Calibration

**Executor**: 2 calibration agents

### Step 4.1: Collect All Scores
Sort all submissions by composite score. Divide into two groups:
- Top half + Bottom quarter → Agent A
- Middle submissions (borderline zone) → Agent B

### Step 4.2: Agent A — Top/Bottom Consistency

For each submission in their group:
1. Read the review file
2. Fetch external architecture docs if not already fetched
3. Cross-check scoring consistency across batch reviewers
4. For RFP Track: verify that Spec Compliance scores are consistent — two submissions addressing the same RFP requirement should not score wildly differently
5. Write findings to `reviews/calibration-top-bottom.md`

### Step 4.3: Agent B — Borderline Zone

For each middle submission:
1. Same doc fetching as Agent A
2. Pairwise comparisons for submissions within 5 points
3. Tier boundary analysis — the 60-point FUND WITH CONDITIONS / DO NOT FUND line is critical
4. For RFP Track: compare proposals for the same RFP category directly — which one better addresses the spec?
5. Write findings to `reviews/calibration-middle.md`

### Step 4.4: Apply Adjustments
Rules:
- Only adjust if there is clear evidence of mis-scoring
- Adjustments should be to specific dimensions
- Document every adjustment with reason

---

## Phase 5: Final Ranking

**Executor**: Leader agent

### Step 5.1: Do NOT Score Referrals
Referrals **do not influence** evaluation or funding. The handbook's Referral Program is explicit (§3.1/3.2): a referral "does not guarantee acceptance into SCF, selection, funding, or any other benefit," and referrers must not imply their referral confers any advantage. **Do not add any referral points to any submission's score.** Ranking is purely substance-based.

Track referrers only as informational metadata. The referrer *reward* is a separate, fully discretionary payout (up to **1%** of the awarded budget, aggregate cap of **6 rewards per application cycle**, paid only when the final Build Award tranche is disbursed; current SDF employees are ineligible). It is a payout to the referrer, not a boost to the applicant — keep it out of the ranking.

### Step 5.2: Write Final Ranking Report
Write `reviews/01-ranking.md` with these sections:

**Section 1: Ranking (Substance-Only) — PRIMARY**
This is the ranking. Full table with columns: Rank, Project, Category, Budget, Calibrated Score, Referrer (informational only). For RFP Track: include RFP Category column. The Referrer column is metadata — it must NOT change the order.

**Section 2: Referrer Sidebar (Non-Scoring)**
A non-scoring reference list of which submissions named an approved referrer, for the SCF team's awareness of potential referrer-reward eligibility. State plainly that these referrals did not influence the ranking and that any referrer reward is a separate discretionary payout (up to 1%, max 6/cycle, final-tranche, SDF employees ineligible).

**Section 3: Do Not Fund — With Specific Feedback**
For each DO NOT FUND submission:
- **Why**: 2-3 sentences
- **To improve**: 2-3 specific actionable suggestions
- For RFP Track: specifically note which RFP requirements were not addressed

**Section 4: Additional Analysis**
- Calibration adjustments table
- Category-level patterns
- Budget distribution analysis
- Statistical summary
- For RFP Track: per-RFP-category comparison (which proposal best addresses each RFP?)

### Step 5.3: Write Results CSV

Write `reviews/results.csv` — a machine-readable summary of all results. This allows the SCF team to import results back into Airtable or use them in spreadsheets.

**Columns:**

| Column | Description |
|---|---|
| `Project` | Project name (as it appears in the CSV) |
| `Slug` | Kebab-case slug |
| `Track` | Open / Integration / RFP |
| `Category` | Financial Protocol / Developer Tooling / Infrastructure / End-User Application |
| `RFP Category` | (RFP Track only) Which RFP the submission targets |
| `Budget` | Requested budget as number |
| `Referrer` | Referrer name or empty (informational only — does NOT affect the score or ranking) |
| `Prescreen` | LIKELY PASS / AT RISK / LIKELY FAIL |
| `Dim 1 Raw` | First scoring dimension raw score (1-5) |
| `Dim 1 Name` | Name of first dimension (track-dependent) |
| `Dim 2 Raw` | Second dimension raw score |
| `Dim 2 Name` | Name of second dimension |
| `Dim 3 Raw` | Third dimension raw score |
| `Dim 3 Name` | Name of third dimension |
| `Dim 4 Raw` | Fourth dimension raw score |
| `Dim 4 Name` | Name of fourth dimension |
| `Dim 5 Raw` | Fifth dimension raw score |
| `Dim 5 Name` | Name of fifth dimension |
| `Dim 6 Raw` | Sixth dimension raw score |
| `Dim 6 Name` | Name of sixth dimension |
| `Composite Score` | Normalized score (0-100) — this is the final substance-only score; referrals do not adjust it |
| `Recommendation` | FUND / FUND WITH CONDITIONS / DO NOT FUND |
| `Key Strength` | One-sentence summary |
| `Key Concern` | One-sentence summary |

Use Python's `csv.writer` to generate the CSV with proper quoting (some fields contain commas).

---

## Common Pitfalls

1. **WebFetch hangs**: Skip after 15 seconds and mark UNVERIFIED.
2. **Score inflation**: Enforce that 5 is rare and 1-2 should be used for weak submissions.
3. **Batch reviewer getting stuck**: Replace after 10 minutes of no output.
4. **Slug filenames**: Explicitly tell agents to use slugs, not IDs.
5. **External docs changing scores**: Architecture docs often contain 10x more detail. Always fetch.
6. **Chain-hopping detection**: Score lower on Ecosystem Impact / Integration Partner Fit.
7. **CSV parsing**: Handle BOM characters, multi-line quoted fields, currency formatting.
8. **Column name changes**: NEVER hardcode column names. Always auto-detect from headers.
9. **RFP spec not fetched**: For RFP Track, ALWAYS fetch the RFP spec before reviews start. Without it, Spec Compliance scoring is meaningless.
10. **Wrong track scoring**: Double-check that agents are using the correct track-specific dimensions. Open Track dimensions applied to an RFP Track review will produce garbage.

---

## Timing Expectations

| Phase | Duration | Parallelism |
|---|---|---|
| Phase 0: Track detection + RFP spec (if applicable) | 5-10 min | Serial |
| Phase 1: Categorize and index | 10-15 min | Serial |
| Phase 2+3: Reviews | 20-40 min | Parallel agents |
| Phase 4: Calibration | 15-25 min | 2 parallel agents |
| Phase 5: Final ranking | 10-15 min | Serial |
| **Total** | **~60-105 min** | |

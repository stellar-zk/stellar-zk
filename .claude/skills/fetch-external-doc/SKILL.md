---
name: fetch-external-doc
description: "Fetch external documents linked in SCF submissions — Google Docs, Google Drive PDFs, GitHub, Notion, IPFS. Use when a submission links to an architecture doc, whitepaper, or technical spec hosted externally. Handles URL transformation for Google services."
---

# Fetch External Document

## Purpose

SCF submissions frequently link to external architecture documents, whitepapers, and technical specs. These often contain 10x more technical detail than the submission text and can significantly affect Technical Depth and Spec Compliance scores. This skill handles fetching them reliably.

## URL Pattern Detection

When you encounter a URL in a submission (especially in the Technical Architecture column), identify the type and follow the corresponding fetch strategy:

### Google Docs

**URL patterns:**
- `docs.google.com/document/d/{DOC_ID}/...`
- `docs.google.com/document/d/{DOC_ID}/edit`
- `docs.google.com/document/d/{DOC_ID}/view`

**How to fetch — MUST use `curl`, not WebFetch:**
1. Extract the `{DOC_ID}` from the URL (the long alphanumeric string after `/document/d/`)
2. Construct the export URL: `https://docs.google.com/document/d/{DOC_ID}/export?format=txt`
3. Fetch using `curl -sL` via the Bash tool:
   ```bash
   curl -sL "https://docs.google.com/document/d/{DOC_ID}/export?format=txt" -o /tmp/gdoc_{slug}.txt
   ```
4. Read the downloaded file with the Read tool
5. If that fails, try: `https://docs.google.com/document/d/{DOC_ID}/pub`

**IMPORTANT**: Do NOT use WebFetch for Google Docs — it cannot follow Google's redirect chain. `curl -sL` handles this correctly and reliably fetches publicly shared docs.

**Example:**
- Input: `https://docs.google.com/document/d/1aBcDeFgHiJkLmNoPqRsTuVwXyZ/edit?usp=sharing`
- Export URL: `https://docs.google.com/document/d/1aBcDeFgHiJkLmNoPqRsTuVwXyZ/export?format=txt`
- Command: `curl -sL "https://docs.google.com/document/d/1aBcDeFgHiJkLmNoPqRsTuVwXyZ/export?format=txt" -o /tmp/gdoc_myproject.txt`

### Google Drive Files (PDFs, etc.)

**URL patterns:**
- `drive.google.com/file/d/{FILE_ID}/view`
- `drive.google.com/file/d/{FILE_ID}/preview`
- `drive.google.com/open?id={FILE_ID}`
- `drive.google.com/uc?id={FILE_ID}`

**How to fetch — MUST use `curl`, not WebFetch:**
1. Extract the `{FILE_ID}` from the URL
2. Construct the download URL: `https://drive.usercontent.google.com/download?id={FILE_ID}&export=download`
3. Fetch using `curl -sL` via the Bash tool:
   ```bash
   curl -sL "https://drive.usercontent.google.com/download?id={FILE_ID}&export=download" -o /tmp/gdrive_{slug}.pdf
   ```
4. Read the downloaded file with the Read tool
5. If the file is a PDF, the Read tool can extract text from it

**IMPORTANT**: Do NOT use WebFetch for Google Drive files — it cannot follow Google's redirect chain. `curl -sL` handles this correctly.

**Example:**
- Input: `https://drive.google.com/file/d/1xYzAbCdEfGhIjKlMnOpQrS/view?usp=sharing`
- Download URL: `https://drive.usercontent.google.com/download?id=1xYzAbCdEfGhIjKlMnOpQrS&export=download`
- Command: `curl -sL "https://drive.usercontent.google.com/download?id=1xYzAbCdEfGhIjKlMnOpQrS&export=download" -o /tmp/gdrive_myproject.pdf`

### Google Drive Folders

**URL patterns:**
- `drive.google.com/drive/folders/{FOLDER_ID}`

**How to handle:**
- Cannot fetch folder contents directly
- Mark as UNFETCHABLE
- Note in the review: "Google Drive folder link — cannot fetch contents automatically"

### GitHub

**URL patterns:**
- `github.com/{owner}/{repo}/blob/{branch}/{path}` — file view
- `github.com/{owner}/{repo}` — repo root
- `github.com/{owner}/{repo}/tree/{branch}/{path}` — directory

**How to fetch:**
1. For file links: convert to raw URL by replacing `github.com` with `raw.githubusercontent.com` and removing `/blob/`
   - Input: `https://github.com/org/repo/blob/main/ARCHITECTURE.md`
   - Raw: `https://raw.githubusercontent.com/org/repo/main/ARCHITECTURE.md`
2. Fetch using WebFetch with the raw URL
3. For repo roots or directories: use WebFetch on the original URL, or use `gh api` if available
4. For README-type content: try `https://raw.githubusercontent.com/{owner}/{repo}/{branch}/README.md`

### Notion Pages

**URL patterns:**
- `notion.site/{page-slug}-{PAGE_ID}`
- `{workspace}.notion.site/{page-slug}-{PAGE_ID}`
- `notion.so/{PAGE_ID}`

**How to fetch:**
- Notion pages are client-side rendered (JavaScript) — WebFetch returns only the loading shell, not the actual content
- Mark as UNFETCHABLE with note: "Notion page — requires JavaScript rendering, content not accessible via WebFetch"
- If the submission relies heavily on a Notion doc for its architecture, note this as a gap in the review — the reviewer could not access the full technical detail

### IPFS Links

**URL patterns:**
- `ipfs.io/ipfs/{HASH}`
- `gateway.pinata.cloud/ipfs/{HASH}`
- `{any-gateway}/ipfs/{HASH}`
- `ipfs://{HASH}`

**How to fetch:**
- Use WebFetch with a public gateway: `https://ipfs.io/ipfs/{HASH}`
- If the default gateway is slow, try: `https://gateway.pinata.cloud/ipfs/{HASH}`
- Skip after 15 seconds — IPFS gateways can be unreliable

### Unfetchable URLs

These services cannot be reliably fetched — mark as UNFETCHABLE:
- **Notion** (`notion.site/...`, `notion.so/...`) — client-side JS rendering, WebFetch returns empty shell
- **DocSend** (`docsend.com/...`) — requires email/login
- **Excalidraw** (`excalidraw.com/...`) — renders as canvas, no text
- **Figma** (`figma.com/...`) — requires authentication
- **Loom** (`loom.com/...`) — video, no text extraction
- **Miro** (`miro.com/...`) — requires authentication
- **Whimsical** (`whimsical.com/...`) — visual diagrams, no text extraction

## Fetch Strategy

When processing a submission's links:

1. **Identify all URLs** in the technical architecture, description, traction, and products & services fields
2. **Classify each URL** using the patterns above
3. **Fetch in order of priority:**
   - Architecture docs first (highest impact on scoring)
   - GitHub repos second
   - Other links third
4. **For each fetch, record:**
   - Original URL
   - Transformed URL (if applicable)
   - Status: ACCESSIBLE / UNVERIFIED / UNFETCHABLE
   - Brief summary of what was found (or why it failed)
5. **Timeout:** If any fetch takes more than 15 seconds, skip it and mark UNVERIFIED. Do not get stuck.

## Using the `read-gdoc` Skill

The `read-gdoc` skill uses `curl -sL` under the hood — which is the correct approach. You can invoke it via the Skill tool:

```
Skill: read-gdoc
Args: https://docs.google.com/document/d/{DOC_ID}/edit
```

For bulk fetching (e.g., Phase 1.7 pre-fetch), use `curl -sL` directly in a Python/Bash loop instead of invoking the skill per-doc — it's faster and more reliable for batch operations.

## Common Issues

- **Google "virus scan" warning**: Large Google Drive files trigger a "can't scan for viruses" page. The download URL still works but may return HTML instead of the file. Check the response content.
- **Google sharing permissions**: If a doc returns a login page, it's not publicly shared. Mark as UNVERIFIED and note "requires authentication."
- **GitHub rate limiting**: If you're fetching many GitHub URLs, you may hit rate limits. Space out requests or use `gh api` with authentication.
- **PDF content**: WebFetch can't extract text from PDFs well. Note "PDF — limited text extraction" and work with whatever you get.
- **Redirects**: Some URLs redirect (e.g., shortened links). WebFetch should follow redirects, but if it returns the redirect target URL, make a second fetch to that URL.

# Manual Smoke Walkthrough

Use this checklist after `npm run tauri dev` launches successfully.

## Required inputs

- Local Ollama service running and reachable at the configured URL.
- A pulled local model such as `llama3.2`.
- Jira Data Center base URL and PAT with access to a safe test ticket.
- Confluence Data Center base URL and PAT with access to a non-production test space.

## Core flow

1. Launch the app.
   Pass: The main window renders, templates load, and the app stays responsive for at least 5 minutes.
   Fail: Crash, blank shell, or repeated runtime errors block use.

2. Open Settings and test each service connection.
   Pass: Ollama, Jira, and Confluence each show a successful connection result.
   Fail: A known-good endpoint and token cannot connect or the state is not saved.

3. Fetch a Jira ticket by key.
   Pass: Summary, description, labels, and comments populate from the selected ticket.
   Fail: Valid tickets cannot load, rich-text fields are blank, or comments are malformed.

4. Draft with AI.
   Pass: The app generates markdown from the Jira ticket and selected template without freezing.
   Fail: Generation errors, hangs, or malformed output block drafting.

5. Edit and review the draft.
   Pass: Markdown edits update preview, and quality scoring still works.
   Fail: Preview desync, content corruption, or broken scoring.

6. Save, reload, and delete the draft.
   Pass: Saved drafts appear in `My Drafts`, reload correctly, and delete cleanly.
   Fail: Data is missing, stale, or cannot be removed cleanly.

7. Publish to Confluence.
   Pass: Spaces load, the sensitive-data scan runs, and publish succeeds to the test space with correct content.
   Fail: Space listing fails, publish errors, or the page content is materially wrong.

8. Restart persistence check.
   Pass: Relaunch keeps saved URLs, Jira reconnect works, and drafts remain available.
   Fail: Jira base URL, Confluence URL, or saved drafts disappear unexpectedly.

## Evidence to capture

- Date and machine used.
- Jira test ticket key.
- Confluence test space key.
- Ollama model used.
- Pass or fail result for each step above.
- Any bug repro steps, logs, or screenshots for failed steps.

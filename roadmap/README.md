# Bezel roadmap → GitHub

`roadmap.json` is the single source of truth: labels, milestones, 16 epics, 83 issues.

```
gh auth refresh -s project,read:project     # once, for Projects v2
REPO=your-org/bezel ./bootstrap.sh          # idempotent; DRY_RUN=1 to preview
```

What it creates:

- labels (`type:*`, `area:*`, `size:*`, `good first issue`, `blocked`) — updated in place on re-run
- milestones with due dates (0.0 Phase 0, 0.1, 0.2, 0.3, Native host gated)
- one issue per task, titled `E<n> · <task>`, labelled with its epic's area + type + size, assigned to the milestone
- one tracking issue per epic with a task checklist linking the child issues and the PRD requirement IDs it satisfies
- a Projects (v2) board named "Bezel roadmap" with `Phase`, `Area`, `Size`, `Epic` fields; every issue added

Re-running after editing `roadmap.json` adds new items and never duplicates existing ones (matched by title).
Renaming a task creates a new issue; close the old one by hand.

The HTML roadmap page renders this same JSON, so the page and the board cannot drift.

#!/usr/bin/env bash
# bootstrap.sh — turn roadmap.json into GitHub labels, milestones, issues and a Projects (v2) board.
# Idempotent: re-running updates labels, skips existing milestones/issues by title, and reuses the project.
#
# Requirements: gh (authenticated, with `project` scope: `gh auth refresh -s project,read:project`), jq.
# Usage:
#   ./bootstrap.sh                      # uses repo from roadmap.json
#   REPO=you/yourfork ./bootstrap.sh    # override
#   DRY_RUN=1 ./bootstrap.sh            # print what would happen
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
SPEC="${SPEC:-$HERE/roadmap.json}"
REPO="${REPO:-$(jq -r .project.repo "$SPEC")}"
OWNER="${REPO%%/*}"
PROJECT_TITLE="$(jq -r .project.name "$SPEC")"
DRY_RUN="${DRY_RUN:-0}"

run() { if [[ "$DRY_RUN" == "1" ]]; then echo "+ $*"; else "$@"; fi; }
log() { printf '\033[1;34m%s\033[0m %s\n' "$1" "${2:-}"; }

command -v gh >/dev/null || { echo "gh not found"; exit 1; }
command -v jq >/dev/null || { echo "jq not found"; exit 1; }

# ---------- labels ----------
log labels "syncing $(jq '.labels|length' "$SPEC") labels"
jq -c '.labels[]' "$SPEC" | while read -r l; do
  name=$(jq -r .name <<<"$l"); color=$(jq -r .color <<<"$l"); desc=$(jq -r .description <<<"$l")
  run gh label create "$name" --repo "$REPO" --color "$color" --description "$desc" --force >/dev/null
done

# ---------- milestones ----------
log milestones "syncing $(jq '.milestones|length' "$SPEC") milestones"
existing_ms=$(gh api "repos/$REPO/milestones?state=all&per_page=100" 2>/dev/null || echo '[]')
declare -A MS_NUM
jq -c '.milestones[]' "$SPEC" | while read -r m; do
  title=$(jq -r .title <<<"$m"); due=$(jq -r '.due // empty' <<<"$m"); desc=$(jq -r .description <<<"$m")
  num=$(jq -r --arg t "$title" '.[]|select(.title==$t)|.number' <<<"$existing_ms")
  if [[ -n "$num" ]]; then echo "  = $title (#$num)"; continue; fi
  args=(-f title="$title" -f description="$desc")
  [[ -n "$due" ]] && args+=(-f due_on="${due}T23:59:59Z")
  run gh api -X POST "repos/$REPO/milestones" "${args[@]}" >/dev/null && echo "  + $title"
done
# refresh map after creation
existing_ms=$(gh api "repos/$REPO/milestones?state=all&per_page=100" 2>/dev/null || echo '[]')
ms_number() { jq -r --arg t "$1" '.[]|select(.title==$t)|.number' <<<"$existing_ms"; }

# ---------- project (v2) ----------
log project "ensuring project '$PROJECT_TITLE' under $OWNER"
PROJECT_NUM=$(gh project list --owner "$OWNER" --format json 2>/dev/null | jq -r --arg t "$PROJECT_TITLE" '.projects[]|select(.title==$t)|.number' | head -n1 || true)
if [[ -z "${PROJECT_NUM:-}" ]]; then
  if [[ "$DRY_RUN" == "1" ]]; then echo "+ gh project create --owner $OWNER --title '$PROJECT_TITLE'"; PROJECT_NUM=0
  else PROJECT_NUM=$(gh project create --owner "$OWNER" --title "$PROJECT_TITLE" --format json | jq -r .number); echo "  + project #$PROJECT_NUM"; fi
else echo "  = project #$PROJECT_NUM"; fi

ensure_field() { # name type options(csv)
  local name="$1" type="$2" opts="${3:-}"
  local have
  have=$(gh project field-list "$PROJECT_NUM" --owner "$OWNER" --format json 2>/dev/null | jq -r --arg n "$name" '.fields[]|select(.name==$n)|.id' || true)
  if [[ -n "$have" ]]; then echo "  = field $name"; return; fi
  if [[ "$type" == "SINGLE_SELECT" ]]; then
    run gh project field-create "$PROJECT_NUM" --owner "$OWNER" --name "$name" --data-type SINGLE_SELECT --single-select-options "$opts" >/dev/null
  else
    run gh project field-create "$PROJECT_NUM" --owner "$OWNER" --name "$name" --data-type "$type" >/dev/null
  fi
  echo "  + field $name"
}
if [[ "$PROJECT_NUM" != "0" ]]; then
  ensure_field "Phase" SINGLE_SELECT "$(jq -r '[.milestones[].title]|join(",")' "$SPEC")"
  ensure_field "Area" SINGLE_SELECT "$(jq -r '[.labels[]|select(.name|startswith("area:"))|.name|ltrimstr("area:")]|join(",")' "$SPEC")"
  ensure_field "Size" SINGLE_SELECT "S,M,L,XL"
  ensure_field "Epic" TEXT
fi

# ---------- issues ----------
log issues "syncing epics and issues"
existing_issues=$(gh issue list --repo "$REPO" --state all --limit 1000 --json number,title 2>/dev/null || echo '[]')
issue_number() { jq -r --arg t "$1" '.[]|select(.title==$t)|.number' <<<"$existing_issues" | head -n1; }

create_issue() { # title body labels(csv) milestone -> prints number
  local title="$1" body="$2" labels="$3" ms="$4" n
  n=$(issue_number "$title")
  if [[ -n "$n" ]]; then echo "$n"; return; fi
  if [[ "$DRY_RUN" == "1" ]]; then echo "+ gh issue create '$title' [$labels] ms=$ms" >&2; echo 0; return; fi
  local args=(--repo "$REPO" --title "$title" --body "$body" --label "$labels")
  [[ -n "$ms" ]] && args+=(--milestone "$ms")
  gh issue create "${args[@]}" | sed -E 's#.*/([0-9]+)$#\1#'
}

add_to_project() { # issue_number
  [[ "$PROJECT_NUM" == "0" || "$1" == "0" ]] && return
  gh project item-add "$PROJECT_NUM" --owner "$OWNER" --url "https://github.com/$REPO/issues/$1" >/dev/null 2>&1 || true
}

jq -c '.epics[]' "$SPEC" | while read -r e; do
  ekey=$(jq -r .key <<<"$e"); etitle=$(jq -r .title <<<"$e"); ems=$(jq -r .milestone <<<"$e")
  elabels=$(jq -r '(["type:epic"]+.labels)|join(",")' <<<"$e")
  prd=$(jq -r '(.prd//[])|join(", ")' <<<"$e")
  # child issues first so the epic body can link them
  children=""
  while read -r i; do
    ititle="$ekey · $(jq -r .title <<<"$i")"
    ilabels=$(jq -r --argjson e "$e" '(.labels + $e.labels)|unique|join(",")' <<<"$i")
    ibody="$(jq -r .body <<<"$i")

---
Epic: $ekey — $etitle
Milestone: $ems"
    num=$(create_issue "$ititle" "$ibody" "$ilabels" "$ems")
    add_to_project "$num"
    children+="- [ ] #$num $(jq -r .title <<<"$i")"$'\n'
    echo "  · #$num $ititle"
  done < <(jq -c '.issues[]' <<<"$e")
  ebody="$(jq -r .body <<<"$e")

**PRD requirements:** ${prd:-—}

## Tasks
$children"
  enum=$(create_issue "$ekey: $etitle" "$ebody" "$elabels" "$ems")
  add_to_project "$enum"
  echo "  ★ #$enum $ekey: $etitle"
done

log done "labels, milestones, $(jq '[.epics[].issues[]]|length' "$SPEC") issues, $(jq '.epics|length' "$SPEC") epics → https://github.com/$REPO"
[[ "$PROJECT_NUM" != "0" ]] && echo "project: https://github.com/orgs/$OWNER/projects/$PROJECT_NUM (or /users/$OWNER/projects/$PROJECT_NUM)"
echo "Tip: set the Phase/Area/Size fields in bulk from the project's table view by grouping on labels; item field automation is left to the UI to keep this script simple."

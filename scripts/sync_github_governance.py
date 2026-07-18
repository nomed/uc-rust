#!/usr/bin/env python3
"""Synchronize GitHub repository metadata and Project v2 from a manifest.

The script has three modes:
- validate: validate only the manifest structure;
- dry-run: report drift without changing GitHub;
- apply: reconcile and delete unmanaged metadata.

Only Python's standard library and the GitHub CLI are required.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

API = "https://api.github.com"
MANIFEST_PATH = Path("governance/github-manifest.json")
APPLY_CONFIRMATION = "DELETE_UNMANAGED_GITHUB_METADATA"


class GovernanceError(RuntimeError):
    pass


def load_manifest() -> dict[str, Any]:
    try:
        manifest = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise GovernanceError(f"Cannot load {MANIFEST_PATH}: {exc}") from exc
    validate_manifest(manifest)
    return manifest


def validate_manifest(manifest: dict[str, Any]) -> None:
    required = {"schema_version", "repository", "cleanup", "labels", "milestones", "project", "issues"}
    missing = required - manifest.keys()
    if missing:
        raise GovernanceError(f"Manifest missing keys: {sorted(missing)}")
    if manifest["schema_version"] != 1:
        raise GovernanceError("Unsupported schema_version")

    label_names = [item["name"] for item in manifest["labels"]]
    if len(label_names) != len(set(label_names)):
        raise GovernanceError("Duplicate label names")

    milestone_titles = [item["title"] for item in manifest["milestones"]]
    if len(milestone_titles) != len(set(milestone_titles)):
        raise GovernanceError("Duplicate milestone titles")

    fields = manifest["project"]["fields"]
    field_names = [item["name"] for item in fields]
    if len(field_names) != len(set(field_names)):
        raise GovernanceError("Duplicate project field names")

    known_labels = set(label_names)
    known_milestones = set(milestone_titles)
    field_options = {
        item["name"]: set(item.get("options", []))
        for item in fields
        if item["type"] == "SINGLE_SELECT"
    }
    field_options["Status"] = set(manifest["project"].get("status_options", []))

    for issue_number, issue in manifest["issues"].items():
        if not issue_number.isdigit():
            raise GovernanceError(f"Issue key is not numeric: {issue_number}")
        unknown_labels = set(issue["labels"]) - known_labels
        if unknown_labels:
            raise GovernanceError(f"Issue #{issue_number} uses undefined labels: {sorted(unknown_labels)}")
        milestone = issue.get("milestone")
        if milestone is not None and milestone not in known_milestones:
            raise GovernanceError(f"Issue #{issue_number} uses undefined milestone: {milestone}")
        for field_name, value in issue.get("project", {}).items():
            if field_name not in field_options:
                raise GovernanceError(f"Issue #{issue_number} uses undefined project field: {field_name}")
            if value not in field_options[field_name]:
                raise GovernanceError(
                    f"Issue #{issue_number} uses invalid {field_name} value {value!r}"
                )


def log(action: str) -> None:
    print(action, flush=True)


class GitHubRest:
    def __init__(self, token: str, repository: str, apply: bool) -> None:
        if not token:
            raise GovernanceError("REPO_TOKEN is required")
        self.token = token
        self.repository = repository
        self.apply = apply

    def request(self, method: str, path: str, payload: Any | None = None) -> Any:
        body = None if payload is None else json.dumps(payload).encode("utf-8")
        request = urllib.request.Request(
            f"{API}{path}",
            data=body,
            method=method,
            headers={
                "Accept": "application/vnd.github+json",
                "Authorization": f"Bearer {self.token}",
                "X-GitHub-Api-Version": "2022-11-28",
                "Content-Type": "application/json",
                "User-Agent": "uc-rust-governance-sync",
            },
        )
        try:
            with urllib.request.urlopen(request) as response:
                content = response.read()
                return json.loads(content) if content else None
        except urllib.error.HTTPError as exc:
            detail = exc.read().decode("utf-8", errors="replace")
            raise GovernanceError(f"GitHub API {method} {path} failed: {exc.code} {detail}") from exc

    def paginate(self, path: str) -> list[dict[str, Any]]:
        separator = "&" if "?" in path else "?"
        page = 1
        result: list[dict[str, Any]] = []
        while True:
            batch = self.request("GET", f"{path}{separator}per_page=100&page={page}")
            result.extend(batch)
            if len(batch) < 100:
                return result
            page += 1

    def change(self, description: str, method: str, path: str, payload: Any | None = None) -> None:
        prefix = "APPLY" if self.apply else "DRIFT"
        log(f"[{prefix}] {description}")
        if self.apply:
            self.request(method, path, payload)


def sync_labels(rest: GitHubRest, manifest: dict[str, Any]) -> None:
    repo = rest.repository
    desired = {item["name"]: item for item in manifest["labels"]}
    existing = {item["name"]: item for item in rest.paginate(f"/repos/{repo}/labels")}

    for name, definition in desired.items():
        payload = {
            "new_name": name,
            "color": definition["color"],
            "description": definition.get("description", ""),
        }
        current = existing.get(name)
        if current is None:
            rest.change(
                f"create label {name}",
                "POST",
                f"/repos/{repo}/labels",
                {"name": name, "color": definition["color"], "description": definition.get("description", "")},
            )
        elif current.get("color", "").lower() != definition["color"].lower() or current.get("description") != definition.get("description", ""):
            rest.change(
                f"update label {name}",
                "PATCH",
                f"/repos/{repo}/labels/{urllib.parse.quote(name, safe='')}",
                payload,
            )

    if manifest["cleanup"].get("delete_undefined_labels", False):
        for name in sorted(set(existing) - set(desired)):
            rest.change(
                f"delete unmanaged label {name}",
                "DELETE",
                f"/repos/{repo}/labels/{urllib.parse.quote(name, safe='')}",
            )


def sync_milestones(rest: GitHubRest, manifest: dict[str, Any]) -> dict[str, int]:
    repo = rest.repository
    desired = {item["title"]: item for item in manifest["milestones"]}
    existing_items = rest.paginate(f"/repos/{repo}/milestones?state=all")
    existing = {item["title"]: item for item in existing_items}

    for title, definition in desired.items():
        payload = {
            "title": title,
            "description": definition.get("description", ""),
            "state": definition.get("state", "open"),
        }
        current = existing.get(title)
        if current is None:
            rest.change(f"create milestone {title}", "POST", f"/repos/{repo}/milestones", payload)
        elif current.get("description") != payload["description"] or current.get("state") != payload["state"]:
            rest.change(
                f"update milestone {title}",
                "PATCH",
                f"/repos/{repo}/milestones/{current['number']}",
                payload,
            )

    if manifest["cleanup"].get("delete_undefined_milestones", False):
        for title in sorted(set(existing) - set(desired)):
            rest.change(
                f"delete unmanaged milestone {title}",
                "DELETE",
                f"/repos/{repo}/milestones/{existing[title]['number']}",
            )

    if rest.apply:
        existing_items = rest.paginate(f"/repos/{repo}/milestones?state=all")
        existing = {item["title"]: item for item in existing_items}
    return {title: item["number"] for title, item in existing.items() if title in desired}


def sync_issues(rest: GitHubRest, manifest: dict[str, Any], milestones: dict[str, int]) -> None:
    repo = rest.repository
    for issue_number, definition in manifest["issues"].items():
        issue = rest.request("GET", f"/repos/{repo}/issues/{issue_number}")
        if "pull_request" in issue:
            raise GovernanceError(f"Manifest item #{issue_number} is a pull request, not an issue")

        current_labels = sorted(label["name"] for label in issue.get("labels", []))
        desired_labels = sorted(definition["labels"])
        if current_labels != desired_labels:
            rest.change(
                f"set exact labels on issue #{issue_number}: {desired_labels}",
                "PUT",
                f"/repos/{repo}/issues/{issue_number}/labels",
                {"labels": desired_labels},
            )

        milestone_title = definition.get("milestone")
        desired_number = None if milestone_title is None else milestones.get(milestone_title)
        current_number = issue.get("milestone", {}).get("number") if issue.get("milestone") else None
        if milestone_title is not None and desired_number is None and not rest.apply:
            log(f"[DRIFT] issue #{issue_number} will use newly created milestone {milestone_title}")
        elif current_number != desired_number:
            rest.change(
                f"set milestone on issue #{issue_number}: {milestone_title or 'none'}",
                "PATCH",
                f"/repos/{repo}/issues/{issue_number}",
                {"milestone": desired_number},
            )


def gh_json(args: list[str], project_token: str) -> Any:
    env = os.environ.copy()
    env["GH_TOKEN"] = project_token
    completed = subprocess.run(
        ["gh", *args, "--format", "json"],
        check=True,
        capture_output=True,
        text=True,
        env=env,
    )
    return json.loads(completed.stdout)


def gh_run(args: list[str], project_token: str, apply: bool, description: str) -> Any | None:
    prefix = "APPLY" if apply else "DRIFT"
    log(f"[{prefix}] {description}")
    if not apply:
        return None
    return gh_json(args, project_token)


def unwrap(payload: Any, key: str) -> list[dict[str, Any]]:
    if isinstance(payload, list):
        return payload
    if isinstance(payload, dict) and isinstance(payload.get(key), list):
        return payload[key]
    raise GovernanceError(f"Unexpected gh JSON shape; expected list or {key!r}")


def field_options(field: dict[str, Any]) -> dict[str, str]:
    options = field.get("options") or []
    return {option["name"]: option["id"] for option in options}


def sync_project(manifest: dict[str, Any], project_token: str, apply: bool) -> None:
    if not project_token:
        raise GovernanceError(
            "PROJECT_TOKEN is required for Project #4. Use a token with project scope."
        )

    project = manifest["project"]
    owner = project["owner"]
    number = str(project["number"])

    view = gh_json(["project", "view", number, "--owner", owner], project_token)
    project_id = view["id"]

    gh_run(
        [
            "project", "edit", number, "--owner", owner,
            "--title", project["title"],
            "--description", project["description"],
            "--readme", project["readme"],
        ],
        project_token,
        apply,
        "align project title, description and readme",
    )

    desired_fields = list(project["fields"])
    # GitHub's built-in Status options are not safely replaceable via CLI. We use a
    # managed custom Stage field and retain the built-in Status as a protected field.
    desired_fields.append(
        {"name": "Stage", "type": "SINGLE_SELECT", "options": project["status_options"]}
    )

    fields_payload = gh_json(["project", "field-list", number, "--owner", owner, "--limit", "100"], project_token)
    existing_fields = unwrap(fields_payload, "fields")
    existing_by_name = {field["name"]: field for field in existing_fields}
    protected = set(project.get("protected_fields", []))
    desired_names = {field["name"] for field in desired_fields}

    for definition in desired_fields:
        name = definition["name"]
        current = existing_by_name.get(name)
        desired_options = definition.get("options", [])
        recreate = False
        if current is not None:
            current_type = str(current.get("type", "")).upper()
            if current_type != definition["type"].upper():
                recreate = True
            elif definition["type"] == "SINGLE_SELECT" and set(field_options(current)) != set(desired_options):
                recreate = True
        if recreate:
            gh_run(
                ["project", "field-delete", "--id", current["id"]],
                project_token,
                apply,
                f"delete drifted project field {name}",
            )
            current = None
        if current is None:
            args = [
                "project", "field-create", number, "--owner", owner,
                "--name", name, "--data-type", definition["type"],
            ]
            if desired_options:
                args.extend(["--single-select-options", ",".join(desired_options)])
            gh_run(args, project_token, apply, f"create project field {name}")

    if manifest["cleanup"].get("delete_undefined_project_fields", False):
        for field in existing_fields:
            name = field["name"]
            if name not in desired_names and name not in protected:
                gh_run(
                    ["project", "field-delete", "--id", field["id"]],
                    project_token,
                    apply,
                    f"delete unmanaged project field {name}",
                )

    if apply:
        fields_payload = gh_json(["project", "field-list", number, "--owner", owner, "--limit", "100"], project_token)
        existing_fields = unwrap(fields_payload, "fields")
        existing_by_name = {field["name"]: field for field in existing_fields}

    items_payload = gh_json(["project", "item-list", number, "--owner", owner, "--limit", "500"], project_token)
    items = unwrap(items_payload, "items")
    issue_items: dict[int, dict[str, Any]] = {}
    for item in items:
        content = item.get("content") or {}
        if content.get("type") == "Issue" and content.get("repository") == manifest["repository"]:
            issue_items[int(content["number"])] = item

    managed_numbers = {int(number_) for number_ in manifest["issues"]}
    if manifest["cleanup"].get("delete_unmanaged_project_items", False):
        for item in items:
            content = item.get("content") or {}
            is_managed_issue = (
                content.get("type") == "Issue"
                and content.get("repository") == manifest["repository"]
                and int(content.get("number", -1)) in managed_numbers
            )
            if not is_managed_issue:
                gh_run(
                    ["project", "item-delete", number, "--owner", owner, "--id", item["id"]],
                    project_token,
                    apply,
                    f"delete unmanaged project item {item.get('title', item['id'])}",
                )

    for issue_number in sorted(managed_numbers):
        if issue_number not in issue_items:
            url = f"https://github.com/{manifest['repository']}/issues/{issue_number}"
            gh_run(
                ["project", "item-add", number, "--owner", owner, "--url", url],
                project_token,
                apply,
                f"add issue #{issue_number} to project",
            )

    if apply:
        items_payload = gh_json(["project", "item-list", number, "--owner", owner, "--limit", "500"], project_token)
        items = unwrap(items_payload, "items")
        issue_items = {}
        for item in items:
            content = item.get("content") or {}
            if content.get("type") == "Issue" and content.get("repository") == manifest["repository"]:
                issue_items[int(content["number"])] = item

    for issue_number_text, issue_definition in manifest["issues"].items():
        issue_number = int(issue_number_text)
        item = issue_items.get(issue_number)
        if item is None:
            if not apply:
                log(f"[DRIFT] project field values will be set after adding issue #{issue_number}")
            continue
        for manifest_field_name, value in issue_definition.get("project", {}).items():
            actual_field_name = "Stage" if manifest_field_name == "Status" else manifest_field_name
            field = existing_by_name.get(actual_field_name)
            if field is None:
                if not apply:
                    log(f"[DRIFT] set {actual_field_name}={value} on issue #{issue_number} after field creation")
                    continue
                raise GovernanceError(f"Missing project field after reconciliation: {actual_field_name}")
            option_id = field_options(field).get(value)
            if option_id is None:
                raise GovernanceError(f"Missing option {value!r} in project field {actual_field_name}")
            gh_run(
                [
                    "project", "item-edit",
                    "--id", item["id"],
                    "--project-id", project_id,
                    "--field-id", field["id"],
                    "--single-select-option-id", option_id,
                ],
                project_token,
                apply,
                f"set {actual_field_name}={value} on issue #{issue_number}",
            )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("validate", "dry-run", "apply"), default="validate")
    parser.add_argument("--confirm", default="")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        manifest = load_manifest()
        log("Manifest validation passed")
        if args.mode == "validate":
            return 0
        if args.mode == "apply" and args.confirm != APPLY_CONFIRMATION:
            raise GovernanceError(
                f"Apply mode requires --confirm {APPLY_CONFIRMATION!r}"
            )

        apply = args.mode == "apply"
        rest = GitHubRest(
            token=os.environ.get("REPO_TOKEN", ""),
            repository=manifest["repository"],
            apply=apply,
        )
        sync_labels(rest, manifest)
        milestones = sync_milestones(rest, manifest)
        sync_issues(rest, manifest, milestones)
        sync_project(manifest, os.environ.get("PROJECT_TOKEN", ""), apply)
        log("Governance synchronization completed")
        return 0
    except (GovernanceError, subprocess.CalledProcessError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        if isinstance(exc, subprocess.CalledProcessError) and exc.stderr:
            print(exc.stderr, file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())

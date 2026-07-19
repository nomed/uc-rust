#!/usr/bin/env python3
"""Synchronize native GitHub issue relationships and Project roadmap dates."""
from __future__ import annotations
import json, os, subprocess, urllib.error, urllib.request
from pathlib import Path

API="https://api.github.com"
STRUCTURE=Path("governance/github-structure.json")
MANIFEST=Path("governance/github-manifest.json")

class SyncError(RuntimeError): pass

def request(token: str, method: str, path: str, payload=None, tolerate=()):
    body=None if payload is None else json.dumps(payload).encode()
    req=urllib.request.Request(API+path,data=body,method=method,headers={
        "Accept":"application/vnd.github+json","Authorization":f"Bearer {token}",
        "X-GitHub-Api-Version":"2026-03-10","Content-Type":"application/json",
        "User-Agent":"uc-rust-structure-sync"})
    try:
        with urllib.request.urlopen(req) as response:
            raw=response.read(); return json.loads(raw) if raw else None
    except urllib.error.HTTPError as exc:
        if exc.code in tolerate: return None
        raise SyncError(f"{method} {path}: {exc.code} {exc.read().decode(errors='replace')}") from exc

def gh(token: str, *args: str):
    env=os.environ.copy(); env["GH_TOKEN"]=token
    result=subprocess.run(["gh",*args,"--format","json"],env=env,text=True,capture_output=True)
    if result.returncode: raise SyncError(result.stderr)
    return json.loads(result.stdout)

def owner_candidates(owner: str):
    candidates=[["--owner",owner]]
    if owner!="@me":
        candidates.append(["--owner","@me"])
    candidates.append([])
    return candidates

def resolve_project_owner_args(project_token: str, number: str, owner: str):
    failures=[]
    for owner_args in owner_candidates(owner):
        try:
            view=gh(project_token,"project","view",number,*owner_args)
            if owner_args!=["--owner",owner]:
                if owner_args:
                    print(f"owner override {' '.join(owner_args)} succeeded for Project #{number}",flush=True)
                else:
                    print(f"owner flag unsupported for Project #{number}; continuing without --owner",flush=True)
            return view,owner_args
        except SyncError as exc:
            failures.append(exc)
    raise failures[0]

def values(payload, key):
    return payload if isinstance(payload,list) else payload.get(key,[])

def sync_relations(repo_token: str, structure: dict):
    repo=structure["repository"]
    numbers={int(n) for n in structure["parents"]}|{int(n) for n in structure["blocked_by"]}
    numbers|={int(v) for v in structure["parents"].values()}
    numbers|={int(v) for xs in structure["blocked_by"].values() for v in xs}
    issues={n:request(repo_token,"GET",f"/repos/{repo}/issues/{n}") for n in sorted(numbers)}
    for child_text,parent in structure["parents"].items():
        child=int(child_text)
        current=request(repo_token,"GET",f"/repos/{repo}/issues/{parent}/sub_issues?per_page=100") or []
        if child not in {item["number"] for item in current}:
            print(f"add #{child} as sub-issue of #{parent}")
            request(repo_token,"POST",f"/repos/{repo}/issues/{parent}/sub_issues",{"sub_issue_id":issues[child]["id"],"replace_parent":True})
    for issue_text, blockers in structure["blocked_by"].items():
        issue=int(issue_text)
        current=request(repo_token,"GET",f"/repos/{repo}/issues/{issue}/dependencies/blocked_by?per_page=100") or []
        current_numbers={item["number"] for item in current}
        for blocker in blockers:
            if blocker not in current_numbers:
                print(f"add blocked-by #{blocker} to #{issue}")
                request(repo_token,"POST",f"/repos/{repo}/issues/{issue}/dependencies/blocked_by",{"issue_id":issues[blocker]["id"]},tolerate=(422,))

def ensure_date_field(project_token, number, owner_args, name):
    fields=values(gh(project_token,"project","field-list",number,*owner_args,"--limit","100"),"fields")
    found=next((f for f in fields if f["name"]==name),None)
    if found: return found
    return gh(project_token,"project","field-create",number,*owner_args,"--name",name,"--data-type","DATE")

def sync_project(project_token: str, structure: dict, manifest: dict):
    project=structure["project"]
    number=str(project["number"])
    view,owner_args=resolve_project_owner_args(project_token,number,project["owner"])
    start=ensure_date_field(project_token,number,owner_args,"Start date")
    target=ensure_date_field(project_token,number,owner_args,"Target date")
    items=values(gh(project_token,"project","item-list",number,*owner_args,"--limit","500"),"items")
    by_number={int(i["content"]["number"]):i for i in items if (i.get("content") or {}).get("type")=="Issue"}
    releases=structure["roadmap"]["releases"]
    for number_text,definition in manifest["issues"].items():
        number=int(number_text); item=by_number.get(number)
        if not item: continue
        release=definition["project"]["Release"]
        dates=releases[release]
        for field,value in ((start,dates["start"]),(target,dates["target"])):
            gh(project_token,"project","item-edit","--id",item["id"],"--project-id",view["id"],"--field-id",field["id"],"--date",value)
    # The user-owned Project Views API currently requires a classic user token.
    request(project_token,"POST",f"/users/{project['owner_id']}/projectsV2/{project['number']}/views",{
        "name":structure["roadmap"]["view_name"],"layout":"roadmap","filter":"is:issue"},tolerate=(304,422))

def main():
    structure=json.loads(STRUCTURE.read_text())
    manifest=json.loads(MANIFEST.read_text())
    repo_token=os.environ.get("REPO_TOKEN","")
    project_token=os.environ.get("PROJECT_TOKEN","")
    if not repo_token: raise SyncError("REPO_TOKEN is required")
    sync_relations(repo_token,structure)
    if project_token: sync_project(project_token,structure,manifest)
    else: print("PROJECT_TOKEN missing: relationships applied, roadmap fields skipped")

if __name__=="__main__":
    try: main()
    except (SyncError,subprocess.CalledProcessError) as exc:
        print(f"ERROR: {exc}"); raise SystemExit(1)

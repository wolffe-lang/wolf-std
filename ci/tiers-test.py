#!/usr/bin/env python3
"""ci/tiers-test.py — the planner's own proof (tl28; wolf-lang's
ci/tiers-test.py, tl21, cut to wolf-std). python3, git and bash only.

    ci/tiers-test.py

  T1  the tiers by event: the nightly and a dispatch with tier all are
      all, every job on every leg; a trunk push, a dispatch (tier full, or
      no tier) and the `full-matrix` label are full, every job on the
      merge's legs (tl30); another label's `labeled` event is a noop; a
      dispatch's tier must be full, all or pr.
  T2  the pull-request tiers by path: each path names the jobs THIS
      repository's block below says, on the pull request's hosts; prose
      alone is noop; prose beside code is that code's tier.
  T3  full wins: a path no rule names, alone or beside anything; changed
      paths that cannot be read; a change that names no path; a dispatched
      branch the same.
  T4  the wiring of .github/workflows/ci.yml: the plan is first and the
      verdict last but the nightly's issue; every job needs the plan, runs
      only when the plan names it and takes its hosts from the plan; every
      plan output is read with a default; the `tier` job needs every job,
      runs on `!cancelled()` and checks with `ci/tiers.py verdict`;
      cancel-in-progress is for pull requests only and every other run has a
      concurrency group of its own run id; the trunk push (and no other
      branch's), the nightly, the label and the dispatch's `tier` input are
      there; the all tier's legs are the ones the jobs ran on before tl28,
      the full tier's are those but the ones tl30 moved to the nightly,
      and the full tier keeps a macOS leg (the smoke).
  T5  the tripwire: no line of this repository's code that reads a file
      names a path the planner calls prose (THIS repository's block). The
      search is shown to fire on paths it must find. (A WARN, not a
      failure: a tracked path no rule names; its pull request runs full.)
  T6  the verdict: green for an all, a full, a pr and a noop run; red for
      an all run on the merge's legs, a full run on every leg (the merge
      before tl30), a chosen
      job failed, skipped or cancelled, a job that ran unasked, a failed
      plan, a plan with no tier, a full run that chose less, a full run on
      the pull request's hosts, and green when GitHub drops an output that
      would have been empty.
  T7  the change read from git: a real merge commit against its first
      parent, with the base moved on meanwhile; red-to-full when HEAD is no
      merge or its second parent is not the pull request's head; a
      dispatched branch against its merge base with origin/trunk, and full
      when there is no origin/trunk to read.
  T8  ci/nightly-red.sh against a stand-in `gh`: no open issue opens one,
      labelled, under the one title; an open one gets a comment and no new
      issue; a label that already exists is no failure.
  T11 a nightly at a commit already green in full: told by ci/covered.py
      that the commit's latest `tier all` is green, the nightly plans
      `tier skipped`, no job, and says so in plain words in its log, a
      warning and the run's summary, naming the covering run and when a
      full night is due (S1); told anything else, it plans all (S2); no
      other event is ever skipped, and no dispatch may ask for it (S3); the
      verdict of a skipped night wants every job skipped and says nothing
      was tested (S4). ci/covered.py, against a fake `gh`, covers only a
      green `tier all` check run of GitHub Actions, of this workflow, at
      this commit, of a run other than tonight's, the latest to complete,
      at most 144 hours before tonight's run; never another tier's, an
      older one, a red, cancelled or running one, another app's, a full
      page, an unanswered question or another event (S5). The plan job
      asks it before it plans, with read-only permissions, and only the
      plan step reads the answer (S6). The nightly's issue says a
      `tier skipped` night is not a green one (S7).

Exit 0 when every check passes, 1 otherwise.
"""
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PLAN = os.path.join(ROOT, "ci", "tiers.py")
RED = os.path.join(ROOT, "ci", "nightly-red.sh")
ENV_KEYS = ("EVENT", "ACTION", "LABEL", "LABELS", "HEAD_SHA", "IN_TIER", "NEEDS",
            "COVERED", "COVERED_BY", "COVERED_AT", "COVERED_RUN", "GITHUB_STEP_SUMMARY")
fails = 0


def say(ok, what):
    global fails
    print("tiers-test: %s %s" % ("PASS" if ok else "FAIL", what))
    fails += not ok


# --- this repository's -------------------------------------------------------------
NIGHTLY = None            # (its tier, its jobs); None: the all tier
FULL_LEG_TIERS = ("all",)  # the tiers that run every leg
WORKFLOW_FILE = "ci.yml"
WORKFLOW_NAME = "ci"
WORKFLOW_JOBS = ["plan", "rig", "gates", "tier", "nightly-red"]
FULL_JOBS = ["rig", "gates"]
PUSH_BRANCHES = "trunk"
OTHER_HOST_JOBS = ()
# The hosts before tl28 (.github/workflows/ci.yml at trunk cceda63).
_H = [{"os": "ubuntu-24.04"}, {"os": "macos-latest"}, {"os": "windows-latest"}]
BEFORE_HOSTS = {"matrix_rig": _H, "matrix_gates": _H}
ALL_HOSTS = BEFORE_HOSTS
PR_HOSTS = {"matrix_rig": _H[:1], "matrix_gates": _H[:1]}
# The merge's legs (tl30, the macOS cap): the macOS rig (std-test, 34 of its
# 35 minutes) is the nightly's; gates on macOS is the merge's smoke.
MERGE_HOSTS = {"matrix_rig": [_H[0], _H[2]], "matrix_gates": _H}
MOVED = {"matrix_rig": [_H[1]], "matrix_gates": []}
BOTH = ["rig", "gates"]
CODE_PATH = ("std/fmt/fmt.lu", BOTH)
PR_PATHS = [(p, BOTH) for p in [
    "std/fmt/fmt.lu", "std/net/net.lu", "std/prelude/prelude.lu", "tests/fs/read.lu", "tests/ledger.toml",
    "tests/ulp/reference.txt", "vendor/tools.toml", "vendor/upstream/PIN", "vendor/upstream/anchors.json",
    "vendor/vectors/README.md", "upstream", "xtask/src/main.rs", "xtask/Cargo.toml", "Cargo.toml", "Cargo.lock",
    "rust-toolchain.toml", ".cargo/config.toml", ".fac/workspace.json", "assets/logo.svg",
    ".github/workflows/ci.yml", ".github/workflows/nightly.yml", "ci/tiers.py", "ci/tiers-test.py",
    "ci/nightly-red.sh", "README.md", "API-CONVENTIONS.md", "GUIDE-ENTRIES-sc40.txt", "LICENSE",
    "LICENSE-EXCEPTION", "LICENSE-TRAINING-DATA", ".gitmodules", ".gitignore", ".gitattributes",
    # Under the library, its tests or its pins: never prose, whatever the name.
    "std/README.md", "tests/README.md", "vendor/upstream/spec/05-conformance.md"]]
PROSE_PATHS = ["CHANGELOG.md", "docs/findings.md", "CONTRIBUTING.md", "docs/error-taxonomy.md",
               "docs/sc64-witness/jobs.lu", "docs/sc62-witness/pty.py", "docs/a-new-page.md"]
PR_UNIONS = []
# Stricter than a read: any line of xtask/ or tests/ with a string literal
# that names a prose root trips (a path is built from literals before it is
# read).
READS = re.compile(r'"')


def is_code(f):
    """The code that could read a prose path: the rig (xtask/) and the
    library's tests (tests/), Rust and wolf."""
    return f.startswith(("xtask/", "tests/")) and f.endswith((".rs", ".lu"))


MUST_FIND = ["vendor/upstream/anchors.json", "tests/ledger.toml", "tests/ulp/reference.txt"]
RULED = {}  # {prose root: {regex of a line that names something else: why}}


def extra_t5(files, cls, say):
    roots = re.search(r'^const ROOTS: \[&str; \d+\] = \[(.*)\];$', open(os.path.join(ROOT, "xtask/src/fmt_lu.rs"), encoding="utf-8").read(), re.M)
    say(roots is not None and re.findall(r'"([^"]+)"', roots.group(1)) == ["std", "tests"],
        "T5 fmt-lu walks std/ and tests/ alone (%s)" % (roots.group(1) if roots else "ROOTS not found"))
    lib = [p for p in files if p.startswith(("std/", "tests/", "vendor/", "xtask/"))]
    say(len(lib) >= 100 and all(cls.get(p) == "rig gates" for p in lib),
        "T5 every tracked path under std/, tests/, vendor/ and xtask/ is pr, both jobs (%d paths)" % len(lib))


def extra_checks(say, plan, expect, blocks, wf):
    # xtask's selftest diffs ci.yml's cargo lines against workflow::CI_STEPS
    # and blesses every continue-on-error (sc41, sc47): tl28 adds neither.
    say(not re.search(r"^\s*continue-on-error:", wf, re.M), "T4 no continue-on-error (xtask's selftest blesses each one; tl28 adds none)")
    steps = re.findall(r'^\s+command: "([^"]+)",$', open(os.path.join(ROOT, "xtask/src/workflow.rs"), encoding="utf-8").read(), re.M)
    have = set()
    for line in wf.splitlines():
        t = line.strip()
        if t.startswith("#"):
            continue
        t = t[2:].strip() if t.startswith("- ") else t
        t = t[4:].strip() if t.startswith("run:") else t
        if t.startswith("cargo "):
            have.add(" ".join(t.split()))
    say(len(steps) >= 10 and have == set(steps), "T4 the workflow's cargo lines are workflow::CI_STEPS exactly (%d), as xtask's selftest wants" % len(steps))
# --- end of this repository's ------------------------------------------------------
WORKFLOW = os.path.join(ROOT, ".github", "workflows", WORKFLOW_FILE)
NIGHTLY = NIGHTLY or ("all", FULL_JOBS)


def clean_env(**more):
    env = {k: v for k, v in os.environ.items() if k not in ENV_KEYS}
    env.update(more)
    return env


def plan(paths=None, **kw):
    args = [sys.executable, PLAN, "plan", "--json"]
    flags = kw.pop("flags", [])
    for k, v in kw.items():
        args += ["--" + k.replace("_", "-"), v]
    args += flags
    if paths is not None:
        args += ["--paths-from", "-"]
    r = subprocess.run(args, input="\n".join(paths or []) + "\n", capture_output=True, text=True, env=clean_env())
    if r.returncode != 0:
        return {"tier": "ERROR rc=%d %s" % (r.returncode, r.stderr.strip()[-200:]), "jobs": [], "matrix": {}}
    return json.loads(r.stdout)


wf = open(WORKFLOW, encoding="utf-8").read()
JOBS = re.findall(r"^  ([A-Za-z0-9_-]+):\s*$", wf[wf.index("\njobs:\n"):], re.M)
EVERY = [j for j in JOBS if j not in ("plan", "tier", "nightly-red")]


def expect(name, got, tier, jobs):
    why = []
    if got["tier"] != tier:
        why.append("tier %s, wanted %s" % (got["tier"], tier))
    if got["jobs"] != jobs:
        why.append("jobs %s, wanted %s" % (" ".join(got["jobs"]) or "none", " ".join(jobs) or "none"))
    hosts = ALL_HOSTS if tier in FULL_LEG_TIERS else MERGE_HOSTS if tier == "full" else PR_HOSTS
    if not got["tier"].startswith("ERROR") and got.get("matrix") != hosts:
        why.append("hosts %s, wanted %s" % (json.dumps(got.get("matrix")), json.dumps(hosts)))
    if tier == "all":
        shown = "every job (%d), every leg" % len(jobs)
    elif tier == "full":
        shown = "every job (%d), the merge's legs" % len(jobs)
    else:
        shown = (" ".join(jobs) or "no job") + (", every leg" if tier in FULL_LEG_TIERS else "")
    say(not why, "%s: %s%s" % (name, tier, (" (" + "; ".join(why) + ")") if why else ", " + shown))


def in_order(jobs):
    return [j for j in EVERY if j in jobs]


# --- T1 the tiers by event ---------------------------------------------------
say(JOBS == WORKFLOW_JOBS, "T1 the workflow's jobs: %s" % " ".join(JOBS))
expect("T1 a push to trunk", plan(event="push"), "full", FULL_JOBS)
expect("T1 the nightly", plan(event="schedule"), NIGHTLY[0], NIGHTLY[1])
expect("T1 a dispatch, no tier", plan(event="workflow_dispatch"), "full", FULL_JOBS)
expect("T1 a dispatch, tier full", plan(event="workflow_dispatch", tier="full"), "full", FULL_JOBS)
expect("T1 a dispatch, tier all", plan(event="workflow_dispatch", tier="all"), "all", FULL_JOBS)
say(plan(event="workflow_dispatch", tier="partial")["tier"].startswith("ERROR rc=2"), "T1 a dispatch's tier is full, all or pr, nothing else")
expect("T1 a pull request labelled full-matrix, prose only",
       plan([PROSE_PATHS[0]], event="pull_request", action="synchronize", labels="bug,full-matrix"), "full", FULL_JOBS)
expect("T1 the full-matrix label added",
       plan([PROSE_PATHS[0]], event="pull_request", action="labeled", label="full-matrix", labels="full-matrix"), "full", FULL_JOBS)
code, code_jobs = CODE_PATH
expect("T1 another label added", plan([code], event="pull_request", action="labeled", label="bug", labels="bug"), "noop", [])
expect("T1 another label added to a labelled pull request",
       plan([code], event="pull_request", action="labeled", label="bug", labels="bug,full-matrix"), "noop", [])
expect("T1 the tier input means nothing to a pull request",
       plan([code], event="pull_request", action="synchronize", tier="full"), "pr", in_order(code_jobs))

# --- T2 the pull-request tiers by path ------------------------------------------
pr = dict(event="pull_request", action="synchronize")
for p, jobs in PR_PATHS:
    expect("T2 %s" % p, plan([p], **pr), "pr", in_order(jobs))
for p in PROSE_PATHS:
    expect("T2 prose: %s" % p, plan([p], **pr), "noop", [])
expect("T2 prose alone, several", plan(PROSE_PATHS[:3], **pr), "noop", [])
expect("T2 prose beside code", plan([PROSE_PATHS[0], code, PROSE_PATHS[-1]], **pr), "pr", in_order(code_jobs))
for paths, jobs in PR_UNIONS:
    expect("T2 %s together" % " + ".join(paths), plan(paths, **pr), "pr", in_order(jobs))

# --- T3 full wins ---------------------------------------------------------------
expect("T3 a path no rule names", plan(["newplace/thing.rs"], **pr), "full", FULL_JOBS)
expect("T3 a new file at the root", plan(["build.rs"], **pr), "full", FULL_JOBS)
expect("T3 a markdown file at the root no rule names", plan(["NOTES.md"], **pr), "full", FULL_JOBS)
expect("T3 an unknown path beside prose", plan([PROSE_PATHS[0], "newplace/notes.md"], **pr), "full", FULL_JOBS)
expect("T3 an unknown path beside code", plan([code, "newplace/thing.rs"], **pr), "full", FULL_JOBS)
expect("T3 no changed paths to read", plan(**pr), "full", FULL_JOBS)
expect("T3 a change that names no path", plan([], **pr), "full", FULL_JOBS)
expect("T3 a merge that cannot be read (no repository there)",
       plan(repo=os.path.join(ROOT, "no", "such", "dir"), flags=["--merge"], **pr), "full", FULL_JOBS)
expect("T3 a dispatch, tier pr, an unknown path", plan(["newplace/thing.rs"], event="workflow_dispatch", tier="pr"), "full", FULL_JOBS)
expect("T3 a dispatch, tier pr, a branch that cannot be read",
       plan(repo=os.path.join(ROOT, "no", "such", "dir"), flags=["--merge"], event="workflow_dispatch", tier="pr"), "full", FULL_JOBS)
expect("T2 a dispatch, tier pr, code", plan([code], event="workflow_dispatch", tier="pr"), "pr", in_order(code_jobs))

# --- T6 the verdict -------------------------------------------------------------
def verdict(needs):
    r = subprocess.run([sys.executable, PLAN, "verdict"], capture_output=True, text=True, env=clean_env(NEEDS=json.dumps(needs)))
    return r.returncode, r.stdout + r.stderr


def run_of(tier, chosen, results=None, drop=(), hosts=None):
    outs = {"tier": tier, "jobs": json.dumps(chosen)}
    for k, v in (hosts if hosts is not None else (ALL_HOSTS if tier in FULL_LEG_TIERS else MERGE_HOSTS if tier == "full" else PR_HOSTS)).items():
        outs[k] = json.dumps(v, separators=(",", ":"))
    for k in drop:  # GitHub leaves an empty output out of `needs`
        del outs[k]
    n = {"plan": {"result": "success", "outputs": outs}}
    for j in EVERY:
        n[j] = {"result": "success" if j in chosen else "skipped", "outputs": {}}
    n.update(results or {})
    return n


def vexpect(name, needs, want):
    rc, text = verdict(needs)
    say(rc == want, "T6 %s: exit %d%s" % (name, rc, "" if rc == want else " (wanted %d) %s" % (want, text.strip()[-300:])))


failed = {"result": "failure", "outputs": {}}
first, last = FULL_JOBS[0], FULL_JOBS[-1]
vexpect("a green full run", run_of("full", FULL_JOBS), 0)
vexpect("a green all run", run_of("all", FULL_JOBS), 0)
if MERGE_HOSTS != ALL_HOSTS:
    vexpect("a full run on every leg (the merge before tl30)", run_of("full", FULL_JOBS, hosts=ALL_HOSTS), 1)
    vexpect("an all run on the merge's legs", run_of("all", FULL_JOBS, hosts=MERGE_HOSTS), 1)
vexpect("an all run that chose less than every job", run_of("all", FULL_JOBS[:-1]), 1)
vexpect("a green pr run", run_of("pr", in_order(code_jobs)), 0)
vexpect("a noop run", run_of("noop", []), 0)
vexpect("a noop run whose jobs output is absent", run_of("noop", [], drop=["jobs"]), 0)
vexpect("a chosen job failed", run_of("full", FULL_JOBS, {last: failed}), 1)
vexpect("a chosen job failed in a pr run", run_of("pr", in_order(code_jobs), {code_jobs[0]: failed}), 1)
vexpect("a chosen job was skipped", run_of("pr", in_order(code_jobs), {code_jobs[0]: {"result": "skipped", "outputs": {}}}), 1)
vexpect("a chosen job was cancelled", run_of("pr", in_order(code_jobs), {code_jobs[0]: {"result": "cancelled", "outputs": {}}}), 1)
vexpect("a job ran in a noop run", run_of("noop", [], {first: {"result": "success", "outputs": {}}}), 1)
vexpect("a full run whose jobs output is absent", run_of("full", FULL_JOBS, drop=["jobs"]), 1)
if len(FULL_JOBS) > 1:
    vexpect("a full run that chose less than every job", run_of("full", FULL_JOBS[:1]), 1)
    vexpect("a job ran unasked", run_of("pr", FULL_JOBS[:1], {FULL_JOBS[1]: {"result": "success", "outputs": {}}}), 1)
if ALL_HOSTS != PR_HOSTS:
    vexpect("a full run on the pull request's hosts", run_of("full", FULL_JOBS, hosts=PR_HOSTS), 1)
    vexpect("a pr run on every host", run_of("pr", in_order(code_jobs), hosts=ALL_HOSTS), 1)
    vexpect("a full run whose %s is absent" % sorted(ALL_HOSTS)[0], run_of("full", FULL_JOBS, drop=[sorted(ALL_HOSTS)[0]]), 1)
vexpect("a pr run that chose nothing", run_of("pr", []), 1)
vexpect("the plan failed", {"plan": {"result": "failure", "outputs": {}}, first: {"result": "skipped", "outputs": {}}}, 1)
vexpect("the plan named no tier", {"plan": {"result": "success", "outputs": {"jobs": "[]"}}, first: {"result": "skipped", "outputs": {}}}, 1)
vexpect("the plan named a tier that is none", run_of("fast", []), 1)
vexpect("the plan's jobs are no list", {"plan": {"result": "success", "outputs": {"tier": "pr", "jobs": first}}}, 1)
vexpect("a chosen job the tier job does not need", run_of("pr", [first, "lint"]), 1)

# --- T4 the wiring --------------------------------------------------------------
blocks = {}
for j in JOBS:
    m = re.search(r"^  %s:\s*\n(.*?)(?=^  [A-Za-z0-9_-]+:\s*$|\Z)" % re.escape(j), wf, re.M | re.S)
    blocks[j] = m.group(1) if m else ""
say(JOBS[:1] == ["plan"] and JOBS[-2:] == ["tier", "nightly-red"], "T4 the plan is the first job, the tier the last but the nightly's issue")
for j in EVERY:
    b = blocks[j]
    nd = re.search(r"^    needs: (plan|\[plan(, [A-Za-z0-9_-]+)*\])$", b, re.M)
    ok = nd is not None
    ok = ok and ("    if: ${{ (github.event_name != 'pull_request' || contains(github.event.pull_request.labels.*.name, 'full-matrix') || contains(github.event.pull_request.labels.*.name, 'ci') || contains(github.event.pull_request.labels.*.name, 'release-matrix')) && (contains(fromJSON(needs.plan.outputs.jobs || '[]'), '%s')) }}\n" % j) in b
    say(ok, "T4 %s needs the plan and runs only when the plan names it" % j)
    out = "matrix_" + j.replace("-", "_")
    if out in ALL_HOSTS:
        say(("        include: ${{ fromJSON(needs.plan.outputs.%s || '[]') }}\n" % out) in b
            and "    runs-on: ${{ matrix.os }}\n" in b and not re.search(r"^        (os|include):\s*$|^        os: \[", b, re.M),
            "T4 %s takes its hosts from the plan" % j)
    else:
        say("matrix.os" not in b and re.search(r"^    runs-on: ubuntu-", b, re.M) is not None or j in OTHER_HOST_JOBS,
            "T4 %s has no matrix of hosts (%s)" % (j, (re.findall(r"^    runs-on: (.*)$", b, re.M) or ["?"])[0]))
reads = re.findall(r"needs\.plan\.outputs\.([a-z_]+)(\s*\|\|)?", wf)
say(len(reads) >= 3 and all(d for _, d in reads), "T4 every plan output is read with a default (%d reads: %s)" % (
    len(reads), " ".join(sorted(set(k for k, _ in reads)))))
say(all(("      %s: ${{ steps.plan.outputs.%s }}\n" % (k, k)) in blocks["plan"] for k in ["tier", "jobs"] + sorted(ALL_HOSTS)),
    "T4 the plan job declares tier, jobs and each matrix")
say('        run: python3 ci/tiers.py plan --merge --github-output "$GITHUB_OUTPUT"\n' in blocks["plan"]
    and "          fetch-depth: ${{ github.event_name == 'workflow_dispatch' && '0' || '2' }}\n" in blocks["plan"]
    and "HEAD_SHA: ${{ github.event.pull_request.head.sha }}" in blocks["plan"]
    and "IN_TIER: ${{ inputs.tier }}" in blocks["plan"],
    "T4 the plan reads the merge commit (depth 2) or a dispatch's branch (the history), and the tier input")
say("        run: python3 ci/tiers-test.py\n" in blocks["plan"]
    and blocks["plan"].index("ci/tiers-test.py") < blocks["plan"].index("ci/tiers.py plan"),
    "T4 the plan job runs this file before it plans")
t = blocks["tier"]
tn = re.findall(r"^      - ([A-Za-z0-9_-]+)\s*$", t, re.M)
say(sorted(tn) == sorted(["plan"] + EVERY), "T4 the tier job needs every job (%d of %d)" % (len(tn), len(EVERY) + 1))
ifs = re.findall(r"^    if: (.*)$", t, re.M)
say(ifs == ["${{ (github.event_name != 'pull_request' || contains(github.event.pull_request.labels.*.name, 'full-matrix') || contains(github.event.pull_request.labels.*.name, 'ci') || contains(github.event.pull_request.labels.*.name, 'release-matrix')) && (!cancelled()) }}"], "T4 the opted-in tier job runs on !cancelled(), never always() (%s)" % "; ".join(ifs))
say("    name: tier ${{ needs.plan.outputs.tier || 'unplanned' }}\n" in t, "T4 the tier job is named for the tier")
say("        run: python3 ci/tiers.py verdict\n" in t and "          NEEDS: ${{ toJSON(needs) }}\n" in t and "actions/checkout" in t,
    "T4 the tier job's check is ci/tiers.py verdict over toJSON(needs)")
n = blocks["nightly-red"]
say("    needs: tier\n" in n and "    if: ${{ failure() && github.event_name == 'schedule' }}\n" in n
    and "      issues: write\n" in n and "        run: ci/nightly-red.sh\n" in n and "WORKFLOW: ${{ github.workflow }}" in n,
    "T4 a red nightly, and only a red nightly, runs ci/nightly-red.sh with issues: write")
say("  cancel-in-progress: ${{ github.event_name == 'pull_request' }}\n" in wf, "T4 cancel-in-progress for pull requests only")
group = re.search(r"^concurrency:\n  group: (.*)$", wf, re.M)
g = group.group(1).rstrip() if group else ""
say(g.endswith("|| format('ci-{0}-{1}', github.event_name, github.run_id) }}") and "github.ref" not in g.replace("github.head_ref", ""),
    "T4 a run that is no pull request's has a concurrency group of its own run id")
say("&& 'full' || 'pr'" in g and "format('noop-{0}', github.run_id)" in g,
    "T4 a pull request's full and pr runs are separate groups, and another label's noop cancels nothing")
say(re.search(r"^  push:\n    branches: \[%s\]\n" % re.escape(PUSH_BRANCHES), wf, re.M) is not None,
    "T4 a push runs on %s alone (a lane's push runs once, as its pull request)" % PUSH_BRANCHES)
say(re.search(r"^  schedule:\n(\s+#.*\n)*\s+- cron: ", wf, re.M) is not None, "T4 the nightly runs")
say("    types: [opened, synchronize, reopened, labeled]\n" in wf, "T4 adding a label starts a run")
say(re.search(r"^  workflow_dispatch:\n    inputs:\n(?:      [a-z_]+:\n(?:        .*\n)+)*      tier:\n", wf, re.M) is not None
    and "        options: [full, all, pr]\n" in wf, "T4 a dispatch takes the tier input, full, all or pr")
say(plan(event="schedule")["matrix"] == ALL_HOSTS, "T4 the all tier's (the nightly's) legs are the ones before tl28: %s" % json.dumps(BEFORE_HOSTS))
say(all(e in ALL_HOSTS.get(k, []) for k, v in BEFORE_HOSTS.items() for e in v) and
    all(len(ALL_HOSTS[k]) == len(v) for k, v in BEFORE_HOSTS.items()), "T4 no host left the all tier (the nightly's)")
say(plan(event="push")["matrix"] == MERGE_HOSTS, "T4 the full tier (the merge, a trunk push) runs the merge's legs: %s" % json.dumps(MERGE_HOSTS))
left = {k: [l for l in v if l not in MERGE_HOSTS.get(k, [])] for k, v in ALL_HOSTS.items()}
say(left == MOVED and all(l in ALL_HOSTS[k] for k, v in MERGE_HOSTS.items() for l in v),
    "T4 what the merge leaves is exactly tl30's move, and every leg of it runs in the nightly: %s" % json.dumps({k: v for k, v in left.items() if v}))
say(any(l.get("os") == "macos-latest" for v in MERGE_HOSTS.values() for l in v), "T4 the merge keeps a macOS smoke (tl30)")

# --- T7 the change read from git ----------------------------------------------------
def g_(repo, *args):
    cmd = ["git", "-C", repo, "-c", "user.name=tiers-test", "-c", "user.email=tiers-test@invalid",
           "-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"] + list(args)
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout.strip()


def write(repo, rel, text):
    full = os.path.join(repo, rel)
    os.makedirs(os.path.dirname(full) or full, exist_ok=True)
    with open(full, "w", encoding="utf-8") as f:
        f.write(text)


MOVED = code + ".moved"


def merged(topic_paths):
    """A repository whose HEAD merges a topic branch (changing topic_paths)
    into a base that moved on meanwhile -> (repo, the topic's head)."""
    repo = tempfile.mkdtemp(prefix="tiers-test-")
    g_(repo, "init", "-q", "-b", "trunk")
    for rel in [code] + PROSE_PATHS[:2]:
        write(repo, rel, "one\n")
    g_(repo, "add", ".")
    g_(repo, "commit", "-q", "-m", "base")
    g_(repo, "checkout", "-q", "-b", "topic")
    for rel in topic_paths:
        write(repo, rel, "the topic\n")
    g_(repo, "add", ".")
    g_(repo, "commit", "-q", "-m", "topic")
    head = g_(repo, "rev-parse", "HEAD")
    g_(repo, "checkout", "-q", "trunk")
    write(repo, MOVED, "the base moved on\n")
    g_(repo, "add", ".")
    g_(repo, "commit", "-q", "-m", "trunk moves")
    g_(repo, "merge", "-q", "--no-ff", "-m", "merge", "topic")
    return repo, head


def dispatched(topic_paths):
    """A clone whose HEAD is a branch off trunk (changing topic_paths), its
    origin/trunk moved on meanwhile -> (the upstream, the clone)."""
    up = tempfile.mkdtemp(prefix="tiers-test-up-")
    g_(up, "init", "-q", "-b", "trunk")
    for rel in [code] + PROSE_PATHS[:1]:
        write(up, rel, "one\n")
    g_(up, "add", ".")
    g_(up, "commit", "-q", "-m", "base")
    clone = tempfile.mkdtemp(prefix="tiers-test-clone-")
    subprocess.run(["git", "clone", "-q", up, clone], check=True, capture_output=True)
    g_(clone, "checkout", "-q", "-b", "tl")
    for rel in topic_paths:
        write(clone, rel, "the branch\n")
    g_(clone, "add", ".")
    g_(clone, "commit", "-q", "-m", "branch")
    write(up, MOVED, "trunk moved on\n")
    g_(up, "add", ".")
    g_(up, "commit", "-q", "-m", "trunk moves")
    g_(clone, "fetch", "-q", "origin")
    return up, clone


made = []
try:
    repo, head = merged(PROSE_PATHS[:2])
    made.append(repo)
    expect("T7 a prose-only merge, the base moved on (its %s is not the pull request's)" % MOVED,
           plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "noop", [])
    expect("T7 the same merge, the head not given", plan(repo=repo, flags=["--merge"], **pr), "noop", [])
    expect("T7 the merge's second parent is not the pull request's head",
           plan(repo=repo, head_sha="0" * 40, flags=["--merge"], **pr), "full", FULL_JOBS)
    g_(repo, "checkout", "-q", "topic")
    expect("T7 HEAD is the branch, not a merge commit", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "full", FULL_JOBS)
    repo, head = merged([PROSE_PATHS[0], code])
    made.append(repo)
    expect("T7 a merge that changes code and prose", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "pr", in_order(code_jobs))
    repo, head = merged([PROSE_PATHS[0], "somewhere/new.txt"])
    made.append(repo)
    expect("T7 a merge that adds a path no rule names", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "full", FULL_JOBS)
    dp = dict(event="workflow_dispatch", tier="pr", flags=["--merge"])
    up, clone = dispatched(PROSE_PATHS[:1])
    made += [up, clone]
    expect("T7 a dispatched prose-only branch, trunk moved on (its %s is not the branch's)" % MOVED,
           plan(repo=clone, **dp), "noop", [])
    up, clone = dispatched([code, PROSE_PATHS[0]])
    made += [up, clone]
    expect("T7 a dispatched branch that changes code", plan(repo=clone, **dp), "pr", in_order(code_jobs))
    up, clone = dispatched(["plant-unmapped.txt", code])
    made += [up, clone]
    expect("T7 a dispatched branch that adds a path no rule names (the plant)", plan(repo=clone, **dp), "full", FULL_JOBS)
    g_(clone, "checkout", "-q", "-B", "on-trunk", "origin/trunk")
    expect("T7 a dispatch at trunk itself, tier pr (no change to read)", plan(repo=clone, **dp), "full", FULL_JOBS)
    lone, _ = merged([code])
    made.append(lone)
    expect("T7 a dispatch with no origin/trunk to read", plan(repo=lone, **dp), "full", FULL_JOBS)
except (subprocess.CalledProcessError, OSError) as e:
    say(False, "T7 could not build its repositories: %s %s" % (e, getattr(e, "stderr", "")))
finally:
    for d in made:
        shutil.rmtree(d, ignore_errors=True)

# --- T8 the nightly's issue -------------------------------------------------------
FAKE_GH = r'''#!/usr/bin/env bash
printf '%s\n' "$*" >> "$FAKE_LOG"
case "$1 $2" in
  "label create") exit "${FAKE_LABEL_RC:-0}" ;;
  "issue list") printf '%s' "${FAKE_EXISTING:-}"; [ -n "${FAKE_EXISTING:-}" ] && echo; exit 0 ;;
esac
exit 0
'''


def nightly_red(existing="", label_rc=0):
    d = tempfile.mkdtemp(prefix="tiers-test-gh-")
    try:
        with open(os.path.join(d, "gh"), "w") as f:
            f.write(FAKE_GH)
        os.chmod(os.path.join(d, "gh"), 0o755)
        log = os.path.join(d, "log")
        env = dict(os.environ, PATH=d + os.pathsep + os.environ.get("PATH", ""), FAKE_LOG=log, FAKE_EXISTING=existing,
                   FAKE_LABEL_RC=str(label_rc), REPO="wolffe-lang/wolf-std", WORKFLOW=WORKFLOW_NAME,
                   RUN_URL="https://github.com/wolffe-lang/wolf-std/actions/runs/1")
        r = subprocess.run(["bash", RED], capture_output=True, text=True, env=env)
        calls = open(log).read().splitlines() if os.path.exists(log) else []
        return r.returncode, calls
    finally:
        shutil.rmtree(d, ignore_errors=True)


TITLE = "nightly red: %s on trunk" % WORKFLOW_NAME
rc, calls = nightly_red()
creates = [c for c in calls if c.startswith("issue create")]
say(rc == 0 and len(creates) == 1 and "--title %s" % TITLE in creates[0] and "--label nightly-red" in creates[0]
    and not [c for c in calls if c.startswith("issue comment")],
    "T8 no open issue: one is opened, labelled nightly-red, titled '%s'" % TITLE)
lists = [c for c in calls if c.startswith("issue list")]
say(len(lists) == 1 and "--state open" in lists[0] and "--label nightly-red" in lists[0] and TITLE in lists[0],
    "T8 the dedupe asks for an open nightly-red issue under the title")
rc, calls = nightly_red(existing="41")
say(rc == 0 and [c for c in calls if c.startswith("issue comment 41 ")] and not [c for c in calls if c.startswith("issue create")],
    "T8 an open issue: a comment on it, no new issue")
rc, calls = nightly_red(label_rc=1)
say(rc == 0 and [c for c in calls if c.startswith("issue create")], "T8 a label that already exists is no failure")
r = subprocess.run(["bash", RED], capture_output=True, text=True, env=clean_env(PATH=os.environ.get("PATH", "")))
say(r.returncode != 0, "T8 the script refuses to run without REPO, WORKFLOW and RUN_URL")
say(re.search(r"^name: %s$" % re.escape(WORKFLOW_NAME), wf, re.M) is not None, "T8 the workflow's name is %s, the title's" % WORKFLOW_NAME)

# --- T5 the tripwire: what the planner calls prose, nothing reads ---------------------
def tracked():
    try:
        return subprocess.run(["git", "-C", ROOT, "ls-files"], check=True, capture_output=True, text=True).stdout.split("\n")
    except (subprocess.CalledProcessError, FileNotFoundError):
        return []


def classes(paths):
    out = {}
    for i in range(0, len(paths), 500):
        r = subprocess.run([sys.executable, PLAN, "classify"] + paths[i:i + 500], check=True, capture_output=True, text=True)
        out.update((line.split("\t")[0], line.split("\t")[1]) for line in r.stdout.splitlines())
    return out


def patterns(p):
    """How a line names p: a directory root (`notes/`) by `notes/` or a quoted
    "notes"; a file by its path, or its own name quoted (os.path.join)."""
    if p.endswith("/"):
        d = re.escape(p[:-1])
        return [re.compile(r"(?<![A-Za-z0-9_.-])%s/" % d), re.compile(r"[\"']%s[\"']" % d)]
    return [re.compile(r"(?<![A-Za-z0-9_./-])%s(?![A-Za-z0-9_])" % re.escape(p)),
            re.compile(r"[\"']%s[\"']" % re.escape(os.path.basename(p)))]


def readers(code_files, paths):
    """{path: ["file:line", ...]}: the lines of code_files that read a file
    (READS) and name the path (patterns). Comment lines are no reader, nor a
    line RULED says names something else (its regex, with its reason)."""
    pats = {p: patterns(p) for p in paths}
    hits = {p: [] for p in paths}
    for c in code_files:
        try:
            lines = open(os.path.join(ROOT, c), encoding="utf-8", errors="replace").read().splitlines()
        except OSError:
            continue
        for n, line in enumerate(lines, 1):
            if line.lstrip().startswith(("#", "//")) or not READS.search(line):
                continue
            for p in paths:
                if any(x.search(line) for x in pats[p]) and not any(re.search(rx, line) for rx in RULED.get(p, {})):
                    hits[p].append("%s:%d" % (c, n))
    return hits


files = [f for f in tracked() if f]
if files:
    cls = classes(files)
    prose = sorted(p for p in files if cls.get(p) == "prose")
    unknown = sorted(p for p in files if cls.get(p) == "FULL")
    print("tiers-test: %d tracked paths are prose: %s" % (len(prose), " ".join(prose[:10]) + (" ..." if len(prose) > 10 else "")))
    code_files = [f for f in files if is_code(f)]
    seen = readers(code_files, MUST_FIND)
    say(all(seen[p] for p in MUST_FIND), "T5 the search fires: it finds what reads %s" % ", ".join(
        "%s (%s)" % (p, seen[p][0] if seen[p] else "NOTHING") for p in MUST_FIND))
    roots = sorted({p.split("/")[0] + ("/" if "/" in p else "") for p in prose})
    hit = {p: w for p, w in readers(code_files, roots).items() if w}
    say(not hit, "T5 no line of %d code file(s) that reads a file names a prose root (%s): %s" % (
        len(code_files), " ".join(roots), "; ".join("%s at %s" % (p, " ".join(w[:3])) for p, w in sorted(hit.items())) or "none"))
    extra_t5(files, cls, say)
    if unknown:
        print("tiers-test: WARN T5 %d tracked path(s) no rule names (a pull request touching one runs full): %s"
              % (len(unknown), " ".join(unknown[:20])))
    else:
        print("tiers-test: PASS T5 every one of %d tracked paths has a rule" % len(files))
else:
    say(False, "T5 not a git checkout: the tracked paths cannot be listed")

extra_checks(say, plan, expect, blocks, wf)

# T9: a runner image named `latest` moves under every job at once. On 2026-11-18
# ubuntu-latest began handing out Ubuntu 26.04 runners, whose toolchain and libc
# differ from 24.04's, and a pull request's run could pass on one image while the
# trunk push ran on the other. No workflow line and no planner leg may say
# ubuntu-latest: the Linux runners are pinned by version, and moving them is a
# change of its own.
import glob
stale = []
for path in sorted(glob.glob(os.path.join(ROOT, ".github", "workflows", "*"))) + [PLAN]:
    with open(path, encoding="utf-8") as fh:
        for n, line in enumerate(fh, 1):
            if "ubuntu-latest" in line:
                stale.append("%s:%d" % (os.path.relpath(path, ROOT), n))
say(not stale, "T9 no workflow or planner leg says ubuntu-latest, the Linux runner image is pinned"
    + ("" if not stale else " (%d places: %s)" % (len(stale), ", ".join(stale))))

# T11 a nightly at a commit already green in full is skipped, and says so (ci/covered.py, against a fake gh).
COVERED_PY = os.path.join(ROOT, "ci", "covered.py")
SKIP_TIER = NIGHTLY[0]
PLAIN_ = re.compile(r"\b(?:lanes?|rulings?|crawls?|sprints?)\b|\b[a-z]{2}\d{2,3}[a-z]?\b", re.I)
GREEN = dict(covered=SKIP_TIER, covered_by="https://github.com/o/r/runs/555", covered_at="2026-11-18T12:00:00Z", covered_run="777")
job0 = EVERY[0]

# S1: the nightly, covered: skipped, no job; its line, its warning and its summary name the covering run.
expect("T11 S1 the nightly at a commit whose tier %s is green" % SKIP_TIER, plan(event="schedule", **GREEN), "skipped", [])
d = tempfile.mkdtemp(prefix="tiers-test-skip-")
try:
    summ, gout = os.path.join(d, "summary.md"), os.path.join(d, "out")
    args = [sys.executable, PLAN, "plan", "--event", "schedule", "--github-output", gout] + [
        x for k, v in GREEN.items() for x in ("--" + k.replace("_", "-"), v)]
    r = subprocess.run(args, capture_output=True, text=True, env=clean_env(GITHUB_STEP_SUMMARY=summ))
    outs = open(gout, encoding="utf-8").read() if os.path.exists(gout) else ""
    summary = open(summ, encoding="utf-8").read() if os.path.exists(summ) else ""
    warn = [l for l in r.stdout.splitlines() if l.startswith("::warning title=nightly skipped::")]
    said_ = ("nothing tested tonight", "tier %s green at this commit in run 777" % SKIP_TIER, "https://github.com/o/r/runs/555",
             "completed 2026-11-18T12:00:00Z", "due by 2026-11-24T12:00:00Z")
    say(r.returncode == 0 and "tier=skipped\n" in outs and "jobs=[]\n" in outs and "covered_by=https://github.com/o/r/runs/555\n" in outs
        and len(warn) == 1 and all(s in warn[0] for s in said_) and all(s in r.stderr for s in said_) and all(s in summary for s in said_)
        and not PLAIN_.search(warn[0] + summary),
        "T11 S1 a skipped night says so in its log, a warning and the run's summary, in plain words, naming the covering run and when a full night is due%s" % (
            "" if r.returncode == 0 else " (rc %d: %s)" % (r.returncode, r.stderr.strip()[-200:])))
finally:
    shutil.rmtree(d, ignore_errors=True)

# S2: anything else the nightly is told is no skip: the nightly's tier, as today.
for c in ("none", "", "full", "pr", "skipped", "noop", "release", SKIP_TIER.upper(), SKIP_TIER + " yes"):
    expect("T11 S2 the nightly told covered=%r" % c, plan(event="schedule", covered=c), NIGHTLY[0], NIGHTLY[1])
expect("T11 S2 the nightly told nothing", plan(event="schedule"), NIGHTLY[0], NIGHTLY[1])

# S3: a skip is the nightly's only.
expect("T11 S3 a push told covered=%s" % SKIP_TIER, plan(event="push", **GREEN), "full", FULL_JOBS)
expect("T11 S3 a dispatch told covered=%s" % SKIP_TIER, plan(event="workflow_dispatch", **GREEN), "full", FULL_JOBS)
expect("T11 S3 a dispatch asking for tier all, told covered=%s" % SKIP_TIER, plan(event="workflow_dispatch", tier="all", **GREEN), "all", FULL_JOBS)
expect("T11 S3 a code pull request told covered=%s" % SKIP_TIER,
       plan([code], event="pull_request", action="synchronize", **GREEN), "pr", in_order(code_jobs))
say(plan(event="workflow_dispatch", tier="skipped")["tier"].startswith("ERROR rc=2"), "T11 S3 a dispatch asking for tier skipped is refused")

# S4: the verdict of a skipped night.
vexpect("T11 S4 a skipped night, every job skipped", run_of("skipped", []), 0)
vexpect("T11 S4 a skipped night in which a job ran", run_of("skipped", [], {job0: {"result": "success", "outputs": {}}}), 1)
vexpect("T11 S4 a skipped night whose plan chose a job", run_of("skipped", [job0]), 1)
n_ = run_of("skipped", [])
n_["plan"]["outputs"]["covered_by"] = "https://github.com/o/r/runs/555"
rc, text = verdict(n_)
say(rc == 0 and "nothing was tested" in text and "https://github.com/o/r/runs/555" in text,
    "T11 S4 the verdict of a skipped night says nothing was tested and names the covering run")

# S5: ci/covered.py against a fake gh.
FAKE_GH_COV = """#!/usr/bin/env python3
import json, os, sys
a = sys.argv[1:]
with open(os.environ["FAKE_GH_LOG"], "a", encoding="utf-8") as f:
    f.write(json.dumps(a) + "\\n")
if os.environ.get("FAKE_GH_FAIL"):
    sys.stderr.write("HTTP 502: the fake refuses\\n")
    sys.exit(1)
path = a[-1] if a[:1] == ["api"] else ""
if "/check-runs" in path:
    sys.stdout.write(os.environ["FAKE_CHECKS"])
elif "/actions/runs?" in path:
    sys.stdout.write(os.environ["FAKE_RUNS"])
elif "/actions/runs/" in path:
    sys.stdout.write(os.environ["FAKE_RUN"])
else:
    sys.stderr.write("the fake knows no %r\\n" % a)
    sys.exit(1)
"""
COV_SHA = "0e536e8f" + "0" * 32


def check(name="tier " + SKIP_TIER, concl="success", done="2026-11-18T12:00:00Z", suite=1, app="github-actions",
          url="https://github.com/o/r/runs/555", status="completed"):
    return {"name": name, "status": status, "conclusion": concl, "completed_at": done, "check_suite": {"id": suite},
            "app": {"slug": app}, "html_url": url}


def wrun(suite=1, path=".github/workflows/ci.yml", sha=COV_SHA, rid=777):
    return {"id": rid, "check_suite_id": suite, "path": path, "head_sha": sha}


def covered(checks, runs=None, event="schedule", fail=False, total=None, drop=()):
    d = tempfile.mkdtemp(prefix="tiers-test-cov-")
    try:
        with open(os.path.join(d, "gh"), "w", encoding="utf-8") as f:
            f.write(FAKE_GH_COV)
        os.chmod(os.path.join(d, "gh"), 0o755)
        log = os.path.join(d, "calls.jsonl")
        runs = [wrun()] if runs is None else runs
        env = clean_env(PATH=d + os.pathsep + os.environ.get("PATH", ""), FAKE_GH_LOG=log, REPO="o/r", SHA=COV_SHA,
                        TIER=SKIP_TIER, RUN_ID="999", EVENT=event, GITHUB_EVENT_NAME="schedule",
                        FAKE_RUN=json.dumps({"id": 999, "check_suite_id": 9, "created_at": "2026-11-20T12:00:00Z"}),
                        FAKE_CHECKS=json.dumps({"total_count": len(checks) if total is None else total, "check_runs": checks}),
                        FAKE_RUNS=json.dumps({"total_count": len(runs), "workflow_runs": runs}))
        for k in drop:
            env.pop(k, None)
        if fail:
            env["FAKE_GH_FAIL"] = "1"
        r = subprocess.run([sys.executable, COVERED_PY], capture_output=True, text=True, env=env)
        got = dict(l.split("=", 1) for l in r.stdout.splitlines() if "=" in l)
        calls = [json.loads(x) for x in open(log, encoding="utf-8")] if os.path.exists(log) else []
        return r.returncode, got, calls, r.stderr
    finally:
        shutil.rmtree(d, ignore_errors=True)


def cexpect(name, res, want):
    rc, got, calls, err = res
    ok = rc == 0 and got.get("covered") == want and bool(got.get("reason"))
    say(ok, "T11 S5 %s: covered=%s%s" % (name, got.get("covered"), "" if ok else " (wanted %s; rc %d; %s)" % (want, rc, err.strip()[-200:])))


if os.name == "nt":
    say(True, "T11 S5 skipped: the fake gh is a script with a shebang")
else:
    res = covered([check()])
    rc, got, calls, err = res
    say(rc == 0 and got.get("covered") == SKIP_TIER and got.get("covered_by") == "https://github.com/o/r/runs/555"
        and got.get("covered_at") == "2026-11-18T12:00:00Z" and got.get("covered_run") == "777"
        and any(("check_name=tier%%20%s" % SKIP_TIER) in c[-1] and "filter=all" in c[-1] for c in calls),
        "T11 S5 (a) a green tier %s of ci.yml at the commit, 2 days old: covered, naming the check run, its time and its run%s" % (
            SKIP_TIER, "" if rc == 0 and got.get("covered") == SKIP_TIER else " (%s; %s)" % (got, err.strip()[-200:])))
    cexpect("(b) the same, 7 days old", covered([check(done="2026-11-13T11:00:00Z")]), "none")
    cexpect("(b) the same, 144 hours old to the second", covered([check(done="2026-11-14T12:00:00Z")]), SKIP_TIER)
    for nm in ("tier full", "tier pr", "tier skipped", "tier noop", "tier " + SKIP_TIER + "s"):
        cexpect("(c) a green %s" % nm, covered([check(name=nm)]), "none")
    cexpect("(d) a green tier %s of another workflow" % SKIP_TIER, covered([check()], runs=[wrun(path=".github/workflows/release.yml")]), "none")
    cexpect("(d) a green tier %s whose run is of another commit" % SKIP_TIER, covered([check()], runs=[wrun(sha="1" * 40)]), "none")
    cexpect("(d) a green tier %s whose run is not at this commit at all" % SKIP_TIER, covered([check()], runs=[]), "none")
    for c in ("failure", "cancelled", "timed_out"):
        cexpect("(e) the tier %s %s" % (SKIP_TIER, c), covered([check(concl=c)]), "none")
    cexpect("(e) the tier %s still running" % SKIP_TIER, covered([check(status="in_progress", concl=None, done=None)]), "none")
    res = covered([check()], fail=True)
    cexpect("(g) gh exits non-zero", res, "none")
    say("unanswered" in res[1].get("reason", ""), "T11 S5 (g) the reason says the question went unanswered")
    cexpect("(h) a green tier %s of another app" % SKIP_TIER, covered([check(app="some-app")]), "none")
    cexpect("(i) a full page of check runs", covered([check()], total=100), "none")
    cexpect("(i) a full page of workflow runs", covered([check()], runs=[wrun(rid=700 + i, suite=1 if i == 0 else 100 + i) for i in range(100)]), "none")
    cexpect("the latest word: a red tier %s after a green one at the commit" % SKIP_TIER,
            covered([check(), check(concl="failure", done="2026-11-19T12:00:00Z", suite=2, url="https://github.com/o/r/runs/556")],
                    runs=[wrun(), wrun(suite=2, rid=778)]), "none")
    cexpect("the latest word: a red first attempt, then a green rerun",
            covered([check(concl="failure", done="2026-11-18T10:00:00Z"), check(done="2026-11-18T12:00:00Z")]), SKIP_TIER)
    cexpect("an earlier attempt of tonight's own run, green (a rerun of a green night)",
            covered([check(suite=9)], runs=[wrun(suite=9, rid=999)]), "none")
    res = covered([check()], event="push")
    cexpect("a push: never covered", res, "none")
    say(res[0] == 0 and not res[2] and "not the nightly" in res[1].get("reason", ""),
        "T11 S5 a push: covered.py asks GitHub nothing (%d calls) and says why" % len(res[2]))
    cexpect("no EVENT: GITHUB_EVENT_NAME decides (schedule)", covered([check()], event=""), SKIP_TIER)
    res = covered([check()], drop=("TIER",))
    say(res[0] == 2 and "must be set" in res[3] and not res[2], "T11 S5 covered.py refuses to run without TIER, and asks nothing")

# S6: the wiring of the plan job.
pb = blocks["plan"]
cov_step = re.search(r"^      - name: what this commit already has green \(ci/covered\.py\)\n((?:        .*\n)+)", pb, re.M)
cs = cov_step.group(1) if cov_step else ""
plan_step = re.search(r"^      - name: the plan\n((?:        .*\n)+)", pb, re.M)
ps = plan_step.group(1) if plan_step else ""
pperms = re.search(r"^    permissions:\n((?:      .*\n)+)", pb, re.M)
say(bool(cs) and "        id: covered\n" in cs and '        run: python3 ci/covered.py --github-output "$GITHUB_OUTPUT"\n' in cs
    and all(("          %s\n" % e) in cs for e in ("GH_TOKEN: ${{ github.token }}", "REPO: ${{ github.repository }}",
                                                   "SHA: ${{ github.sha }}", "RUN_ID: ${{ github.run_id }}", "TIER: %s" % SKIP_TIER))
    and bool(ps) and pb.index("ci/covered.py") < pb.index("ci/tiers.py plan"),
    "T11 S6 the plan job asks ci/covered.py, with the nightly's tier, before it plans")
say(all(("          %s: ${{ steps.covered.outputs.%s }}\n" % (k.upper(), k)) in ps for k in ("covered", "covered_by", "covered_at", "covered_run"))
    and wf.count("steps.covered.outputs.") == 4 and "      covered_by: ${{ steps.plan.outputs.covered_by }}\n" in pb,
    "T11 S6 only the plan step reads ci/covered.py's answer, and the plan passes covered_by on to the verdict")
say(pperms is not None and sorted(pperms.group(1).split()) == sorted(["contents:", "read", "checks:", "read", "actions:", "read"]),
    "T11 S6 the plan job reads the code, the checks and the runs, and writes nothing")

# S7: the nightly's issue says a skipped night is not a green one.
# The stand-in logs each call on one line, and the body's own lines after it: read each run's log whole.
said_ = ["\n".join(nightly_red(existing=e)[1]) for e in ("", "41")]
say("issue create" in said_[0] and "issue comment 41" in said_[1]
    and all("close this issue once a nightly is green again (a `tier skipped` night is not one)" in t for t in said_),
    "T11 S7 the nightly's issue, opened or commented, says to close it once a nightly is green again, and that a tier skipped night is not one")

print("tiers-test: %s" % ("PASS" if fails == 0 else "FAIL (%d)" % fails))
sys.exit(1 if fails else 0)

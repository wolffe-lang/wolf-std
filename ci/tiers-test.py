#!/usr/bin/env python3
"""ci/tiers-test.py — the planner's own proof (tl28; wolf-lang's
ci/tiers-test.py, tl21, cut to wolf-std). python3, git and bash only.

    ci/tiers-test.py

  T1  the tiers by event: a trunk push, the nightly, a dispatch (tier full,
      or no tier) and the `full-matrix` label are full on every host;
      another label's `labeled` event is a noop; a dispatch's tier must be
      full or pr.
  T2  the pull-request tiers by path: library, tests, vendor, rig and CI
      paths are pr (rig and gates, on ubuntu alone); prose alone is noop;
      prose beside code is pr. The library's paths are never prose.
  T3  full wins: a path no rule names, alone or beside anything; changed
      paths that cannot be read; a change that names no path.
  T4  the wiring of .github/workflows/ci.yml: the plan is first and the
      verdict last but the nightly's issue; every job needs the plan, runs
      only when the plan names it and takes its hosts from the plan; every
      plan output is read with a default; the `tier` job needs every job,
      runs on `!cancelled()` and checks with `ci/tiers.py verdict`;
      cancel-in-progress is for pull requests only and every other run has a
      concurrency group of its own run id; the trunk push (and no other
      branch's), the nightly, the label and the dispatch's `tier` input are
      there; MATRIX's hosts are the three the jobs ran on before tl28.
  T5  the tripwire: no line of xtask/ or tests/ names a path the planner
      calls prose (or the directory above one), and fmt-lu's roots are
      std/ and tests/. The search is shown to fire on paths it must find.
      (A WARN, not a failure: a tracked path no rule names; its pull
      request runs full.)
  T6  the verdict: green for a full, a pr and a noop run; red for a chosen
      job failed, skipped or cancelled, a job that ran unasked, a failed
      plan, a plan with no tier, a full run that chose less, a full run
      whose hosts are the pull request's, and green when GitHub drops an
      output that would have been empty.
  T7  the change read from git: a real merge commit against its first
      parent, with the base moved on meanwhile; red-to-full when HEAD is no
      merge or its second parent is not the pull request's head; a
      dispatched branch against its merge base with origin/trunk, and full
      when there is no origin/trunk to read.
  T8  ci/nightly-red.sh against a stand-in `gh`: no open issue opens one,
      labelled, under the one title; an open one gets a comment and no new
      issue; a label that already exists is no failure.

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
WORKFLOW = os.path.join(ROOT, ".github", "workflows", "ci.yml")
ENV_KEYS = ("EVENT", "ACTION", "LABEL", "LABELS", "HEAD_SHA", "IN_TIER", "NEEDS")
fails = 0


def say(ok, what):
    global fails
    print("tiers-test: %s %s" % ("PASS" if ok else "FAIL", what))
    fails += not ok


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
HOSTS = ["ubuntu-latest", "macos-latest", "windows-latest"]
ALL_HOSTS = {"matrix_rig": [{"os": h} for h in HOSTS], "matrix_gates": [{"os": h} for h in HOSTS]}
PR_HOSTS = {"matrix_rig": [{"os": "ubuntu-latest"}], "matrix_gates": [{"os": "ubuntu-latest"}]}
BOTH = ["rig", "gates"]


def expect(name, got, tier, jobs):
    why = []
    if got["tier"] != tier:
        why.append("tier %s, wanted %s" % (got["tier"], tier))
    if got["jobs"] != jobs:
        why.append("jobs %s, wanted %s" % (" ".join(got["jobs"]) or "none", " ".join(jobs) or "none"))
    hosts = ALL_HOSTS if tier == "full" else PR_HOSTS
    if not got["tier"].startswith("ERROR") and got.get("matrix") != hosts:
        why.append("hosts %s, wanted %s" % (json.dumps(got.get("matrix")), json.dumps(hosts)))
    shown = "every job (%d), every host" % len(EVERY) if jobs == EVERY and tier == "full" else (" ".join(jobs) or "no job")
    say(not why, "%s: %s%s" % (name, tier, (" (" + "; ".join(why) + ")") if why else ", " + shown))


# --- T1 the tiers by event ---------------------------------------------------
say(JOBS == ["plan", "rig", "gates", "tier", "nightly-red"], "T1 the workflow's jobs: %s" % " ".join(JOBS))
expect("T1 a push to trunk", plan(event="push"), "full", EVERY)
expect("T1 the nightly", plan(event="schedule"), "full", EVERY)
expect("T1 a dispatch, no tier", plan(event="workflow_dispatch"), "full", EVERY)
expect("T1 a dispatch, tier full", plan(event="workflow_dispatch", tier="full"), "full", EVERY)
say(plan(event="workflow_dispatch", tier="partial")["tier"].startswith("ERROR rc=2"), "T1 a dispatch's tier is full or pr, nothing else")
expect("T1 a pull request labelled full-matrix, prose only",
       plan(["CHANGELOG.md"], event="pull_request", action="synchronize", labels="bug,full-matrix"), "full", EVERY)
expect("T1 the full-matrix label added",
       plan(["CHANGELOG.md"], event="pull_request", action="labeled", label="full-matrix", labels="full-matrix"), "full", EVERY)
expect("T1 another label added", plan(["std/fmt/fmt.lu"], event="pull_request", action="labeled", label="bug", labels="bug"), "noop", [])
expect("T1 another label added to a labelled pull request",
       plan(["std/fmt/fmt.lu"], event="pull_request", action="labeled", label="bug", labels="bug,full-matrix"), "noop", [])
expect("T1 the tier input means nothing to a pull request",
       plan(["std/fmt/fmt.lu"], event="pull_request", action="synchronize", tier="full"), "pr", BOTH)

# --- T2 the pull-request tiers by path ------------------------------------------
pr = dict(event="pull_request", action="synchronize")
for p in ["std/fmt/fmt.lu", "std/net/net.lu", "std/prelude/prelude.lu", "tests/fs/read.lu", "tests/ledger.toml",
          "tests/ulp/reference.txt", "vendor/tools.toml", "vendor/upstream/PIN", "vendor/upstream/anchors.json",
          "vendor/vectors/README.md", "upstream", "xtask/src/main.rs", "xtask/Cargo.toml", "Cargo.toml", "Cargo.lock",
          "rust-toolchain.toml", ".cargo/config.toml", ".fac/workspace.json", "assets/logo.svg",
          ".github/workflows/ci.yml", ".github/workflows/nightly.yml", "ci/tiers.py", "ci/tiers-test.py",
          "ci/nightly-red.sh", "README.md", "API-CONVENTIONS.md", "GUIDE-ENTRIES-sc40.txt", "LICENSE",
          "LICENSE-EXCEPTION", "LICENSE-TRAINING-DATA", ".gitmodules", ".gitignore", ".gitattributes"]:
    expect("T2 %s" % p, plan([p], **pr), "pr", BOTH)
for p in ["CHANGELOG.md", "CONTRIBUTING.md", "docs/findings.md", "docs/error-taxonomy.md",
          "docs/sc64-witness/jobs.lu", "docs/sc62-witness/pty.py", "docs/a-new-page.md"]:
    expect("T2 prose: %s" % p, plan([p], **pr), "noop", [])
expect("T2 prose alone, several", plan(["CHANGELOG.md", "docs/findings.md", "CONTRIBUTING.md"], **pr), "noop", [])
expect("T2 prose beside code", plan(["CHANGELOG.md", "std/fmt/fmt.lu", "docs/findings.md"], **pr), "pr", BOTH)
for p in ["std/README.md", "tests/README.md", "vendor/upstream/spec/05-conformance.md"]:
    expect("T2 not prose (under the library, its tests or its pins): %s" % p, plan([p], **pr), "pr", BOTH)

# --- T3 full wins ---------------------------------------------------------------
expect("T3 a path no rule names", plan(["newplace/thing.rs"], **pr), "full", EVERY)
expect("T3 a new file at the root", plan(["build.rs"], **pr), "full", EVERY)
expect("T3 a markdown file at the root no rule names", plan(["NOTES.md"], **pr), "full", EVERY)
expect("T3 an unknown path beside prose", plan(["CHANGELOG.md", "newplace/notes.md"], **pr), "full", EVERY)
expect("T3 an unknown path beside code", plan(["std/fmt/fmt.lu", "newplace/thing.rs"], **pr), "full", EVERY)
expect("T3 no changed paths to read", plan(**pr), "full", EVERY)
expect("T3 a change that names no path", plan([], **pr), "full", EVERY)
expect("T3 a merge that cannot be read (no repository there)",
       plan(repo=os.path.join(ROOT, "no", "such", "dir"), flags=["--merge"], **pr), "full", EVERY)
expect("T3 a dispatch, tier pr, an unknown path", plan(["newplace/thing.rs"], event="workflow_dispatch", tier="pr"), "full", EVERY)
expect("T3 a dispatch, tier pr, a branch that cannot be read",
       plan(repo=os.path.join(ROOT, "no", "such", "dir"), flags=["--merge"], event="workflow_dispatch", tier="pr"), "full", EVERY)
expect("T2 a dispatch, tier pr, code", plan(["std/fmt/fmt.lu"], event="workflow_dispatch", tier="pr"), "pr", BOTH)

# --- T6 the verdict -------------------------------------------------------------
def verdict(needs):
    r = subprocess.run([sys.executable, PLAN, "verdict"], capture_output=True, text=True, env=clean_env(NEEDS=json.dumps(needs)))
    return r.returncode, r.stdout + r.stderr


def run_of(tier, chosen, results=None, drop=(), hosts=None):
    outs = {"tier": tier, "jobs": json.dumps(chosen)}
    for k, v in (hosts or (ALL_HOSTS if tier == "full" else PR_HOSTS)).items():
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
vexpect("a green full run", run_of("full", EVERY), 0)
vexpect("a green pr run", run_of("pr", BOTH), 0)
vexpect("a green pr run of one job", run_of("pr", ["gates"]), 0)
vexpect("a noop run", run_of("noop", []), 0)
vexpect("a noop run whose jobs output is absent", run_of("noop", [], drop=["jobs"]), 0)
vexpect("a chosen job failed", run_of("full", EVERY, {"rig": failed}), 1)
vexpect("a chosen job failed in a pr run", run_of("pr", BOTH, {"gates": failed}), 1)
vexpect("a chosen job was skipped", run_of("pr", BOTH, {"gates": {"result": "skipped", "outputs": {}}}), 1)
vexpect("a chosen job was cancelled", run_of("pr", BOTH, {"rig": {"result": "cancelled", "outputs": {}}}), 1)
vexpect("a job ran unasked", run_of("pr", ["gates"], {"rig": {"result": "success", "outputs": {}}}), 1)
vexpect("a job ran in a noop run", run_of("noop", [], {"gates": {"result": "success", "outputs": {}}}), 1)
vexpect("a full run that chose less than every job", run_of("full", ["rig"]), 1)
vexpect("a full run whose jobs output is absent", run_of("full", EVERY, drop=["jobs"]), 1)
vexpect("a full run on the pull request's hosts", run_of("full", EVERY, hosts=PR_HOSTS), 1)
vexpect("a full run whose rig hosts are absent", run_of("full", EVERY, drop=["matrix_rig"]), 1)
vexpect("a pr run on every host", run_of("pr", BOTH, hosts=ALL_HOSTS), 1)
vexpect("a pr run that chose nothing", run_of("pr", []), 1)
vexpect("the plan failed", {"plan": {"result": "failure", "outputs": {}}, "rig": {"result": "skipped", "outputs": {}}}, 1)
vexpect("the plan named no tier", {"plan": {"result": "success", "outputs": {"jobs": "[]"}}, "rig": {"result": "skipped", "outputs": {}}}, 1)
vexpect("the plan named a tier that is none", run_of("fast", []), 1)
vexpect("the plan's jobs are no list", {"plan": {"result": "success", "outputs": {"tier": "pr", "jobs": "rig"}}}, 1)
vexpect("a chosen job the tier job does not need", run_of("pr", ["rig", "lint"]), 1)

# --- T4 the wiring --------------------------------------------------------------
blocks = {}
for j in JOBS:
    m = re.search(r"^  %s:\s*\n(.*?)(?=^  [A-Za-z0-9_-]+:\s*$|\Z)" % re.escape(j), wf, re.M | re.S)
    blocks[j] = m.group(1) if m else ""
say(JOBS[:1] == ["plan"] and JOBS[-2:] == ["tier", "nightly-red"], "T4 the plan is the first job, the tier the last but the nightly's issue")
for j in EVERY:
    b = blocks[j]
    ok = re.search(r"^    needs: plan$", b, re.M) is not None
    ok = ok and ("    if: contains(fromJSON(needs.plan.outputs.jobs || '[]'), '%s')\n" % j) in b
    say(ok, "T4 %s needs the plan and runs only when the plan names it" % j)
    say(("        include: ${{ fromJSON(needs.plan.outputs.matrix_%s || '[]') }}\n" % j.replace("-", "_")) in b
        and "    runs-on: ${{ matrix.os }}\n" in b and not re.search(r"^        os: ", b, re.M),
        "T4 %s takes its hosts from the plan" % j)
reads = re.findall(r"needs\.plan\.outputs\.([a-z_]+)(\s*\|\|)?", wf)
say(len(reads) >= 5 and all(d for _, d in reads), "T4 every plan output is read with a default (%d reads: %s)" % (
    len(reads), " ".join(sorted(set(k for k, _ in reads)))))
say(all(("      %s: ${{ steps.plan.outputs.%s }}\n" % (k, k)) in blocks["plan"] for k in ("tier", "jobs", "matrix_rig", "matrix_gates")),
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
say(ifs == ["${{ !cancelled() }}"], "T4 the tier job runs on !cancelled(), never always() (%s)" % "; ".join(ifs))
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
say(re.search(r"^  push:\n    branches: \[trunk\]\n", wf, re.M) is not None, "T4 a push runs on trunk alone (a lane's push runs once, as its pull request)")
say(re.search(r"^  schedule:\n(\s+#.*\n)*\s+- cron: ", wf, re.M) is not None, "T4 the nightly runs")
say("    types: [opened, synchronize, reopened, labeled]\n" in wf, "T4 adding a label starts a run")
say(re.search(r"^  workflow_dispatch:\n    inputs:\n      tier:\n", wf, re.M) is not None and "        options: [full, pr]\n" in wf,
    "T4 a dispatch takes the tier input, full or pr")
say(plan(event="push")["matrix"] == ALL_HOSTS, "T4 the full tier's hosts are ubuntu, macOS and windows for both jobs, as before tl28")
say(not re.search(r"^\s*continue-on-error:", wf, re.M), "T4 no continue-on-error (xtask's selftest blesses each one; tl28 adds none)")

# --- T7 the change read from git ----------------------------------------------------
def g_(repo, *args):
    cmd = ["git", "-C", repo, "-c", "user.name=tiers-test", "-c", "user.email=tiers-test@invalid",
           "-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"] + list(args)
    return subprocess.run(cmd, check=True, capture_output=True, text=True).stdout.strip()


def write(repo, rel, text):
    full = os.path.join(repo, rel)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as f:
        f.write(text)


def merged(topic_paths):
    """A repository whose HEAD merges a topic branch (changing topic_paths)
    into a base that moved on meanwhile -> (repo, the topic's head)."""
    repo = tempfile.mkdtemp(prefix="tiers-test-")
    g_(repo, "init", "-q", "-b", "trunk")
    for rel in ("std/a/a.lu", "docs/findings.md", "CHANGELOG.md", "vendor/tools.toml"):
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
    write(repo, "std/a/moved.lu", "the base moved on\n")
    g_(repo, "add", ".")
    g_(repo, "commit", "-q", "-m", "trunk moves")
    g_(repo, "merge", "-q", "--no-ff", "-m", "merge", "topic")
    return repo, head


def dispatched(topic_paths):
    """A clone whose HEAD is a branch off trunk (changing topic_paths), its
    origin/trunk moved on meanwhile -> the clone."""
    up = tempfile.mkdtemp(prefix="tiers-test-up-")
    g_(up, "init", "-q", "-b", "trunk")
    write(up, "std/a/a.lu", "one\n")
    write(up, "docs/findings.md", "one\n")
    g_(up, "add", ".")
    g_(up, "commit", "-q", "-m", "base")
    clone = tempfile.mkdtemp(prefix="tiers-test-clone-")
    subprocess.run(["git", "clone", "-q", up, clone], check=True, capture_output=True)
    g_(clone, "checkout", "-q", "-b", "tl")
    for rel in topic_paths:
        write(clone, rel, "the branch\n")
    g_(clone, "add", ".")
    g_(clone, "commit", "-q", "-m", "branch")
    write(up, "std/a/moved.lu", "trunk moved on\n")
    g_(up, "add", ".")
    g_(up, "commit", "-q", "-m", "trunk moves")
    g_(clone, "fetch", "-q", "origin")
    return up, clone


made = []
try:
    repo, head = merged(["docs/findings.md", "CHANGELOG.md", "docs/a new page.md"])
    made.append(repo)
    expect("T7 a prose-only merge, the base moved on (its std/a/moved.lu is not the pull request's)",
           plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "noop", [])
    expect("T7 the same merge, the head not given", plan(repo=repo, flags=["--merge"], **pr), "noop", [])
    expect("T7 the merge's second parent is not the pull request's head",
           plan(repo=repo, head_sha="0" * 40, flags=["--merge"], **pr), "full", EVERY)
    g_(repo, "checkout", "-q", "topic")
    expect("T7 HEAD is the branch, not a merge commit", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "full", EVERY)
    repo, head = merged(["docs/findings.md", "std/a/a.lu"])
    made.append(repo)
    expect("T7 a merge that changes code and prose", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "pr", BOTH)
    repo, head = merged(["CHANGELOG.md", "somewhere/new.txt"])
    made.append(repo)
    expect("T7 a merge that adds a path no rule names", plan(repo=repo, head_sha=head, flags=["--merge"], **pr), "full", EVERY)
    dp = dict(event="workflow_dispatch", tier="pr", flags=["--merge"])
    up, clone = dispatched(["docs/findings.md"])
    made += [up, clone]
    expect("T7 a dispatched prose-only branch, trunk moved on (its std/a/moved.lu is not the branch's)",
           plan(repo=clone, **dp), "noop", [])
    up, clone = dispatched(["std/a/a.lu", "docs/findings.md"])
    made += [up, clone]
    expect("T7 a dispatched branch that changes code", plan(repo=clone, **dp), "pr", BOTH)
    up, clone = dispatched(["plant/unmapped.txt", "std/a/a.lu"])
    made += [up, clone]
    expect("T7 a dispatched branch that adds a path no rule names (the plant)", plan(repo=clone, **dp), "full", EVERY)
    g_(clone, "checkout", "-q", "-B", "on-trunk", "origin/trunk")
    expect("T7 a dispatch at trunk itself, tier pr (no change to read)", plan(repo=clone, **dp), "full", EVERY)
    lone, _ = merged(["std/a/a.lu"])
    made.append(lone)
    expect("T7 a dispatch with no origin/trunk to read", plan(repo=lone, **dp), "full", EVERY)
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
                   FAKE_LABEL_RC=str(label_rc), REPO="wolffe-lang/wolf-std", WORKFLOW="ci",
                   RUN_URL="https://github.com/wolffe-lang/wolf-std/actions/runs/1")
        r = subprocess.run(["bash", RED], capture_output=True, text=True, env=env)
        calls = open(log).read().splitlines() if os.path.exists(log) else []
        return r.returncode, calls
    finally:
        shutil.rmtree(d, ignore_errors=True)


TITLE = "nightly red: ci on trunk"
rc, calls = nightly_red()
creates = [c for c in calls if c.startswith("issue create")]
say(rc == 0 and len(creates) == 1 and "--title %s" % TITLE in creates[0] and "--label nightly-red" in creates[0]
    and not [c for c in calls if c.startswith("issue comment")],
    "T8 no open issue: one is opened, labelled nightly-red, titled '%s' (%s)" % (TITLE, "; ".join(c[:60] for c in calls)))
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


def naming(paths):
    """{path: ["file:line", ...]}: the lines of xtask/ and tests/ (Rust and
    wolf) with a string literal that is the path or a directory above it."""
    code = [p for p in files if p.startswith(("xtask/", "tests/")) and p.endswith((".rs", ".lu"))]
    names = {}
    for p in paths:
        cand = {p}
        d = os.path.dirname(p)
        while d:
            cand.add(d)
            d = os.path.dirname(d)
        names[p] = cand
    hits = {p: [] for p in paths}
    for c in code:
        with open(os.path.join(ROOT, c), encoding="utf-8", errors="replace") as f:
            for n, line in enumerate(f, 1):
                s = line.lstrip()
                if '"' not in line or s.startswith(("//", "#")):
                    continue
                lits = set(re.findall(r'"\.?/?((?:[A-Za-z0-9_.-]+/)*[A-Za-z0-9_.-]+)/?"', line))
                for p in paths:
                    if lits & names[p]:
                        hits[p].append("%s:%d" % (c, n))
    return hits


files = [f for f in tracked() if f]
if files:
    cls = classes(files)
    prose = sorted(p for p in files if cls.get(p) == "prose")
    unknown = sorted(p for p in files if cls.get(p) == "FULL")
    print("tiers-test: the %d tracked paths that are prose: %s" % (len(prose), " ".join(prose[:12]) + (" ..." if len(prose) > 12 else "")))
    must = ["vendor/upstream/anchors.json", "tests/ledger.toml", "tests/ulp/reference.txt"]
    seen = naming(must)
    exact = {p: [w for w in seen[p] if p in open(os.path.join(ROOT, w.rsplit(":", 1)[0]), encoding="utf-8",
                                                 errors="replace").read().splitlines()[int(w.rsplit(":", 1)[1]) - 1]]
             for p in must}
    say(all(exact[p] for p in must), "T5 the search fires: it finds a line naming each of %s" % ", ".join(
        "%s (%s)" % (p, exact[p][0] if exact[p] else "NOTHING") for p in must))
    hit = {p: w for p, w in naming(prose).items() if w}
    say(not hit, "T5 no line of xtask/ or tests/ names a prose path: %s" % (
        "; ".join("%s at %s" % (p, " ".join(w[:3])) for p, w in sorted(hit.items())) or "none"))
    roots = re.search(r'^const ROOTS: \[&str; \d+\] = \[(.*)\];$', open(os.path.join(ROOT, "xtask/src/fmt_lu.rs"), encoding="utf-8").read(), re.M)
    say(roots is not None and re.findall(r'"([^"]+)"', roots.group(1)) == ["std", "tests"],
        "T5 fmt-lu walks std/ and tests/ alone (%s)" % (roots.group(1) if roots else "ROOTS not found"))
    lib = [p for p in files if p.startswith(("std/", "tests/", "vendor/", "xtask/"))]
    say(len(lib) >= 100 and all(cls.get(p) == "rig gates" for p in lib), "T5 every tracked path under std/, tests/, vendor/ and xtask/ is pr, both jobs (%d paths)" % len(lib))
    if unknown:
        print("tiers-test: WARN T5 %d tracked path(s) no rule names (a pull request touching one runs full): %s"
              % (len(unknown), " ".join(unknown[:20])))
    else:
        print("tiers-test: PASS T5 every one of %d tracked paths has a rule" % len(files))
else:
    say(False, "T5 not a git checkout: the tracked paths cannot be listed")

print("tiers-test: %s" % ("PASS" if fails == 0 else "FAIL (%d)" % fails))
sys.exit(1 if fails else 0)

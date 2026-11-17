#!/usr/bin/env python3
"""Publish the workspace library crates to crates.io in dependency order.

Usage: publish.py <tag>            publish (requires CARGO_REGISTRY_TOKEN)
       publish.py <tag> --order    print the order only, publish nothing

The tag must be `crates_<version>` and match the workspace version. Crates whose
version is already on crates.io are skipped, so a failed run can be re-run.
Waits and retries when crates.io has not yet indexed a dependency or rate-limits us.
"""

import json
import os
import subprocess
import sys
import time
import urllib.error
import urllib.request

USER_AGENT = "prime-rs-publish (github actions)"
RETRIES = 12
WAIT_SECONDS = 60


def workspace_packages():
    meta = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"]))
    pkgs = {}
    for p in meta["packages"]:
        publish = p["publish"]
        if publish == []:  # `publish = false`
            continue
        pkgs[p["name"]] = p
    return pkgs


def publish_order(pkgs):
    """Topological order: a crate comes after its normal and build dependencies that are published too."""
    order, done, visiting = [], set(), set()

    def visit(name):
        if name in done:
            return
        if name in visiting:
            raise SystemExit(f"dependency cycle through {name}")
        visiting.add(name)
        for dep in pkgs[name]["dependencies"]:
            if dep["kind"] in (None, "build") and dep["name"] in pkgs:
                visit(dep["name"])
        visiting.discard(name)
        done.add(name)
        order.append(name)

    for name in sorted(pkgs):
        visit(name)
    return order


def already_published(name, version):
    url = f"https://crates.io/api/v1/crates/{name}/{version}"
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(req, timeout=30):
            return True
    except urllib.error.HTTPError as err:
        if err.code == 404:
            return False
        raise


def publish(name):
    for attempt in range(1, RETRIES + 1):
        result = subprocess.run(["cargo", "publish", "-p", name], capture_output=True, text=True)
        output = result.stdout + result.stderr
        sys.stdout.write(output)
        if result.returncode == 0:
            return
        retryable = any(
            marker in output
            for marker in ("429", "rate limit", "no matching package named", "failed to select a version")
        )
        if not retryable or attempt == RETRIES:
            raise SystemExit(f"publishing {name} failed (attempt {attempt})")
        print(f"{name}: waiting {WAIT_SECONDS}s before retry {attempt + 1}/{RETRIES}", flush=True)
        time.sleep(WAIT_SECONDS)


def main():
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    tag = sys.argv[1]
    order_only = "--order" in sys.argv
    if not tag.startswith("crates_"):
        raise SystemExit(f"tag must look like crates_<version>, got {tag!r}")
    version = tag[len("crates_"):]

    pkgs = workspace_packages()
    versions = {p["version"] for p in pkgs.values()}
    if versions != {version}:
        raise SystemExit(f"tag version {version} does not match workspace versions {sorted(versions)}")

    order = publish_order(pkgs)
    print(f"{len(order)} crates in publish order", flush=True)
    if order_only:
        print("\n".join(order))
        return

    if not os.environ.get("CARGO_REGISTRY_TOKEN"):
        raise SystemExit("CARGO_REGISTRY_TOKEN is not set")

    for name in order:
        if already_published(name, version):
            print(f"{name} {version}: already on crates.io, skipping", flush=True)
            continue
        print(f"publishing {name} {version}", flush=True)
        publish(name)
    print("done", flush=True)


if __name__ == "__main__":
    main()

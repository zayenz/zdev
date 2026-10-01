# Releasing zdev

Push a version tag after running `scripts/check-release.sh vMAJOR.MINOR.PATCH`.
The Release workflow builds and checks the binaries, creates the GitHub release,
then calls `publish-crates.yml` to publish the same tag to crates.io. Prereleases
are excluded from crates.io publishing. A publishing failure fails the Release
workflow; an already published version is skipped when retrying.

## Authentication

Publishing uses [crates.io Trusted Publishing](https://crates.io/docs/trusted-publishing).
In the [zdev crate settings](https://crates.io/crates/zdev/settings), configure
GitHub publishers for repository owner `zayenz`, repository `zdev`, and these
workflow filenames, with no environment:

- `release.yml`, for automatic releases through the reusable workflow.
- `publish-crates.yml`, for manually publishing an existing release.

crates.io verifies the calling workflow filename. No permanent API token or
GitHub repository secret is needed.

## Publish or retry an existing release

Once the workflows are on `main`, run:

```sh
gh workflow run publish-crates.yml --ref main -f tag=v1.6.0
```

This checks out the existing tag, confirms a stable, published GitHub release
and matching Cargo version, verifies the package, and publishes it. Use the
same command with another stable release tag to retry a failed publication.

## Updating the release workflow

Edit `dist-workspace.toml`, then run `scripts/sync-release-workflow.sh`.
The script regenerates `release.yml` and applies the repository's hardening
patch. Run `scripts/sync-release-workflow.sh --check` to verify it is current.

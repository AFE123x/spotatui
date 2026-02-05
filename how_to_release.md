# To create a release

The releases are automated via GitHub actions, using [this configuration file](https://github.com/LargeModGames/spotatui/blob/master/.github/workflows/cd.yml).

The release is triggered by pushing a tag.

1. Bump the version in `Cargo.toml` and run the app to update the `lock` file
1. Update the "Unreleased" header for the new version in the `CHANGELOG`. Use `### Added/Fixed/Changed` headers as appropriate
1. Commit the changes and push them.
1. Create a new tag e.g. `git tag -a v0.7.0` and add the CHANGELOG to the commit body
1. Push the tag `git push --tags`
1. Wait for the build to finish on the [Actions page](https://github.com/LargeModGames/spotatui/actions)
1. This should publish to cargo as well

### Homebrew Packaging

Homebrew publishing is automated via the CD workflow. When you push a new tag:

1. The `publish-homebrew` job downloads the release artifacts
2. Calculates SHA256 checksums for each platform binary
3. Updates the formula in [homebrew-spotatui](https://github.com/LargeModGames/homebrew-spotatui)



TODO: Scoop packaging is not yet set up for spotatui. If you'd like to contribute packaging, PRs are welcome!

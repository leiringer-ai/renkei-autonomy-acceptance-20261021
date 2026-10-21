# Disposable Renkei autonomy acceptance

This repository is a synthetic execution fixture, not a user project.

## Completion requirements

- `successor` returns the next signed 64-bit integer and saturates at `i64::MAX`.
- `label` prefixes arbitrary string input with `item:` and preserves the content.
- All committed contract tests pass through `mise run test`.
- Keep the tests, mise configuration and CI requirements intact.

The two implementations occupy disjoint files and can be assigned to independent peers.
After verified delivery, assess these original requirements before declaring completion.

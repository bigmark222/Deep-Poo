# Extending schema/warehouse and release/versioning

## Adding new fields to schemas/warehouse
- Define schema changes with clear defaults; document in data schema tables.
- Update ETL to populate new fields; adjust validation rules if needed.
- Consider format/version bumps if shard layout changes; document in warehouse pages.
- Add tests covering the new fields (ETL path + loader path).

## Versioning rules
- Bump warehouse version when cacheable transforms, schema, prune config, or ETL code affecting outputs changes.
- Record `code_version` (git SHA) and the transform/prune configs in the manifest.
- Avoid silent changes: update docs and changelog when versions shift.

## Release cadence (suggested)
- Land changes behind tests and docs updates.
- Produce a fresh warehouse build for major schema/transform changes.
- Tag releases when format/version bumps occur.

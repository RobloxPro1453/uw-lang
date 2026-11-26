# uw_atlas

Reads the atlas JSON produced by `py/atlas_dump.py` (Native ops, hooks, ranges, every table entry with name/index/value/source line) and `kit-meta/*.toml`. Read-only data for check, plan and `uw explain`.

Depends on: uw_diag.

Contract status: direction only, not frozen (frozen after D2 + D3; see docs/ARCHITECTURE.md).

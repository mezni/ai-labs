"""Rollback page — repoint ``current_version`` at a stored snapshot.

Read-only except for an explicit, confirmed rollback that calls the same
service as the CLI: ``src/indexing/versioning.py`` (snapshot-exists guardrail).
"""

from __future__ import annotations

import streamlit as st

from src.config import get_settings
from src.indexing.versioning import RollbackError, Versioning


def render() -> None:
    st.title("Rollback")
    st.caption("Point `current_version` back to a stored snapshot.")

    settings = get_settings()
    versioning = Versioning(settings)

    versions = sorted(versioning.registry.versions)
    if not versions:
        st.info("No index versions recorded — run `rag-dataops index` first.")
        return

    current = versioning.registry.current_version
    st.markdown(f"**Current version:** {current or '—'}")

    candidates = [v for v in versions if v != current]
    if not candidates:
        st.info("Nothing to roll back to — only the current version exists.")
        return

    target = st.selectbox("Target snapshot", candidates)

    if st.button("Validate target snapshot"):
        config = versioning.registry.versions[target]
        try:
            count = versioning.validate_snapshot(target, config.collection_name)
            st.success(f"Snapshot {target} is valid and holds {count} chunks.")
        except RollbackError as exc:
            st.error(str(exc))

    st.warning("Rollback is irreversible for the current pointer.")
    if st.button(f"Roll back to {target}", type="primary"):
        try:
            versioning.rollback(target)
        except RollbackError as exc:
            st.error(str(exc))
        else:
            st.success(f"Rolled back to {target}.")


def main() -> None:
    render()


if __name__ == "__main__":
    main()
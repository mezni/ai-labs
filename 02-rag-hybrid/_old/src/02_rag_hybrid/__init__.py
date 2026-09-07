"""Package matching the project name for the wheel build.

The runtime functionality lives in the `rag_hybrid` module (a leading digit in
this package name is not importable via a standard ``from ... import``, which
the generated console-script wrapper relies on).
"""
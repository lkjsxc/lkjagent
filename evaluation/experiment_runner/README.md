# Experiment Runner Modules

## Purpose

Separate campaign orchestration, per-run execution, evidence capture, and strict
input/output helpers while keeping the public runner command small.

- `campaign.py`: build, resume, matrix, escalation, and adoption orchestration.
- `evidence.py`: database exports, manifests, and recursive redaction.
- `io.py`: hashes, source inputs, environment isolation, and subprocess capture.
- `run.py`: fresh-store endpoint probe execution and computed metrics.

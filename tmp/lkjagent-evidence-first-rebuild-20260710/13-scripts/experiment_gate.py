from __future__ import annotations

import csv
import re
from pathlib import Path

from experiment_metrics import check_metrics
from experiment_run_checks import validate_run


HASH = re.compile(r"^sha256:[0-9a-f]{64}$")
REQUIRED_FACTORS = {
    "action-grammar",
    "constrained-output",
    "context-selection",
    "long-work-splitting",
    "recovery-ladder",
    "tool-view",
    "tui-event-model",
}


def safe(root: Path, reference: str) -> Path | None:
    candidate = (root / reference).resolve()
    if root.resolve() not in candidate.parents or candidate.is_symlink():
        return None
    return candidate


def values(path: Path) -> dict[str, str]:
    return dict(
        line.split("\t", 1)
        for line in path.read_text(encoding="utf-8").splitlines()
        if "\t" in line
    )


def check_experiments(
    repo: Path,
    evidence: Path,
    final_configurations: set[str],
    source_commit: str,
    errors: list[str],
) -> list[Path]:
    matrix_path = evidence / "experiment-matrix.tsv"
    adoption_path = evidence / "adoption.tsv"
    if not matrix_path.is_file() or not adoption_path.is_file():
        errors.append("experiment matrix or adoption ledger missing")
        return []
    matrix = list(csv.DictReader(matrix_path.open(encoding="utf-8"), delimiter="\t"))
    adoption = list(csv.DictReader(adoption_path.open(encoding="utf-8"), delimiter="\t"))
    required = {
        "experiment_id",
        "scenario_id",
        "candidate_class",
        "configuration_fingerprint",
        "repeat",
        "evidence_ref",
        "database_fingerprint",
        "tested_commit",
        "factor_families",
    }
    if not matrix or not required.issubset(matrix[0]):
        errors.append("experiment matrix columns missing")
        return []
    cells: dict[tuple[str, str, str], set[str]] = {}
    classes: dict[str, set[str]] = {}
    configurations: dict[str, set[str]] = {}
    references: set[Path] = set()
    database_hashes: set[str] = set()
    runs: list[tuple[Path, str]] = []
    metric_records: list[dict[str, object]] = []
    integrated: set[tuple[str, str]] = set()
    factors: set[str] = set()
    semantic_configurations: set[str] = set()
    configuration_classes: dict[tuple[str, str], set[str]] = {}
    for row in matrix:
        configuration = row["configuration_fingerprint"]
        factors.update(item for item in row["factor_families"].split(",") if item)
        if not HASH.fullmatch(configuration):
            errors.append("experiment configuration fingerprint malformed")
        cell = (row["experiment_id"], row["scenario_id"], configuration)
        cells.setdefault(cell, set()).add(row["repeat"])
        classes.setdefault(row["experiment_id"], set()).add(row["candidate_class"])
        configurations.setdefault(row["experiment_id"], set()).add(configuration)
        configuration_classes.setdefault(
            (row["experiment_id"], configuration), set()
        ).add(row["candidate_class"])
        if row["candidate_class"] == "integrated":
            integrated.add((row["experiment_id"], configuration))
        run = safe(evidence, row["evidence_ref"])
        if run is None or not run.is_dir() or run in references:
            errors.append(f"experiment run escaped, missing, or reused: {row['evidence_ref']}")
            continue
        references.add(run)
        facts = validate_run(repo, run, row, source_commit, errors)
        if facts is None:
            continue
        canonical, database_hash, measured = facts
        if database_hash in database_hashes:
            errors.append(f"experiment database hash reused: {row['evidence_ref']}")
        database_hashes.add(database_hash)
        if canonical:
            semantic_configurations.add(canonical)
        runs.append((run, configuration))
        metric_records.append(
            {
                "experiment": row["experiment_id"],
                "scenario": row["scenario_id"],
                "configuration": configuration,
                "class": row["candidate_class"],
                "reference": row["evidence_ref"],
                "metrics": measured,
            }
        )
    if any(len(repeats) < 3 for repeats in cells.values()):
        errors.append("experiment cell has fewer than three distinct repeats")
    required_classes = {"baseline", "isolated", "integrated"}
    if any(not required_classes.issubset(found) for found in classes.values()):
        errors.append("experiment lacks baseline, isolated, or integrated candidate")
    if any(len(found) < 3 for found in configurations.values()):
        errors.append("experiment compares fewer than three configurations")
    if any(len(found) != 1 for found in configuration_classes.values()):
        errors.append("one configuration is assigned to multiple candidate classes")
    if len(semantic_configurations) < 3:
        errors.append("fewer than three semantically distinct configurations")
    if not REQUIRED_FACTORS.issubset(factors):
        errors.append(f"required experiment factors missing: {sorted(REQUIRED_FACTORS - factors)}")
    decisions: dict[tuple[str, str], str] = {}
    adopted: set[tuple[str, str]] = set()
    for row in adoption:
        key = (row.get("experiment_id", ""), row.get("configuration_fingerprint", ""))
        decision = row.get("decision", "")
        if key in decisions or decision not in {"adopt", "reject", "conditional"}:
            errors.append("duplicate or invalid adoption decision")
        decisions[key] = decision
        if decision == "adopt":
            adopted.add(key)
        reference = safe(evidence, row.get("evidence_ref", ""))
        if reference is None or not reference.is_file() or reference.stat().st_size < 80:
            errors.append("adoption rationale missing, escaped, or empty")
    expected_decisions = {
        (experiment, configuration)
        for experiment, found in configurations.items()
        for configuration in found
    }
    if expected_decisions != decisions.keys():
        errors.append("adoption ledger does not cover exactly every candidate")
    final_pairs = {
        (experiment, configuration)
        for experiment in configurations
        for configuration in final_configurations
    }
    if not final_configurations or not final_pairs.issubset(adopted):
        errors.append("final campaigns do not use an adopted configuration")
    if not adopted.issubset(integrated):
        errors.append("a non-integrated candidate was adopted")
    check_metrics(metric_records, adopted, errors)
    adopted_runs = [
        run
        for run, configuration in runs
        if any(key[1] == configuration for key in adopted)
    ]
    if any(values(run / "result.tsv").get("source_commit") != source_commit for run in adopted_runs):
        errors.append("adopted candidates were not rerun on frozen source")
    return adopted_runs

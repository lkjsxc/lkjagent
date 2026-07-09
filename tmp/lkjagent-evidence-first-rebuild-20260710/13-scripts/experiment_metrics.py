from __future__ import annotations

import csv
import math
import statistics
from pathlib import Path


HARD_ZERO = {
    "duplicate_effect_count",
    "false_close_count",
    "lost_owner_byte_count",
    "path_escape_count",
    "tui_invariant_failure_count",
    "unsupported_claim_count",
}
HARD_ONE = {"required_check_pass_rate", "required_source_recall"}


def metrics(path: Path) -> dict[str, float]:
    rows = list(csv.DictReader(path.open(encoding="utf-8"), delimiter="\t"))
    if not rows or not {"metric", "value"}.issubset(rows[0]):
        raise ValueError(f"metrics columns missing: {path}")
    result = {row["metric"]: float(row["value"]) for row in rows}
    if len(result) != len(rows):
        raise ValueError(f"duplicate metric: {path}")
    if "primary_task_success" not in result:
        raise ValueError(f"primary task metric missing: {path}")
    if any(not math.isfinite(value) or value < 0 for value in result.values()):
        raise ValueError(f"metric is negative or non-finite: {path}")
    rates = {key for key in result if key.endswith("_rate")} | {
        "primary_task_success",
        "protected_regression_ratio",
        "required_source_recall",
    }
    if any(result[key] > 1 for key in rates):
        raise ValueError(f"rate metric is above one: {path}")
    return result


def hard_floors(data: dict[str, float], reference: str, errors: list[str]) -> None:
    required = HARD_ZERO | HARD_ONE | {
        "first_pass_admission_rate",
        "first_pass_parse_rate",
        "primary_task_success",
        "protected_regression_ratio",
    }
    if required - data.keys():
        errors.append(f"hard metrics missing: {reference}")
        return
    if any(data[item] != 0 for item in HARD_ZERO):
        errors.append(f"hard zero floor failed: {reference}")
    if any(data[item] != 1 for item in HARD_ONE):
        errors.append(f"hard one floor failed: {reference}")
    if data["first_pass_parse_rate"] < 0.98 or data["first_pass_admission_rate"] < 0.97:
        errors.append(f"protocol floor failed: {reference}")
    if data["protected_regression_ratio"] > 0.05:
        errors.append(f"protected regression floor failed: {reference}")


def check_metrics(
    records: list[dict[str, object]], adopted: set[tuple[str, str]], errors: list[str]
) -> None:
    groups: dict[tuple[str, str, str], list[dict[str, object]]] = {}
    valid_records: list[dict[str, object]] = []
    for record in records:
        data = record["metrics"]
        if not isinstance(data, dict) or "primary_task_success" not in data:
            errors.append(f"primary experiment metric unavailable: {record['reference']}")
            continue
        valid_records.append(record)
        key = (
            str(record["experiment"]),
            str(record["scenario"]),
            str(record["configuration"]),
        )
        groups.setdefault(key, []).append(record)
        if (str(record["experiment"]), str(record["configuration"])) in adopted:
            hard_floors(
                record["metrics"],  # type: ignore[arg-type]
                str(record["reference"]),
                errors,
            )
    for key, items in groups.items():
        outcomes = {float(item["metrics"]["primary_task_success"]) for item in items}  # type: ignore[index]
        if len(outcomes) > 1 and len(items) < 5:
            errors.append(f"unstable experiment cell needs five repeats: {key}")
    experiment_scenarios = {(key[0], key[1]) for key in groups}
    for experiment, scenario in experiment_scenarios:
        baseline = [
            item
            for item in valid_records
            if item["experiment"] == experiment
            and item["scenario"] == scenario
            and item["class"] == "baseline"
        ]
        winner_configs = {config for owner, config in adopted if owner == experiment}
        if not baseline or not winner_configs:
            errors.append(f"baseline or adopted comparison missing: {experiment}/{scenario}")
            continue
        before = statistics.mean(float(item["metrics"]["primary_task_success"]) for item in baseline)  # type: ignore[index]
        for configuration in winner_configs:
            winners = [
                item for item in valid_records
                if item["experiment"] == experiment and item["scenario"] == scenario
                and item["configuration"] == configuration
            ]
            if not winners:
                errors.append(f"adopted cell missing: {experiment}/{scenario}/{configuration}")
                continue
            after = statistics.mean(float(item["metrics"]["primary_task_success"]) for item in winners)  # type: ignore[index]
            success_gain = after - before >= 0.10
            efficiency = False
            if after == 1 and before == 1:
                for metric in ("rendered_tokens", "endpoint_calls", "recovery_seconds"):
                    if all(metric in item["metrics"] for item in baseline + winners):  # type: ignore[operator]
                        old = statistics.median(float(item["metrics"][metric]) for item in baseline)  # type: ignore[index]
                        new = statistics.median(float(item["metrics"][metric]) for item in winners)  # type: ignore[index]
                        efficiency |= old > 0 and new <= old * 0.85
            if not success_gain and not efficiency:
                errors.append(
                    f"adoption improvement threshold failed: {experiment}/{scenario}/{configuration}"
                )

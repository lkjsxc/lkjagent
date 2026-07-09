from __future__ import annotations

import sqlite3


def count(db: sqlite3.Connection, sql: str) -> int:
    return int(db.execute(sql).fetchone()[0])


def linkage_check(
    db: sqlite3.Connection, required_checks: set[str], errors: list[str]
) -> None:
    passed = {
        row[0] for row in db.execute("SELECT id FROM checks WHERE current!=0 AND passed!=0")
    }
    if not required_checks.issubset(passed):
        errors.append("required anchored scenario checks did not pass")
    if count(
        db,
        "SELECT COUNT(*) FROM obligations o LEFT JOIN checks c "
        "ON c.id=o.satisfied_by_check_id WHERE o.required!=0 "
        "AND lower(o.state)='satisfied' "
        "AND (c.id IS NULL OR c.passed=0 OR c.current=0 OR c.matter_id!=o.matter_id)",
    ):
        errors.append("satisfied obligation lacks its current matter-scoped check")
    if count(
        db,
        "SELECT COUNT(*) FROM (SELECT a.id,COUNT(e.id) AS n FROM tool_admissions a "
        "LEFT JOIN effect_journal e ON e.admission_id=a.id "
        "WHERE lower(a.status)='accepted' AND a.effectful!=0 GROUP BY a.id HAVING n!=1)",
    ):
        errors.append("accepted effectful admission does not have exactly one effect")
    if count(
        db,
        "SELECT COUNT(*) FROM tool_admissions a JOIN effect_journal e "
        "ON e.admission_id=a.id WHERE lower(a.status)!='accepted'",
    ):
        errors.append("rejected admission has an effect")
    if count(
        db,
        "SELECT COUNT(*) FROM (SELECT e.id,COUNT(o.id) AS n FROM effect_journal e "
        "LEFT JOIN observations o ON o.effect_id=e.id GROUP BY e.id HAVING n!=1)",
    ):
        errors.append("effect does not settle to exactly one observation")
    if count(
        db,
        "SELECT COUNT(*) FROM effect_journal WHERE lower(state) IN ('prepared','applying')",
    ):
        errors.append("evidence snapshot contains an unsettled effect")
    if count(
        db,
        "SELECT COUNT(*) FROM (SELECT o.id FROM operations o "
        "LEFT JOIN tool_admissions a ON a.operation_id=o.id "
        "WHERE o.requires_admission!=0 GROUP BY o.id HAVING COUNT(a.id)!=1)",
    ):
        errors.append("effect operation does not have exactly one admission")
    if count(
        db,
        "SELECT COUNT(*) FROM observations WHERE lower(status)='failed' "
        "AND lower(attempt_outcome)='ok'",
    ):
        errors.append("failed native tool remained an Ok attempt")
    if count(db, "SELECT COUNT(*) FROM provider_exchanges") < 3:
        errors.append("fewer than three provider exchanges")
    if count(db, "SELECT COUNT(*) FROM context_frames") == 0:
        errors.append("no context frames")
    if count(
        db,
        "SELECT COUNT(*) FROM provider_exchanges p LEFT JOIN context_frames c "
        "ON c.decision_id=p.decision_id WHERE c.id IS NULL",
    ):
        errors.append("provider exchange lacks decision context frame")
    if count(
        db,
        "SELECT COUNT(*) FROM runtime_decisions WHERE tool_count>4 "
        "OR prompt_tokens>prompt_token_cap OR semantic_duplicate_count!=0 "
        "OR harness_json_count!=0 OR unresolved_material_conflict_count!=0",
    ):
        errors.append("live decision violates tool, budget, dedup, JSON, or conflict contract")
    if count(
        db,
        "SELECT COUNT(*) FROM (SELECT operation_id,prompt_fingerprint,"
        "tool_view_fingerprint,budget_fingerprint,fault_signature,strategy,"
        "external_condition_fingerprint FROM failure_lineages GROUP BY 1,2,3,4,5,6,7 "
        "HAVING COUNT(*)>1)",
    ):
        errors.append("failure lineage repeated without a changed causal condition")

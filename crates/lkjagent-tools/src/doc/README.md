# Document Tool Source

## Purpose

This directory holds the semantic document scaffold and audit implementation.

## Table of Contents

- [audit.rs](audit.rs): deterministic document topology audit.
- [audit_report.rs](audit_report.rs): audit lane and next-action rendering.
- [body.rs](body.rs): generated README and leaf Markdown templates.
- [content_audit.rs](content_audit.rs): content-artifact scaffold-only checks.
- [content_signals.rs](content_signals.rs): banned generated text and state markers.
- [fit.rs](fit.rs): exact-count group fitting helpers.
- [model.rs](model.rs): scaffold input, plan, files, and profiles.
- [names.rs](names.rs): semantic names, joins, and forbidden-name checks.
- [profile.rs](profile.rs): pure profile selection and plan generation.
- [profile_builders.rs](profile_builders.rs): generic profile file constructors.
- [repeated_content.rs](repeated_content.rs): repeated generated-body detection.
- [roles.rs](roles.rs): semantic fallback role names for requested counts.
- [semantic_seed.rs](semantic_seed.rs): lkjagent seed planner.
- [semantic_seed_body.rs](semantic_seed_body.rs): implementation seed page bodies.
- [semantic_seed_domain.rs](semantic_seed_domain.rs): domain seed page bodies.
- [semantic_seed_extra.rs](semantic_seed_extra.rs): project and relation seed bodies.
- [semantic_seed_select.rs](semantic_seed_select.rs): seed scope selection.
- [semantic_workspace.rs](semantic_workspace.rs): relation-first workspace seed.
- [semantic_workspace_body.rs](semantic_workspace_body.rs): workspace seed page bodies.
- [semantic_workspace_readme.rs](semantic_workspace_readme.rs): workspace README bodies.
- [semantic_workspace_terms.rs](semantic_workspace_terms.rs): owner term extraction for seeds.
- [shape_profiles.rs](shape_profiles.rs): built-in semantic scaffold shapes.
- [shapes.rs](shapes.rs): scaffold profile selection.
- [write.rs](write.rs): filesystem effect boundary for scaffold plans.

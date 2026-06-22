# Document Tool Source

## Purpose

This directory holds the semantic document scaffold and audit implementation.

## Table of Contents

- [audit.rs](audit.rs): deterministic document topology audit.
- [body.rs](body.rs): generated README and leaf Markdown templates.
- [content_audit.rs](content_audit.rs): content-artifact scaffold-only checks.
- [fit.rs](fit.rs): exact-count group fitting helpers.
- [model.rs](model.rs): scaffold input, plan, files, and profiles.
- [names.rs](names.rs): semantic names, joins, and forbidden-name checks.
- [profile.rs](profile.rs): pure profile selection and plan generation.
- [roles.rs](roles.rs): semantic fallback role names for requested counts.
- [shape_profiles.rs](shape_profiles.rs): built-in semantic scaffold shapes.
- [shapes.rs](shapes.rs): scaffold profile selection.
- [write.rs](write.rs): filesystem effect boundary for scaffold plans.

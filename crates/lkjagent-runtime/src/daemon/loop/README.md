# Daemon Loop Helpers

## Purpose

This directory owns the resident poll loop, endpoint call path, startup, owner
queue delivery, idle maintenance opening, and maintenance wait guard.

## Table of Contents

- [runner.rs](runner.rs): resident poll loop and effect interpretation.
- [startup.rs](startup.rs): seed copying and prefix input loading.
- [endpoint.rs](endpoint.rs): endpoint completion and oversize handling.
- [endpoint_log_json.rs](endpoint_log_json.rs): provider exchange JSON rendering helpers.
- [endpoint_logging.rs](endpoint_logging.rs): provider exchange file and store writers.
- [owner_delivery.rs](owner_delivery.rs): queue delivery and owner step opening.
- [idle.rs](idle.rs): automatic idle maintenance cycle opening.
- [maintenance_wait.rs](maintenance_wait.rs): maintenance ask auto-close guard.
- [endpoint_runtime_effect.rs](endpoint_runtime_effect.rs): endpoint runtime effect source module.

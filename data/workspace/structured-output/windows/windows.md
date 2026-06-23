# Windows

## Concrete Record

This document describes the Windows platform, including system compatibility, deployment options, and verification evidence tied to this exact path.

## Examples And Checks

Example one names a path, an invariant, and the command or audit that proves it. Example two names a failure mode, the repair owner, and the evidence needed before completion.

**Path:** `structured-output/windows/windows.md`  
**Invariant:** Windows documentation includes system compatibility and deployment options.  
**Check:** `grep -c "version" structured-output/windows/windows.md` should return at least 2.

Example two names a failure mode, the repair owner, and the evidence needed before completion.  
**Failure Mode:** Missing Windows API reference or platform-specific configuration documentation.  
**Repair Owner:** Documentation team.  
**Evidence Needed:** Platform compatibility table with supported versions documented.
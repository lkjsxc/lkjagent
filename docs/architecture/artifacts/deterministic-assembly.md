# Deterministic Assembly

## Purpose

Define how approved content atoms become final large artifact files without a
model-authored giant write.

## Contract

Assembly is a daemon-owned step. It reads approved atom files, preserves their
order, and writes the exact final target only after atom readiness passes. The
assembly step must be deterministic: the same atom set, root, and target path
produce the same output bytes.

Assembly records evidence that names:

- root;
- target path;
- source atom paths;
- assembled word count;
- readiness status;
- completion gate result.

## Manuscript Chapters

For manuscripts, scene atoms may be assembled into chapter paths under
`manuscript/`. The chapter file is the readiness evidence. Scene-only output is
not completion evidence until the chapter path exists and the manuscript audit
counts its real prose words.

## Restrictions

- Do not assemble from weak, missing, or scaffold-only atoms.
- Do not write outside the requested root or exact owner target path.
- Do not use a direct endpoint manuscript as daemon completion evidence.
- Do not close while assembly evidence is missing.

## Status

contract active; manuscript scene assembly is implemented in the daemon path
when scene atoms are used.

# Establish reproducible reference provenance and regeneration

Type: research
Status: resolved
Blocked by: none

## Question

What exact generation environment, commands, rcParams/layout operations, fonts, backend details, and metadata are required to reproducibly regenerate and verify checked-in Matplotlib references from commit `bde111fb4ee0023852b48d48f14436647400663d` without making ordinary Rust CI depend on Python?

The answer must audit the existing `scripts/mpl_reference.py` and goldens, resolve their currently undocumented environment provenance, distinguish library defaults from explicit fixture configuration, define the metadata to store with references, and identify known platform/font variance that fidelity calibration must accommodate.

## Answer

Resolved in [reference provenance and regeneration](../research/reference-provenance-and-regeneration.md).

The checked-in PNGs identify Matplotlib 3.10.9 and do not record enough
environment data to reconstruct their original builder, so they are legacy
evidence rather than valid references for the pinned adjacent commit. The
release oracle must be regenerated explicitly with a hash-locked Python
builder, the exact clean Matplotlib checkout, Agg, local FreeType 2.14.3, the
packaged and hash-verified DejaVu Sans font, a fresh config/cache, fixed locale
and environment, reset rc defaults plus enumerated fixture overrides, explicit
tight-layout/save settings, and a checked-in corpus manifest. Regeneration is
a maintainer workflow; ordinary Rust CI consumes the committed PNGs and never
depends on Python. Decoded RGBA reproducibility is required across clean runs;
byte equality is required only on the canonical locked builder because PNG
encoding may vary independently of pixels.

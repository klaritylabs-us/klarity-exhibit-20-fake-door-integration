# Fake-door FHIR Bundle integration — Rust

Small library and CLI that sanity-check **FHIR Bundle** JSON used in **fake-door** or stub integrations before demos or contract tests: every **`entry`** must carry a **`resource`**, with warnings when **`fullUrl`** is missing — the truncation pattern that looks fine in Postman but breaks reference graphs in real clients.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 20** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [FHIR Bundle](http://hl7.org/fhir/R4/bundle.html)

## What it checks

- `resourceType` is **`Bundle`**
- **`entry`** array present
- Each entry has non-null **`resource`** (errors flag “fake door / truncation”)
- Warning per entry if **`fullUrl`** is missing or empty

| Problem | What breaks |
|--------|-------------|
| **Placeholder entries** | UI shows success; client resolves `#` references to nothing |
| **Missing fullUrl** | Bundle imports fail cross-resource linking |
| **Empty Bundle** | Contract test passes count check but ships no data |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-fake-door -- examples/valid-bundle.json
cargo run --bin validate-fake-door -- examples/invalid-bundle.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-fake-door -- examples/valid-bundle.json
```

**Pass** (`examples/valid-bundle.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-bundle.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "entry[0]",
    "detail": "missing resource — fake door / truncation",
    "rule_id": "missing-resource"
  }
]
```


## Layout

- [docs/architecture.md](./docs/architecture.md) — fake-door Bundle pattern
- `src/validate.rs` — rules
- `examples/` — valid and broken Bundle payloads for tests and demos

## Inferno vs this repo

Inferno exercises **real endpoint behavior** against certification scenarios. This repo catches **stub Bundles** — incomplete entries your mock server returns during sales or integration dry runs.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

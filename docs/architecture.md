# Architecture

```text
Stub / mock FHIR server (demo, contract test, sales env)
  → Bundle.json response
       → validate_fake_door_bundle()
            · each entry.resource present
            · fullUrl warnings
  → Vec<ValidationIssue>
```

```text
  Postman "200 OK"          Real client (SPA, bulk importer)
        │                              │
        ▼                              ▼
   Bundle looks full            Resolves references
   entry count > 0              needs fullUrl + resource body
        │                              │
        └──────── fake door ───────────┘
              (success signal, empty graph)
```

| Pattern | Symptom |
|---------|---------|
| `entry` without `resource` | UI shows data; client gets null resources |
| Missing `fullUrl` | Bundle import / graph walk fails |
| Empty `entry` | Contract test passes count; no clinical content |

Capstone exhibit: complements conformance tools by catching **integration dishonesty** before production cutover.

Out of scope: full reference integrity across server, `_history`, or authenticated queries.

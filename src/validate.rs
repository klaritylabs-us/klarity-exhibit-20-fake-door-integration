use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_bundle_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Bundle") {
        r.push(ValidationIssue::error("resourceType", "expected Bundle", "bundle-type"));
        return Ok(r);
    }
    let entries = v.get("entry").and_then(|e| e.as_array());
    if entries.is_none() {
        r.push(ValidationIssue::error("entry", "bundle needs entries", "bundle-entry"));
        return Ok(r);
    }
    for (i, entry) in entries.unwrap().iter().enumerate() {
        let res = entry.get("resource");
        if res.is_none() || res.is_some_and(|r| r.is_null()) {
            r.push(ValidationIssue::error(format!("entry[{}]", i), "missing resource — fake door / truncation", "missing-resource"));
        }
        if entry.get("fullUrl").and_then(|u| u.as_str()).is_none_or(|s| s.is_empty()) {
            r.push(ValidationIssue::warning(format!("entry[{}].fullUrl", i), "broken reference graph if fullUrl absent", "fullurl"));
        }
    }
    Ok(r)
}

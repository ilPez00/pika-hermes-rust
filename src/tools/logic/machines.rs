//! Philosophical-machine tools — expose the physis_pro reducers to the agent.
//!
//! Each tool is a thin `BaseTool` over a pure reducer in `physis_pro::machines`:
//! parse JSON params → run the reducer → return its analysis as text the agent
//! can act on. Sync + dependency-light (no engine round-trip needed for the
//! reduction itself).

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::tools::types::{BaseTool, ToolParameter, ToolResult};

use physis_pro::machines::{chronos, elenchus, harmony, lex, polis, soma, techne};

fn arr(params: &HashMap<String, Value>, key: &str) -> Result<Value, ToolResult> {
    match params.get(key) {
        Some(v) => Ok(v.clone()),
        None => Err(ToolResult::error(&format!("missing required param `{key}`"))),
    }
}

fn parse<T: for<'de> Deserialize<'de>>(v: Value) -> Result<T, ToolResult> {
    serde_json::from_value(v).map_err(|e| ToolResult::error(&format!("bad input: {e}")))
}

// ── chronos ─────────────────────────────────────────────────────────────────

pub struct ChronosTool;

#[derive(Deserialize)]
struct EventIn {
    label: String,
    /// RFC3339 timestamp, e.g. "2026-06-07T09:00:00Z".
    at: String,
}

impl BaseTool for ChronosTool {
    fn name(&self) -> String { "chronos".into() }
    fn display_name(&self) -> String { "Chronos (temporal machine)".into() }
    fn description(&self) -> String {
        "Find recurring rhythms in timestamped events and report what's due soon or slipping. \
         Param `events`: [{label, at(RFC3339)}].".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("events", "array", "List of {label, at} events", true)]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<EventIn> = match arr(&params, "events").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let mut events = Vec::new();
        for e in raw {
            match chrono::DateTime::parse_from_rfc3339(&e.at) {
                Ok(dt) => events.push(chronos::TimedEvent { label: e.label, at: dt.with_timezone(&chrono::Utc) }),
                Err(err) => return ToolResult::error(&format!("bad timestamp `{}`: {err}", e.at)),
            }
        }
        let rhythms = chronos::detect_rhythms(&events);
        let now = chrono::Utc::now();
        let mut out = format!("{} rhythms:\n", rhythms.len());
        for r in &rhythms {
            out += &format!("  {} every {:.1}d (conf {:.2}, next {})\n", r.label, r.period_days, r.confidence, r.next_due.format("%Y-%m-%d"));
        }
        let slip = chronos::slipping(&rhythms, now, 0.5);
        if !slip.is_empty() {
            out += "slipping: ";
            out += &slip.iter().map(|r| r.label.as_str()).collect::<Vec<_>>().join(", ");
        }
        ToolResult::success(&out)
    }
}

// ── techne ──────────────────────────────────────────────────────────────────

pub struct TechneTool;

#[derive(Deserialize)]
struct FileIn {
    path: String,
    #[serde(default)]
    lines: Vec<String>,
}

impl BaseTool for TechneTool {
    fn name(&self) -> String { "techne".into() }
    fn display_name(&self) -> String { "Techne (codebase machine)".into() }
    fn description(&self) -> String {
        "Cluster source files into capabilities and trace dependency impact. \
         Params: `files` [{path, lines[]}], optional `impact` (module name).".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("files", "array", "List of {path, lines}", true),
            ToolParameter::new("impact", "string", "Module to trace impact of", false),
        ]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<FileIn> = match arr(&params, "files").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let files: Vec<techne::SourceFile> =
            raw.into_iter().map(|f| techne::SourceFile { path: f.path, lines: f.lines }).collect();
        let mut out = String::from("capabilities:\n");
        for c in techne::classify_capabilities(&files) {
            out += &format!("  {}: {}\n", c.name, c.files.join(", "));
        }
        let edges = techne::dependency_edges(&files);
        if let Some(Value::String(target)) = params.get("impact") {
            let impact = techne::impact_of(&edges, target);
            out += &format!("impact of `{}`: {}\n", target, impact.join(", "));
        }
        ToolResult::success(&out)
    }
}

// ── soma ────────────────────────────────────────────────────────────────────

pub struct SomaTool;

#[derive(Deserialize)]
struct SessionIn {
    #[serde(default)]
    day: u32,
    #[serde(default)]
    activity: String,
    load: f32,
    recovery: f32,
}

impl BaseTool for SomaTool {
    fn name(&self) -> String { "soma".into() }
    fn display_name(&self) -> String { "Soma (body machine)".into() }
    fn description(&self) -> String {
        "Classify training sessions (effort/recovery/overreaching) and detect drift toward burnout. \
         Param `sessions`: [{day, activity, load(0-1), recovery(0-1)}].".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("sessions", "array", "List of {day, activity, load, recovery}", true)]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<SessionIn> = match arr(&params, "sessions").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let sessions: Vec<soma::Session> = raw.into_iter()
            .map(|s| soma::Session { day: s.day, activity: s.activity, load: s.load, recovery: s.recovery })
            .collect();
        let mut out = String::new();
        for s in &sessions {
            out += &format!("  day {}: {:?}\n", s.day, soma::classify(s));
        }
        let d = soma::drift(&sessions, sessions.len().min(6).max(2));
        out += &format!("drift: {:+.2} ({})", d, if d > 0.5 { "overreaching risk" } else { "stable" });
        ToolResult::success(&out)
    }
}

// ── elenchus ────────────────────────────────────────────────────────────────

pub struct ElenchusTool;

#[derive(Deserialize)]
struct StatementIn {
    id: String,
    /// "claim" | "warrant" | "rebuttal"
    kind: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    target: Option<String>,
}

impl BaseTool for ElenchusTool {
    fn name(&self) -> String { "elenchus".into() }
    fn display_name(&self) -> String { "Elenchus (dialogue machine)".into() }
    fn description(&self) -> String {
        "Evaluate an argument: which claims are supported, contested, or unsupported. \
         Param `statements`: [{id, kind(claim|warrant|rebuttal), text, target}].".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("statements", "array", "List of {id, kind, text, target}", true)]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<StatementIn> = match arr(&params, "statements").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let mut statements = Vec::new();
        for s in raw {
            let kind = match s.kind.to_lowercase().as_str() {
                "claim" => elenchus::Kind::Claim,
                "warrant" => elenchus::Kind::Warrant,
                "rebuttal" => elenchus::Kind::Rebuttal,
                other => return ToolResult::error(&format!("unknown kind `{other}`")),
            };
            statements.push(elenchus::Statement { id: s.id, kind, text: s.text, target: s.target });
        }
        let mut out = String::new();
        for (id, st) in elenchus::evaluate(&statements) {
            out += &format!("  {}: {:?}\n", id, st);
        }
        out += &format!("press: {:?}\nrecalibrate: {:?}", elenchus::unsupported_claims(&statements), elenchus::contested_claims(&statements));
        ToolResult::success(&out)
    }
}

// ── lex ─────────────────────────────────────────────────────────────────────

pub struct LexTool;

#[derive(Deserialize)]
struct ClauseIn {
    id: String,
    /// "obligation" | "right" | "condition"
    kind: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    requires: Vec<String>,
}

impl BaseTool for LexTool {
    fn name(&self) -> String { "lex".into() }
    fn display_name(&self) -> String { "Lex (contract machine)".into() }
    fn description(&self) -> String {
        "Analyze obligations: breach paths if a condition fails, and unhandled contingencies. \
         Params: `clauses` [{id, kind(obligation|right|condition), text, requires[]}], optional `failed` [condition ids].".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("clauses", "array", "List of {id, kind, text, requires}", true),
            ToolParameter::new("failed", "array", "Failed condition ids", false),
        ]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<ClauseIn> = match arr(&params, "clauses").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let mut clauses = Vec::new();
        for c in raw {
            let kind = match c.kind.to_lowercase().as_str() {
                "obligation" => lex::Kind::Obligation,
                "right" => lex::Kind::Right,
                "condition" => lex::Kind::Condition,
                other => return ToolResult::error(&format!("unknown kind `{other}`")),
            };
            clauses.push(lex::Clause { id: c.id, kind, text: c.text, requires: c.requires });
        }
        let failed: Vec<String> = params.get("failed")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();
        let mut out = format!("always due: {:?}\n", lex::unconditional_obligations(&clauses));
        out += &format!("breach if {:?} fail: {:?}\n", failed, lex::breach_paths(&clauses, &failed));
        out += &format!("unhandled contingencies: {:?}", lex::dangling_conditions(&clauses));
        ToolResult::success(&out)
    }
}

// ── harmony ─────────────────────────────────────────────────────────────────

pub struct HarmonyTool;

impl BaseTool for HarmonyTool {
    fn name(&self) -> String { "harmony".into() }
    fn display_name(&self) -> String { "Harmony (music machine)".into() }
    fn description(&self) -> String {
        "Reduce a melody to key-invariant intervals + recurring motifs. \
         Params: `notes` [int MIDI pitches], optional `motif_len` (default 3).".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("notes", "array", "MIDI pitch integers", true),
            ToolParameter::new("motif_len", "number", "Motif interval-run length", false),
        ]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let notes: Vec<i32> = match arr(&params, "notes").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let len = params.get("motif_len").and_then(|v| v.as_u64()).unwrap_or(3) as usize;
        let mut out = format!("intervals: {:?}\nmotifs:\n", harmony::intervals(&notes));
        for (seq, count) in harmony::motifs(&notes, len) {
            out += &format!("  {:?} x{}\n", seq, count);
        }
        ToolResult::success(&out)
    }
}

// ── polis ───────────────────────────────────────────────────────────────────

pub struct PolisTool;

#[derive(Deserialize)]
struct InteractionIn {
    from: String,
    to: String,
    weight: f32,
}

impl BaseTool for PolisTool {
    fn name(&self) -> String { "polis".into() }
    fn display_name(&self) -> String { "Polis (social machine)".into() }
    fn description(&self) -> String {
        "Aggregate directed interactions into ties with reciprocity; flag strengthening vs fading. \
         Param `interactions`: [{from, to, weight}].".into()
    }
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![ToolParameter::new("interactions", "array", "List of {from, to, weight}", true)]
    }
    fn execute(&self, params: HashMap<String, Value>) -> ToolResult {
        let raw: Vec<InteractionIn> = match arr(&params, "interactions").and_then(parse) { Ok(v) => v, Err(e) => return e };
        let interactions: Vec<polis::Interaction> = raw.into_iter()
            .map(|i| polis::Interaction { from: i.from, to: i.to, weight: i.weight })
            .collect();
        let ties = polis::ties(&interactions);
        let mut out = String::from("ties:\n");
        for t in &ties {
            out += &format!("  {}-{}: strength {:.1} recip {:.2}\n", t.a, t.b, t.strength, t.reciprocity);
        }
        let fade = polis::fading(&ties, 0.5);
        if !fade.is_empty() {
            out += "fading: ";
            out += &fade.iter().map(|t| format!("{}-{}", t.a, t.b)).collect::<Vec<_>>().join(", ");
        }
        ToolResult::success(&out)
    }
}

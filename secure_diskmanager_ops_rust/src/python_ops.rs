//! Std-only Rust equivalents for the tiny Python library demo scripts.
//!
//! These are not line-for-line ports of third-party Python packages. They are
//! compact, dependency-light Rust operators that preserve the demo-level
//! behaviours: small date/time helpers, nested dictionaries, HTML text pulls,
//! chart/string generation, fake data, validation, fuzzy matching, geometry,
//! retry loops, simple matrix maths, rough PDF text/page inspection, and file
//! watching.

use crate::error::{Result, SdmError};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Names of the Python demo files represented by this module.
pub fn represented_python_demos() -> &'static [&'static str] {
    &[
        "arrow_demo.py",
        "benedict_demo.py",
        "bs4_demo.py",
        "cutecharts_demo.py",
        "dataclasses_demo.py",
        "datashader_demo.py",
        "dynaconf_demo.py",
        "faker_demo.py",
        "fastapi_demo.py",
        "flask_demo.py",
        "fuzzywuzzy_demo.py",
        "gradio_demo.py",
        "holoviews_demo.py",
        "httpx_demo.py",
        "humanize_demo.py",
        "icecream_demo.py",
        "lightningchart_demo.py",
        "logging_demo.py",
        "loguru_demo.py",
        "openai_demo.py",
        "orjson_demo.py",
        "pandas_demo.py",
        "pdfplumber_demo.py",
        "pendulum_demo.py",
        "pillow_demo.py",
        "playwright_demo.py",
        "plotext_demo.py",
        "prefect_demo.py",
        "pydantic_demo.py",
        "pyecharts_demo.py",
        "pyinputplus_demo.py",
        "pymupdf_demo.py",
        "pytesseract_demo.py",
        "pywaffle_demo.py",
        "pywhat_demo.py",
        "redirect_stdout_demo.py",
        "requests_demo.py",
        "rich_demo.py",
        "ruff_demo.py",
        "schedule_demo.py",
        "selenium_demo.py",
        "sentence_transformers_demo.py",
        "shapely_demo.py",
        "tenacity_demo.py",
        "torch_tensor_demo.py",
        "typer_demo.py",
        "watchdog_demo.py",
    ]
}

// ---------------------------------------------------------------------------
// arrow_demo.py / pendulum_demo.py / humanize_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleDateTime {
    pub unix_seconds: i64,
    pub offset_minutes: i32,
}

impl SimpleDateTime {
    pub fn now_with_offset(offset_minutes: i32) -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| SdmError::InvalidInput("system clock is before Unix epoch".to_string()))?;
        Ok(Self { unix_seconds: now.as_secs() as i64, offset_minutes })
    }

    pub fn tokyo_now() -> Result<Self> {
        Self::now_with_offset(9 * 60)
    }

    pub fn add_seconds(self, seconds: i64) -> Self {
        Self { unix_seconds: self.unix_seconds.saturating_add(seconds), offset_minutes: self.offset_minutes }
    }

    pub fn add_minutes(self, minutes: i64) -> Self {
        self.add_seconds(minutes.saturating_mul(60))
    }

    pub fn add_hours(self, hours: i64) -> Self {
        self.add_seconds(hours.saturating_mul(3_600))
    }

    pub fn add_days(self, days: i64) -> Self {
        self.add_seconds(days.saturating_mul(86_400))
    }

    pub fn humanize_since(self, reference: Self) -> String {
        let delta = (self.unix_seconds as i128) - (reference.unix_seconds as i128);
        human_duration(delta.unsigned_abs().min(u64::MAX as u128) as u64, delta >= 0)
    }

    pub fn iso8601(self) -> String {
        let local_seconds = self.unix_seconds.saturating_add((self.offset_minutes as i64) * 60);
        let days = div_floor(local_seconds, 86_400);
        let seconds_of_day = local_seconds - days * 86_400;
        let (year, month, day) = civil_from_days(days);
        let hour = seconds_of_day / 3_600;
        let minute = (seconds_of_day % 3_600) / 60;
        let second = seconds_of_day % 60;
        let sign = if self.offset_minutes >= 0 { '+' } else { '-' };
        let abs_offset = self.offset_minutes.unsigned_abs();
        let oh = abs_offset / 60;
        let om = abs_offset % 60;
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}{sign}{oh:02}:{om:02}")
    }
}

fn div_floor(a: i64, b: i64) -> i64 {
    let mut q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q -= 1;
    }
    q
}

fn civil_from_days(days_since_epoch: i64) -> (i32, u32, u32) {
    // Howard Hinnant's civil calendar algorithm. Input day 0 = 1970-01-01.
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year as i32, m as u32, d as u32)
}

pub fn human_duration(seconds: u64, future: bool) -> String {
    let (amount, unit) = if seconds < 60 {
        (seconds.max(1), "second")
    } else if seconds < 3_600 {
        (seconds / 60, "minute")
    } else if seconds < 86_400 {
        (seconds / 3_600, "hour")
    } else if seconds < 2_592_000 {
        (seconds / 86_400, "day")
    } else if seconds < 31_536_000 {
        (seconds / 2_592_000, "month")
    } else {
        (seconds / 31_536_000, "year")
    };
    let plural = if amount == 1 { "" } else { "s" };
    if future { format!("in {amount} {unit}{plural}") } else { format!("{amount} {unit}{plural} ago") }
}

pub fn natural_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 { format!("{} {}", bytes, UNITS[unit]) } else { format!("{value:.1} {}", UNITS[unit]) }
}

// ---------------------------------------------------------------------------
// benedict_demo.py / orjson_demo.py style nested data and JSON-ish output
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum MiniValue {
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    Array(Vec<MiniValue>),
    Object(BTreeMap<String, MiniValue>),
}

impl MiniValue {
    pub fn object() -> Self {
        Self::Object(BTreeMap::new())
    }

    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<String, MiniValue>> {
        match self {
            MiniValue::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn to_json_string(&self) -> String {
        match self {
            MiniValue::Null => "null".to_string(),
            MiniValue::Bool(v) => v.to_string(),
            MiniValue::Number(v) => {
                if v.fract() == 0.0 { format!("{v:.0}") } else { v.to_string() }
            }
            MiniValue::Text(s) => format!("\"{}\"", escape_json(s)),
            MiniValue::Array(items) => {
                let inner = items.iter().map(MiniValue::to_json_string).collect::<Vec<_>>().join(",");
                format!("[{inner}]")
            }
            MiniValue::Object(map) => {
                let inner = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_json(k), v.to_json_string()))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{{{inner}}}")
            }
        }
    }
}

pub fn escape_json(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

pub fn set_dot_path(root: &mut MiniValue, path: &str, value: MiniValue) -> Result<()> {
    if path.trim().is_empty() {
        return Err(SdmError::InvalidInput("dot path cannot be empty".to_string()));
    }
    let parts = path.split('.').collect::<Vec<_>>();
    let mut current = root;
    for part in &parts[..parts.len() - 1] {
        if part.is_empty() {
            return Err(SdmError::InvalidInput("dot path contains an empty segment".to_string()));
        }
        let map = current.as_object_mut().ok_or_else(|| {
            SdmError::InvalidInput(format!("path segment '{part}' is not an object"))
        })?;
        current = map.entry((*part).to_string()).or_insert_with(MiniValue::object);
    }
    let last = parts[parts.len() - 1];
    if last.is_empty() {
        return Err(SdmError::InvalidInput("dot path contains an empty final segment".to_string()));
    }
    let map = current.as_object_mut().ok_or_else(|| {
        SdmError::InvalidInput(format!("path segment '{last}' is not an object"))
    })?;
    map.insert(last.to_string(), value);
    Ok(())
}

pub fn get_dot_path<'a>(root: &'a MiniValue, path: &str) -> Option<&'a MiniValue> {
    let mut current = root;
    for part in path.split('.') {
        match current {
            MiniValue::Object(map) => current = map.get(part)?,
            _ => return None,
        }
    }
    Some(current)
}

pub fn demo_json_value() -> MiniValue {
    let mut map = BTreeMap::new();
    map.insert("n".to_string(), MiniValue::Number(1.0));
    map.insert("items".to_string(), MiniValue::Array(vec![MiniValue::Number(1.0), MiniValue::Number(2.0), MiniValue::Number(3.0)]));
    map.insert("ok".to_string(), MiniValue::Bool(true));
    MiniValue::Object(map)
}

// ---------------------------------------------------------------------------
// bs4_demo.py / requests_demo.py / httpx_demo.py
// ---------------------------------------------------------------------------

pub fn extract_first_tag_text(html: &str, tag: &str) -> Option<String> {
    extract_tag_texts(html, tag).into_iter().next()
}

pub fn extract_tag_texts(html: &str, tag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let lower = html.to_ascii_lowercase();
    let tag_lower = tag.to_ascii_lowercase();
    let open_prefix = format!("<{tag_lower}");
    let close = format!("</{tag_lower}>");
    let mut pos = 0usize;
    while let Some(open_rel) = lower[pos..].find(&open_prefix) {
        let open_abs = pos + open_rel;
        let Some(open_end_rel) = lower[open_abs..].find('>') else { break };
        let text_start = open_abs + open_end_rel + 1;
        let Some(close_rel) = lower[text_start..].find(&close) else { break };
        let text_end = text_start + close_rel;
        out.push(strip_tags(&html[text_start..text_end]).trim().to_string());
        pos = text_end + close.len();
    }
    out
}

pub fn strip_tags(html_fragment: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in html_fragment.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    html_unescape_basic(&out)
}

pub fn html_unescape_basic(input: &str) -> String {
    input
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

pub fn build_http_get_request(host: &str, path: &str) -> String {
    format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: secure-diskmanager-ops/0.2\r\nConnection: close\r\n\r\n")
}

pub fn plain_http_get(host: &str, port: u16, path: &str, timeout: Duration) -> Result<String> {
    use std::net::TcpStream;
    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    stream.write_all(build_http_get_request(host, path).as_bytes())?;
    let mut body = String::new();
    stream.read_to_string(&mut body)?;
    Ok(body)
}

// ---------------------------------------------------------------------------
// dataclasses_demo.py / pydantic_demo.py / pyinputplus_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct PointRecord {
    pub x: f64,
    pub y: f64,
    pub meta: BTreeMap<String, String>,
}

impl PointRecord {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, meta: BTreeMap::new() }
    }

    pub fn with_meta(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.meta.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedUser {
    pub name: String,
    pub age: u8,
}

pub fn validate_user(name: impl Into<String>, age: i32) -> Result<ValidatedUser> {
    let name = name.into();
    if name.trim().is_empty() {
        return Err(SdmError::InvalidInput("name cannot be empty".to_string()));
    }
    if !(0..=150).contains(&age) {
        return Err(SdmError::InvalidInput("age must be between 0 and 150".to_string()));
    }
    Ok(ValidatedUser { name, age: age as u8 })
}

pub fn parse_bounded_i32(input: &str, min: i32, max: i32) -> Result<i32> {
    let value = input
        .trim()
        .parse::<i32>()
        .map_err(|_| SdmError::InvalidInput("input must be an integer".to_string()))?;
    if value < min || value > max {
        return Err(SdmError::InvalidInput(format!("input must be between {min} and {max}")));
    }
    Ok(value)
}

// ---------------------------------------------------------------------------
// dynaconf_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MiniConfig {
    sections: BTreeMap<String, BTreeMap<String, String>>,
    active_section: String,
    env_prefix: Option<String>,
}

impl MiniConfig {
    pub fn parse_toml_like(input: &str) -> Result<Self> {
        let mut cfg = MiniConfig { active_section: "default".to_string(), ..MiniConfig::default() };
        let mut section = "default".to_string();
        cfg.sections.entry(section.clone()).or_default();
        for raw_line in input.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].trim().to_string();
                if section.is_empty() {
                    return Err(SdmError::InvalidFormat("empty config section".to_string()));
                }
                cfg.sections.entry(section.clone()).or_default();
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(SdmError::InvalidFormat(format!("bad config line: {line}")));
            };
            cfg.sections
                .entry(section.clone())
                .or_default()
                .insert(key.trim().to_ascii_uppercase(), value.trim().trim_matches('"').to_string());
        }
        Ok(cfg)
    }

    pub fn with_env_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.env_prefix = Some(prefix.into());
        self
    }

    pub fn set_env(mut self, section: impl Into<String>) -> Self {
        self.active_section = section.into();
        self
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let upper = key.to_ascii_uppercase();
        if let Some(prefix) = &self.env_prefix {
            let env_key = format!("{prefix}_{upper}");
            if let Ok(value) = std::env::var(env_key) {
                return Some(value);
            }
        }
        self.sections
            .get(&self.active_section)
            .and_then(|s| s.get(&upper))
            .cloned()
            .or_else(|| self.sections.get("default").and_then(|s| s.get(&upper)).cloned())
    }
}

// ---------------------------------------------------------------------------
// faker_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FakePerson {
    pub name: String,
    pub email: String,
    pub address_line: String,
}

#[derive(Debug, Clone)]
pub struct DeterministicFaker {
    state: u64,
}

impl DeterministicFaker {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_index(&mut self, len: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 32) as usize) % len.max(1)
    }

    pub fn person(&mut self) -> FakePerson {
        const FIRST: [&str; 8] = ["Ada", "Grace", "Linus", "Katherine", "Alan", "Margaret", "Donald", "Barbara"];
        const LAST: [&str; 8] = ["Lovelace", "Hopper", "Torvalds", "Johnson", "Turing", "Hamilton", "Knuth", "Liskov"];
        const STREET: [&str; 6] = ["Maple Road", "Turing Way", "Kernel Lane", "Lambda Street", "Packet Close", "Vector Avenue"];
        let first = FIRST[self.next_index(FIRST.len())];
        let last = LAST[self.next_index(LAST.len())];
        let number = 1 + self.next_index(220);
        let street = STREET[self.next_index(STREET.len())];
        FakePerson {
            name: format!("{first} {last}"),
            email: format!("{}.{}@example.test", first.to_ascii_lowercase(), last.to_ascii_lowercase()),
            address_line: format!("{number} {street}"),
        }
    }
}

// ---------------------------------------------------------------------------
// fastapi_demo.py / flask_demo.py / gradio_demo.py / typer_demo.py
// ---------------------------------------------------------------------------

pub fn ping_json() -> &'static str {
    "{\"ping\":\"pong\"}"
}

pub fn ping_text() -> &'static str {
    "pong"
}

pub fn greet(name: &str) -> String {
    format!("hello {}", if name.trim().is_empty() { "world" } else { name.trim() })
}

pub fn serve_ping_once(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    if let Some(stream) = listener.incoming().next() {
        let mut stream = stream?;
        let mut request = [0u8; 1024];
        let _ = stream.read(&mut request)?;
        let body = ping_json();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// fuzzywuzzy_demo.py
// ---------------------------------------------------------------------------

pub fn levenshtein(a: &str, b: &str) -> usize {
    let b_chars = b.chars().collect::<Vec<_>>();
    let mut costs = (0..=b_chars.len()).collect::<Vec<_>>();
    for (i, ca) in a.chars().enumerate() {
        let mut last_diag = i;
        costs[0] = i + 1;
        for (j, cb) in b_chars.iter().enumerate() {
            let old = costs[j + 1];
            let substitution = last_diag + usize::from(ca != *cb);
            let insertion = costs[j] + 1;
            let deletion = old + 1;
            costs[j + 1] = substitution.min(insertion).min(deletion);
            last_diag = old;
        }
    }
    costs[b_chars.len()]
}

pub fn fuzzy_ratio(a: &str, b: &str) -> u8 {
    let max_len = a.chars().count().max(b.chars().count());
    if max_len == 0 {
        return 100;
    }
    let dist = levenshtein(a, b).min(max_len);
    (((max_len - dist) * 100) / max_len) as u8
}

pub fn best_fuzzy_match<'a>(query: &str, choices: &'a [&str]) -> Option<(&'a str, u8)> {
    choices.iter().map(|choice| (*choice, fuzzy_ratio(query, choice))).max_by_key(|(_, score)| *score)
}

// ---------------------------------------------------------------------------
// chart demos: cutecharts, holoviews, plotext, pyecharts, pywaffle,
// datashader, lightningchart
// ---------------------------------------------------------------------------

pub fn line_chart_svg(title: &str, labels: &[&str], values: &[f64], width: u32, height: u32) -> Result<String> {
    if labels.len() != values.len() || values.is_empty() {
        return Err(SdmError::InvalidInput("labels and values must be non-empty and equal length".to_string()));
    }
    let width = width.max(80);
    let height = height.max(60);
    let pad = 30.0;
    let plot_w = (width as f64 - 2.0 * pad).max(1.0);
    let plot_h = (height as f64 - 2.0 * pad).max(1.0);
    let min_v = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max_v = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (max_v - min_v).abs().max(1.0);
    let mut points = String::new();
    for (i, value) in values.iter().enumerate() {
        let x = pad + (i as f64) * plot_w / ((values.len() - 1).max(1) as f64);
        let y = pad + plot_h - ((*value - min_v) / span) * plot_h;
        let _ = write!(points, "{x:.2},{y:.2} ");
    }
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\"><title>{}</title><rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"10\" y=\"20\">{}</text><polyline points=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"2\"/></svg>",
        escape_json(title),
        escape_json(title),
        points.trim()
    ))
}

pub fn bar_chart_html(title: &str, labels: &[&str], values: &[f64]) -> Result<String> {
    if labels.len() != values.len() || values.is_empty() {
        return Err(SdmError::InvalidInput("labels and values must be non-empty and equal length".to_string()));
    }
    let max = values.iter().copied().fold(0.0f64, f64::max).max(1.0);
    let mut rows = String::new();
    for (label, value) in labels.iter().zip(values.iter()) {
        let width_pct = ((*value / max) * 100.0).clamp(0.0, 100.0);
        let _ = write!(rows, "<div><span>{}</span><div style=\"display:inline-block;background:#333;height:1em;width:{width_pct:.1}%\"></div> {value}</div>", escape_json(label));
    }
    Ok(format!("<!doctype html><meta charset=\"utf-8\"><title>{}</title><h1>{}</h1>{rows}", escape_json(title), escape_json(title)))
}

pub fn ascii_plot(values: &[f64], height: usize) -> String {
    if values.is_empty() || height == 0 {
        return String::new();
    }
    let min_v = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max_v = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (max_v - min_v).abs().max(1.0);
    let scaled = values
        .iter()
        .map(|v| (((v - min_v) / span) * ((height - 1) as f64)).round() as usize)
        .collect::<Vec<_>>();
    let mut out = String::new();
    for row in (0..height).rev() {
        for value in &scaled {
            out.push(if *value == row { '*' } else { ' ' });
        }
        out.push('\n');
    }
    out
}

pub fn waffle_chart_text(values: &[(&str, usize)], rows: usize) -> String {
    let rows = rows.max(1);
    let total: usize = values.iter().map(|(_, v)| *v).sum();
    if total == 0 {
        return String::new();
    }
    let columns = (total + rows - 1) / rows;
    let mut cells = Vec::new();
    for (label, count) in values {
        let marker = label.chars().next().unwrap_or('?').to_ascii_uppercase();
        cells.extend(std::iter::repeat(marker).take(*count));
    }
    let mut out = String::new();
    for row in 0..rows {
        for col in 0..columns {
            let idx = col * rows + row;
            out.push(cells.get(idx).copied().unwrap_or(' '));
        }
        out.push('\n');
    }
    out
}

pub fn density_grid(points: &[(f64, f64)], width: usize, height: usize) -> Vec<Vec<u32>> {
    let width = width.max(1);
    let height = height.max(1);
    let mut grid = vec![vec![0u32; width]; height];
    if points.is_empty() {
        return grid;
    }
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    let span_x = (max_x - min_x).abs().max(1.0);
    let span_y = (max_y - min_y).abs().max(1.0);
    for &(x, y) in points {
        let col = (((x - min_x) / span_x) * ((width - 1) as f64)).round() as usize;
        let row = (((y - min_y) / span_y) * ((height - 1) as f64)).round() as usize;
        grid[height - 1 - row.min(height - 1)][col.min(width - 1)] += 1;
    }
    grid
}

// ---------------------------------------------------------------------------
// logging_demo.py / loguru_demo.py / icecream_demo.py / rich_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub fn log_line(level: LogLevel, message: &str) -> String {
    let label = match level {
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
    };
    format!("{label}:{message}")
}

pub fn append_log_line(path: impl AsRef<Path>, level: LogLevel, message: &str, rotate_after_bytes: u64) -> Result<()> {
    let path = path.as_ref();
    if rotate_after_bytes > 0 {
        if let Ok(meta) = fs::metadata(path) {
            if meta.len() >= rotate_after_bytes {
                let rotated = path.with_extension("log.1");
                let _ = fs::remove_file(&rotated);
                fs::rename(path, rotated)?;
            }
        }
    }
    let mut file = fs::OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{}", log_line(level, message))?;
    Ok(())
}

pub fn icecream_debug<T: std::fmt::Debug>(label: &str, value: &T) -> String {
    format!("ic| {label}: {value:?}")
}

pub fn rich_table(title: &str, headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();
    for row in rows {
        if row.len() > widths.len() {
            widths.resize(row.len(), 0);
        }
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    let mut out = String::new();
    let _ = writeln!(out, "{title}");
    for (i, header) in headers.iter().enumerate() {
        let _ = write!(out, "| {:width$} ", header, width = widths[i]);
    }
    out.push_str("|\n");
    for width in &widths {
        let _ = write!(out, "|-{}-", "-".repeat(*width));
    }
    out.push_str("|\n");
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let _ = write!(out, "| {:width$} ", cell, width = widths[i]);
        }
        out.push_str("|\n");
    }
    out
}

// ---------------------------------------------------------------------------
// pandas_demo.py / torch_tensor_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataRow {
    pub x: i64,
    pub y: i64,
}

impl DataRow {
    pub fn z(self) -> i64 {
        self.x * self.y
    }
}

pub fn dataframe_assign_product(rows: &[DataRow]) -> Vec<(i64, i64, i64)> {
    rows.iter().map(|row| (row.x, row.y, row.z())).collect()
}

pub fn dataframe_to_table(rows: &[DataRow]) -> String {
    let table_rows = dataframe_assign_product(rows)
        .into_iter()
        .map(|(x, y, z)| vec![x.to_string(), y.to_string(), z.to_string()])
        .collect::<Vec<_>>();
    rich_table("dataframe", &["x", "y", "z"], &table_rows)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self> {
        if rows == 0 || cols == 0 {
            return Err(SdmError::InvalidInput("matrix dimensions must be non-zero".to_string()));
        }
        if data.len() != rows * cols {
            return Err(SdmError::InvalidInput("matrix data length does not match dimensions".to_string()));
        }
        Ok(Self { rows, cols, data })
    }

    pub fn ones_like(other: &Self) -> Self {
        Self { rows: other.rows, cols: other.cols, data: vec![1.0; other.rows * other.cols] }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn matmul(&self, rhs: &Self) -> Result<Self> {
        if self.cols != rhs.rows {
            return Err(SdmError::InvalidInput("left columns must equal right rows".to_string()));
        }
        let mut out = vec![0.0; self.rows * rhs.cols];
        for r in 0..self.rows {
            for c in 0..rhs.cols {
                let mut acc = 0.0;
                for k in 0..self.cols {
                    acc += self.get(r, k) * rhs.get(k, c);
                }
                out[r * rhs.cols + c] = acc;
            }
        }
        Matrix::new(self.rows, rhs.cols, out)
    }

    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }
}

// ---------------------------------------------------------------------------
// pdfplumber_demo.py / pymupdf_demo.py / pytesseract_demo.py / pillow_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfSummary {
    pub page_count_guess: usize,
    pub first_text: String,
}

pub fn summarize_pdf_rough(path: impl AsRef<Path>, max_chars: usize) -> Result<PdfSummary> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes);
    let page_count_guess = text.matches("/Type /Page").count().saturating_sub(text.matches("/Type /Pages").count()).max(1);
    let first_text = extract_pdf_literal_strings(&text).chars().take(max_chars).collect::<String>();
    Ok(PdfSummary { page_count_guess, first_text })
}

pub fn extract_pdf_literal_strings(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '(' {
            let mut literal = String::new();
            let mut escaped = false;
            for inner in chars.by_ref() {
                if escaped {
                    literal.push(match inner {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        other => other,
                    });
                    escaped = false;
                } else if inner == '\\' {
                    escaped = true;
                } else if inner == ')' {
                    break;
                } else {
                    literal.push(inner);
                }
            }
            if literal.chars().any(|c| c.is_alphabetic()) {
                if !out.is_empty() {
                    out.push(' ');
                }
                out.push_str(&literal);
            }
        }
    }
    out
}

pub fn demo_image_svg() -> Vec<u8> {
    br#"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="80"><rect width="120" height="80" fill="white"/><rect x="10" y="10" width="100" height="60" fill="none" stroke="black"/><text x="20" y="45" font-family="monospace" font-size="16">hi</text></svg>"#.to_vec()
}

pub fn tesseract_status_message() -> &'static str {
    "OCR wrapper placeholder: install Tesseract and call an OCR engine-specific binding from the application layer."
}

// ---------------------------------------------------------------------------
// pywhat_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierKind {
    Email,
    Uuid,
    BitcoinAddress,
    Unknown,
}

pub fn identify_string(input: &str) -> IdentifierKind {
    let s = input.trim();
    if looks_like_email(s) {
        IdentifierKind::Email
    } else if looks_like_uuid(s) {
        IdentifierKind::Uuid
    } else if looks_like_bitcoin_address(s) {
        IdentifierKind::BitcoinAddress
    } else {
        IdentifierKind::Unknown
    }
}

fn looks_like_email(s: &str) -> bool {
    let Some((local, domain)) = s.split_once('@') else { return false };
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn looks_like_uuid(s: &str) -> bool {
    let parts = s.split('-').collect::<Vec<_>>();
    let lens = [8usize, 4, 4, 4, 12];
    parts.len() == lens.len() && parts.iter().zip(lens).all(|(p, l)| p.len() == l && p.chars().all(|c| c.is_ascii_hexdigit()))
}

fn looks_like_bitcoin_address(s: &str) -> bool {
    let len = s.len();
    (26..=62).contains(&len)
        && (s.starts_with('1') || s.starts_with('3') || s.starts_with("bc1"))
        && s.chars().all(|c| c.is_ascii_alphanumeric())
}

// ---------------------------------------------------------------------------
// redirect_stdout_demo.py
// ---------------------------------------------------------------------------

pub fn noisy_lines() -> Vec<&'static str> {
    vec!["alpha", "beta"]
}

pub fn capture_noisy() -> String {
    noisy_lines().join("\n")
}

// ---------------------------------------------------------------------------
// ruff_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonLintIssue {
    pub line: usize,
    pub message: String,
}

pub fn mini_python_lint(source: &str) -> Vec<PythonLintIssue> {
    let mut issues = Vec::new();
    let mut imports = Vec::new();
    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line.ends_with(' ') || line.ends_with('\t') {
            issues.push(PythonLintIssue { line: line_no, message: "trailing whitespace".to_string() });
        }
        if line.contains("print(  ") {
            issues.push(PythonLintIssue { line: line_no, message: "extra spaces after '(' in print call".to_string() });
        }
        if let Some(name) = line.strip_prefix("import ") {
            imports.push((line_no, name.trim().split_whitespace().next().unwrap_or("").to_string()));
        }
    }
    for (line_no, name) in imports {
        if !name.is_empty() && source.matches(name.as_str()).count() <= 1 {
            issues.push(PythonLintIssue { line: line_no, message: format!("imported module '{name}' appears unused") });
        }
    }
    issues
}

// ---------------------------------------------------------------------------
// schedule_demo.py / tenacity_demo.py / prefect_demo.py / watchdog_demo.py
// ---------------------------------------------------------------------------

pub fn run_fixed_schedule<F>(interval: Duration, ticks: usize, mut job: F) -> Result<usize>
where
    F: FnMut(usize) -> Result<()>,
{
    for tick in 1..=ticks {
        job(tick)?;
        if tick != ticks {
            thread::sleep(interval);
        }
    }
    Ok(ticks)
}

pub fn retry_fixed<T, E, F>(attempts: usize, wait: Duration, mut op: F) -> std::result::Result<T, E>
where
    F: FnMut(usize) -> std::result::Result<T, E>,
{
    let attempts = attempts.max(1);
    for attempt in 1..=attempts {
        match op(attempt) {
            Ok(value) => return Ok(value),
            Err(err) if attempt == attempts => return Err(err),
            Err(_) => thread::sleep(wait),
        }
    }
    unreachable!("attempts.max(1) guarantees loop runs")
}

pub fn extract_task() -> Vec<i32> {
    vec![1, 2, 3]
}

pub fn transform_task(xs: &[i32]) -> Vec<i32> {
    xs.iter().map(|x| x * 10).collect()
}

pub fn etl_flow() -> Vec<i32> {
    transform_task(&extract_task())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub kind: String,
}

pub fn watch_file_mtime(path: impl AsRef<Path>, timeout: Duration, poll_every: Duration) -> Result<Option<FileChangeEvent>> {
    let path = path.as_ref().to_path_buf();
    let before = fs::metadata(&path)?.modified()?;
    let start = Instant::now();
    while start.elapsed() < timeout {
        thread::sleep(poll_every);
        let after = fs::metadata(&path)?.modified()?;
        if after > before {
            return Ok(Some(FileChangeEvent { path, kind: "modified".to_string() }));
        }
    }
    Ok(None)
}

// ---------------------------------------------------------------------------
// sentence_transformers_demo.py
// ---------------------------------------------------------------------------

pub fn bag_of_words_vector(sentence: &str) -> BTreeMap<String, f64> {
    let mut map = BTreeMap::new();
    for word in sentence
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_ascii_lowercase())
    {
        *map.entry(word).or_insert(0.0) += 1.0;
    }
    let norm = map.values().map(|v| v * v).sum::<f64>().sqrt().max(1.0);
    for value in map.values_mut() {
        *value /= norm;
    }
    map
}

pub fn cosine_similarity_sparse(a: &BTreeMap<String, f64>, b: &BTreeMap<String, f64>) -> f64 {
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    small.iter().filter_map(|(k, av)| large.get(k).map(|bv| av * bv)).sum()
}

pub fn sentence_similarity_scores<'a>(query: &str, sentences: &'a [&str]) -> Vec<(&'a str, f64)> {
    let qv = bag_of_words_vector(query);
    sentences.iter().map(|s| (*s, cosine_similarity_sparse(&qv, &bag_of_words_vector(s)))).collect()
}

// ---------------------------------------------------------------------------
// shapely_demo.py
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

pub fn polygon_area(points: &[Point2D]) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }
    let mut area = 0.0;
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        area += a.x * b.y - b.x * a.y;
    }
    area.abs() / 2.0
}

pub fn polygon_contains_point(poly: &[Point2D], point: Point2D) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let pi = poly[i];
        let pj = poly[j];
        let intersects = ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / ((pj.y - pi.y).abs().max(f64::EPSILON)) + pi.x);
        if intersects {
            inside = !inside;
        }
        j = i;
    }
    inside
}

// ---------------------------------------------------------------------------
// openai_demo.py / playwright_demo.py / selenium_demo.py / pytesseract_demo.py
// ---------------------------------------------------------------------------

pub fn external_service_blocked(service: &'static str) -> Result<()> {
    Err(SdmError::Blocked(match service {
        "openai" => "OpenAI API calls require the application layer to provide API credentials and a network client",
        "playwright" => "browser automation is not embedded in this std-only operator crate",
        "selenium" => "browser automation is not embedded in this std-only operator crate",
        "tesseract" => "OCR engine invocation is not embedded in this std-only operator crate",
        _ => "external service is intentionally not embedded in this std-only operator crate",
    }))
}

// ---------------------------------------------------------------------------
// Small all-in-one demo helper
// ---------------------------------------------------------------------------

pub fn demo_report() -> Result<String> {
    let mut out = String::new();
    let now = SimpleDateTime::tokyo_now()?;
    let _ = writeln!(out, "tokyo now: {}", now.iso8601());
    let later = now.add_minutes(90);
    let _ = writeln!(out, "in 90 min: {}", later.humanize_since(now));

    let mut nested = MiniValue::object();
    set_dot_path(&mut nested, "a.b", MiniValue::Number(2.0))?;
    let _ = writeln!(out, "nested a.b: {:?}", get_dot_path(&nested, "a.b"));

    let html = "<html><body><h1>Hello</h1><ul><li>a</li><li>b</li></ul></body></html>";
    let _ = writeln!(out, "h1: {}", extract_first_tag_text(html, "h1").unwrap_or_default());
    let _ = writeln!(out, "li: {:?}", extract_tag_texts(html, "li"));

    let choices = ["kitten", "sitting", "knitting", "bitten"];
    let _ = writeln!(out, "best fuzzy for kittn: {:?}", best_fuzzy_match("kittn", &choices));

    let mut faker = DeterministicFaker::new(42);
    let fake = faker.person();
    let _ = writeln!(out, "fake: {} | {} | {}", fake.name, fake.email, fake.address_line);

    let tri = [Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 0.0 }, Point2D { x: 1.0, y: 2.0 }];
    let _ = writeln!(out, "triangle area: {}", polygon_area(&tri));
    let _ = writeln!(out, "contains point: {}", polygon_contains_point(&tri, Point2D { x: 1.0, y: 0.5 }));

    let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;
    let c = a.matmul(&Matrix::ones_like(&a))?;
    let _ = writeln!(out, "matrix c: {:?}, sum={}", c.data, c.sum());

    Ok(out)
}

#[allow(dead_code)]
fn _keep_hashmap_import_live(_: Option<HashMap<String, String>>, _: Option<BTreeSet<String>>) {}

# Python Demo Expansion Map — v0.2.0

This file maps `python_library_demos_sm0l.tar.gz` into safe, std-only Rust operator equivalents added under `src/python_ops.rs`.

The Python files are tiny library demos, not production packages, so the Rust expansion preserves the useful behaviours rather than embedding heavyweight Python-equivalent runtimes. Browser automation, paid/licensed chart renderers, OCR engines, and OpenAI API calls are represented as explicit application-layer responsibilities or blocked placeholders instead of hidden network/browser/API execution.

| Python demo | Original behaviour | Rust equivalent |
|---|---|---|
| `arrow_demo.py` | timezone now + 90-minute human diff | `SimpleDateTime::tokyo_now`, `add_minutes`, `iso8601`, `humanize_since` |
| `pendulum_demo.py` | timezone now + date arithmetic + human diff | `SimpleDateTime`, `add_days`, `add_hours`, `humanize_since` |
| `humanize_demo.py` | natural size/time text | `natural_size`, `human_duration` |
| `benedict_demo.py` | dot-path nested dictionary | `MiniValue`, `set_dot_path`, `get_dot_path` |
| `orjson_demo.py` | JSON bytes + roundtrip idea | `MiniValue::to_json_string`, `demo_json_value` |
| `bs4_demo.py` | extract `h1` and `li` text | `extract_first_tag_text`, `extract_tag_texts`, `strip_tags` |
| `dataclasses_demo.py` | `Point` data class | `PointRecord` |
| `pydantic_demo.py` | validate user name/age | `ValidatedUser`, `validate_user` |
| `pyinputplus_demo.py` | bounded integer input | `parse_bounded_i32` |
| `dynaconf_demo.py` | minimal TOML sections + env override | `MiniConfig::parse_toml_like`, `with_env_prefix`, `set_env`, `get` |
| `faker_demo.py` | fake name/email/address | `DeterministicFaker`, `FakePerson` |
| `fastapi_demo.py` | `/ping` JSON app | `ping_json`, `serve_ping_once` |
| `flask_demo.py` | `/ping` text app | `ping_text`, `serve_ping_once` |
| `gradio_demo.py` | greeting callback | `greet` |
| `typer_demo.py` | CLI greeting | `greet`, CLI command `py-greet` |
| `fuzzywuzzy_demo.py` | ratio + best match | `levenshtein`, `fuzzy_ratio`, `best_fuzzy_match` |
| `httpx_demo.py` | HTTP GET demo | `build_http_get_request`, `plain_http_get` for plain HTTP only |
| `requests_demo.py` | HTTP GET demo | `build_http_get_request`, `plain_http_get` for plain HTTP only |
| `cutecharts_demo.py` | line chart HTML/notebook | `line_chart_svg` |
| `pyecharts_demo.py` | bar chart HTML | `bar_chart_html` |
| `plotext_demo.py` | terminal plot | `ascii_plot` |
| `pywaffle_demo.py` | waffle chart | `waffle_chart_text` |
| `datashader_demo.py` | density grid/image | `density_grid` |
| `holoviews_demo.py` | curve object | `line_chart_svg` / chart data operators |
| `lightningchart_demo.py` | licensed chart import | represented by chart data/SVG helpers; no license-bound renderer embedded |
| `logging_demo.py` | basic logger | `log_line` |
| `loguru_demo.py` | rotating file logger | `append_log_line` |
| `icecream_demo.py` | labelled debug dump | `icecream_debug` |
| `rich_demo.py` | terminal table | `rich_table` |
| `pandas_demo.py` | table + derived column | `DataRow`, `dataframe_assign_product`, `dataframe_to_table` |
| `torch_tensor_demo.py` | tensor matmul/sum | `Matrix::new`, `ones_like`, `matmul`, `sum` |
| `pdfplumber_demo.py` | page count + first page text | `summarize_pdf_rough`, `extract_pdf_literal_strings` |
| `pymupdf_demo.py` | page count + first page text | `summarize_pdf_rough`, `extract_pdf_literal_strings` |
| `pillow_demo.py` | create small image | `demo_image_svg` |
| `pytesseract_demo.py` | OCR package/version note | `tesseract_status_message`, `external_service_blocked("tesseract")` |
| `pywhat_demo.py` | identify sample strings | `identify_string`, `IdentifierKind` |
| `redirect_stdout_demo.py` | capture printed output | `noisy_lines`, `capture_noisy` |
| `ruff_demo.py` | lint bad Python file | `mini_python_lint`, `PythonLintIssue` |
| `schedule_demo.py` | run ticking job | `run_fixed_schedule` |
| `tenacity_demo.py` | retry flaky task | `retry_fixed` |
| `prefect_demo.py` | extract-transform-flow | `extract_task`, `transform_task`, `etl_flow` |
| `watchdog_demo.py` | watch modified file | `watch_file_mtime`, `FileChangeEvent` |
| `sentence_transformers_demo.py` | sentence embeddings + cosine similarity | `bag_of_words_vector`, `cosine_similarity_sparse`, `sentence_similarity_scores` |
| `shapely_demo.py` | polygon area + contains point | `Point2D`, `polygon_area`, `polygon_contains_point` |
| `openai_demo.py` | OpenAI Responses API call | `external_service_blocked("openai")`; caller must supply API client/credentials explicitly |
| `playwright_demo.py` | headless browser title | `external_service_blocked("playwright")`; browser automation stays app-layer |
| `selenium_demo.py` | headless browser title | `external_service_blocked("selenium")`; browser automation stays app-layer |

## New CLI Commands

```text
py-demo-list
py-demo-report
py-html <tag> <html>
py-fuzzy <query> <choice> [choice...]
py-identify <string>
py-human-size <bytes>
py-validate-user <name> <age>
py-tensor-demo
py-chart-svg <out.svg>
py-pdf-summary <file.pdf>
py-greet [name]
```

## Build Tests To Run On Windows

```powershell
cargo clean
cargo build --bin secure_diskmanager_ops
cargo build --example demo
cargo build --example python_ops_demo
cargo run --bin secure_diskmanager_ops -- py-demo-report
cargo run --example python_ops_demo
```

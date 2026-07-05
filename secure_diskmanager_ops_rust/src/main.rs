use secure_diskmanager_ops::{
    compression, disk_management, dpapi, encryption, entropy, python_ops, rng, search, secure_deletion,
};
use std::env;
use std::path::PathBuf;
use std::process;

fn usage() -> &'static str {
    r#"secure_diskmanager_ops command-line wrapper

USAGE:
  secure_diskmanager_ops <command> [args]

SAFE LOCAL COMMANDS:
  help
      Show this help text.

  disk-usage <path>
      Sum regular-file byte usage under <path>.

  search <root> <regex> [--entropy]
      Regex-search file paths under <root>. Optional entropy score per hit.

  ultra-search <root> <substring> [--csv <out.csv>]
      Fast substring file-path search. Optionally export results to CSV.

  entropy <file>
      Calculate Shannon entropy for a file.

  rng-keyfile <out-file> <num-bytes>
      Write OS-random bytes to a keyfile.

  compress <input> <output>
      Legacy zlib compressor: [u32 original_len][zlib payload].

  decompress <input> <output>
      Decompress legacy zlib format.

  compress-mt <input> <output> [chunk-size]
      Chunked compressor. Default chunk-size: 1048576.

  decompress-mt <input> <output>
      Decompress chunked compressor format.

  compress-mt-enc <input> <output> <keyfile-or-> [fast|ratio]
      Chunked legacy XOR-obfuscated compressor. Use '-' for no keyfile.

  decompress-mt-enc <input> <output> <keyfile-or->
      Decompress chunked legacy XOR-obfuscated compressor output.

  rc4-encrypt <input> <key-string>
      Legacy RC4 compatibility transform. Writes <input>.enc.

  rc4-decrypt <input> <key-string>
      Legacy RC4 compatibility transform. Writes <input>.dec.

  rc4-encrypt-keyfile <input> <keyfile>
      Legacy RC4 compatibility transform using keyfile. Writes <input>.enc.

  rc4-decrypt-keyfile <input> <keyfile>
      Legacy RC4 compatibility transform using keyfile. Writes <input>.dec.

  dpapi-protect <plain-text> <out.xml>
      Windows only. Protect text with current-user DPAPI and write XML.

  dpapi-unprotect <in.xml>
      Windows only. Unprotect XML and print plaintext.

  shred <file> [passes]
      Overwrite and remove a regular file. Default passes: 3.


PYTHON-DEMO EQUIVALENT COMMANDS:
  py-demo-list
      List the attached Python demo files represented in Rust.

  py-demo-report
      Run a small std-only report exercising the Python-demo equivalent operators.

  py-html <tag> <html>
      Extract text for a simple HTML tag, BeautifulSoup-demo style.

  py-fuzzy <query> <choice> [choice...]
      Fuzzy-match a query against choices using Levenshtein ratio.

  py-identify <string>
      Identify simple email / UUID / Bitcoin-address-like strings.

  py-human-size <bytes>
      Render bytes as a human-readable size.

  py-validate-user <name> <age>
      Pydantic-style validation: name present, age 0..150.

  py-tensor-demo
      Run the tiny matrix multiplication equivalent of torch_tensor_demo.py.

  py-chart-svg <out.svg>
      Write a small SVG line chart equivalent to the chart demos.

  py-pdf-summary <file.pdf>
      Rough std-only PDF page/text summary. Not a full PDF engine.

  py-greet [name]
      Gradio/Typer-style greeting.

BLOCKED BY DESIGN:
  selfdelete, kill-switch, stealth-mailer, tor-dispatch, mac-spoof, stealth-tunnel
      These were present/stubbed in the C++ archive but are not operationally ported.

EXAMPLES:
  secure_diskmanager_ops disk-usage .
  secure_diskmanager_ops search . "\\.rs$" --entropy
  secure_diskmanager_ops compress input.bin input.bin.z
  secure_diskmanager_ops decompress input.bin.z input.roundtrip.bin
  secure_diskmanager_ops rng-keyfile my.key 32
"#
}

fn require_arg<'a>(args: &'a [String], index: usize, name: &str) -> secure_diskmanager_ops::Result<&'a str> {
    args.get(index)
        .map(String::as_str)
        .ok_or_else(|| secure_diskmanager_ops::SdmError::InvalidInput(format!("missing {name}")))
}

fn parse_usize(value: &str, name: &str) -> secure_diskmanager_ops::Result<usize> {
    value
        .parse::<usize>()
        .map_err(|_| secure_diskmanager_ops::SdmError::InvalidInput(format!("{name} must be a positive integer")))
}

fn keyfile_arg(value: &str) -> Option<PathBuf> {
    if value == "-" {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

fn run() -> secure_diskmanager_ops::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("help");
    let rest = &args[1..];

    match command {
        "help" | "--help" | "-h" => {
            print!("{}", usage());
            Ok(())
        }

        "disk-usage" => {
            let path = require_arg(rest, 0, "path")?;
            let bytes = disk_management::get_disk_usage(path)?;
            println!("{bytes}");
            Ok(())
        }

        "search" => {
            let root = require_arg(rest, 0, "root")?;
            let pattern = require_arg(rest, 1, "regex")?;
            let use_entropy = rest.iter().any(|arg| arg == "--entropy");
            let hits = search::find_matching_files(root, pattern, use_entropy)?;
            print!("{}", search::format_search_results(&hits));
            eprintln!("{} hit(s)", hits.len());
            Ok(())
        }

        "ultra-search" => {
            let root = require_arg(rest, 0, "root")?;
            let pattern = require_arg(rest, 1, "substring")?;
            let hits = search::ultra_fast_search(pattern, root)?;
            print!("{}", search::format_file_entries(&hits));
            if let Some(csv_pos) = rest.iter().position(|arg| arg == "--csv") {
                let out = rest.get(csv_pos + 1).ok_or_else(|| {
                    secure_diskmanager_ops::SdmError::InvalidInput("--csv requires an output path".to_string())
                })?;
                search::export_to_csv(&hits, out)?;
                eprintln!("wrote CSV: {out}");
            }
            eprintln!("{} hit(s)", hits.len());
            Ok(())
        }

        "entropy" => {
            let file = require_arg(rest, 0, "file")?;
            let score = entropy::calculate_file_entropy(file)?;
            println!("{score:.8}");
            Ok(())
        }

        "rng-keyfile" => {
            let out = require_arg(rest, 0, "out-file")?;
            let bytes = parse_usize(require_arg(rest, 1, "num-bytes")?, "num-bytes")?;
            rng::write_keyfile(out, bytes)?;
            println!("wrote {bytes} random byte(s) to {out}");
            Ok(())
        }

        "compress" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            compression::compress(input, output)?;
            println!("compressed {input} -> {output}");
            Ok(())
        }

        "decompress" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            compression::decompress(input, output)?;
            println!("decompressed {input} -> {output}");
            Ok(())
        }

        "compress-mt" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            let chunk_size = match rest.get(2) {
                Some(value) => parse_usize(value, "chunk-size")?,
                None => 1_048_576,
            };
            compression::compress_mt(input, output, chunk_size)?;
            println!("compressed {input} -> {output} using chunk size {chunk_size}");
            Ok(())
        }

        "decompress-mt" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            compression::decompress_mt(input, output)?;
            println!("decompressed {input} -> {output}");
            Ok(())
        }

        "compress-mt-enc" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            let keyfile = keyfile_arg(require_arg(rest, 2, "keyfile-or-")?);
            let mode = rest.get(3).map(String::as_str).unwrap_or("fast");
            if mode != "fast" && mode != "ratio" {
                return Err(secure_diskmanager_ops::SdmError::InvalidInput(
                    "mode must be 'fast' or 'ratio'".to_string(),
                ));
            }
            compression::compress_mt_enc(input, output, keyfile.as_deref(), mode)?;
            println!("compressed {input} -> {output} using legacy XOR-obfuscated chunk format");
            Ok(())
        }

        "decompress-mt-enc" => {
            let input = require_arg(rest, 0, "input")?;
            let output = require_arg(rest, 1, "output")?;
            let keyfile = keyfile_arg(require_arg(rest, 2, "keyfile-or-")?);
            compression::decompress_mt_enc(input, output, keyfile.as_deref())?;
            println!("decompressed {input} -> {output}");
            Ok(())
        }

        "rc4-encrypt" => {
            let input = require_arg(rest, 0, "input")?;
            let key = require_arg(rest, 1, "key-string")?;
            let out = encryption::encrypt_file(input, key)?;
            println!("wrote {}", out.display());
            Ok(())
        }

        "rc4-decrypt" => {
            let input = require_arg(rest, 0, "input")?;
            let key = require_arg(rest, 1, "key-string")?;
            let out = encryption::decrypt_file(input, key)?;
            println!("wrote {}", out.display());
            Ok(())
        }

        "rc4-encrypt-keyfile" => {
            let input = require_arg(rest, 0, "input")?;
            let keyfile = require_arg(rest, 1, "keyfile")?;
            let out = encryption::encrypt_file_with_keyfile(input, keyfile)?;
            println!("wrote {}", out.display());
            Ok(())
        }

        "rc4-decrypt-keyfile" => {
            let input = require_arg(rest, 0, "input")?;
            let keyfile = require_arg(rest, 1, "keyfile")?;
            let out = encryption::decrypt_file_with_keyfile(input, keyfile)?;
            println!("wrote {}", out.display());
            Ok(())
        }

        "dpapi-protect" => {
            let plain_text = require_arg(rest, 0, "plain-text")?;
            let out = require_arg(rest, 1, "out.xml")?;
            dpapi::encrypt_to_xml(plain_text, out)?;
            println!("wrote DPAPI XML: {out}");
            Ok(())
        }

        "dpapi-unprotect" => {
            let input = require_arg(rest, 0, "in.xml")?;
            let plain = dpapi::decrypt_from_xml(input)?;
            println!("{plain}");
            Ok(())
        }


        "py-demo-list" => {
            for name in python_ops::represented_python_demos() {
                println!("{name}");
            }
            Ok(())
        }

        "py-demo-report" => {
            print!("{}", python_ops::demo_report()?);
            Ok(())
        }

        "py-html" => {
            let tag = require_arg(rest, 0, "tag")?;
            let html = require_arg(rest, 1, "html")?;
            let hits = python_ops::extract_tag_texts(html, tag);
            for hit in hits {
                println!("{hit}");
            }
            Ok(())
        }

        "py-fuzzy" => {
            let query = require_arg(rest, 0, "query")?;
            if rest.len() < 2 {
                return Err(secure_diskmanager_ops::SdmError::InvalidInput(
                    "py-fuzzy requires at least one choice".to_string(),
                ));
            }
            let choices = rest[1..].iter().map(String::as_str).collect::<Vec<_>>();
            if let Some((choice, score)) = python_ops::best_fuzzy_match(query, &choices) {
                println!("{choice}\t{score}");
            }
            Ok(())
        }

        "py-identify" => {
            let value = require_arg(rest, 0, "string")?;
            println!("{:?}", python_ops::identify_string(value));
            Ok(())
        }

        "py-human-size" => {
            let value = require_arg(rest, 0, "bytes")?
                .parse::<u64>()
                .map_err(|_| secure_diskmanager_ops::SdmError::InvalidInput("bytes must be a non-negative integer".to_string()))?;
            println!("{}", python_ops::natural_size(value));
            Ok(())
        }

        "py-validate-user" => {
            let name = require_arg(rest, 0, "name")?;
            let age = require_arg(rest, 1, "age")?
                .parse::<i32>()
                .map_err(|_| secure_diskmanager_ops::SdmError::InvalidInput("age must be an integer".to_string()))?;
            let user = python_ops::validate_user(name, age)?;
            println!("name={} age={}", user.name, user.age);
            Ok(())
        }

        "py-tensor-demo" => {
            let a = python_ops::Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0])?;
            let b = python_ops::Matrix::ones_like(&a);
            let c = a.matmul(&b)?;
            println!("a={:?}", a.data);
            println!("c={:?}", c.data);
            println!("sum={}", c.sum());
            Ok(())
        }

        "py-chart-svg" => {
            let out = require_arg(rest, 0, "out.svg")?;
            let svg = python_ops::line_chart_svg("Rust chart demo", &["A", "B", "C"], &[1.0, 4.0, 9.0], 360, 220)?;
            std::fs::write(out, svg)?;
            println!("wrote {out}");
            Ok(())
        }

        "py-pdf-summary" => {
            let file = require_arg(rest, 0, "file.pdf")?;
            let summary = python_ops::summarize_pdf_rough(file, 200)?;
            println!("Pages: {}", summary.page_count_guess);
            println!("First text: {}", summary.first_text);
            Ok(())
        }

        "py-greet" => {
            let name = rest.first().map(String::as_str).unwrap_or("world");
            println!("{}", python_ops::greet(name));
            Ok(())
        }

        "shred" => {
            let file = require_arg(rest, 0, "file")?;
            let passes = match rest.get(1) {
                Some(value) => parse_usize(value, "passes")?,
                None => 3,
            };
            secure_deletion::shred_file(file, passes)?;
            println!("shredded {file} with {passes} pass(es)");
            Ok(())
        }

        "selfdelete" | "kill-switch" | "stealth-mailer" | "tor-dispatch" | "mac-spoof" | "stealth-tunnel" => {
            Err(secure_diskmanager_ops::SdmError::Blocked(
                "this operationally stealthy or destructive command is intentionally not ported",
            ))
        }

        other => Err(secure_diskmanager_ops::SdmError::InvalidInput(format!(
            "unknown command '{other}'. Run: secure_diskmanager_ops help"
        ))),
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

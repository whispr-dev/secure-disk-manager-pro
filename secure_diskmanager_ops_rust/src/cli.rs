use crate::error::{Result, SdmError};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CommandContext {
    pub args: Vec<String>,
    pub search_query: Option<String>,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub key_file: Option<String>,
    pub server_url: Option<String>,
    pub mode: Option<String>,
    pub retries: usize,
    pub encrypt: bool,
    pub decrypt: bool,
    pub compress: bool,
    pub decompress: bool,
    pub upload: bool,
    pub download: bool,
    pub search: bool,
    pub shred: bool,
    pub selfdelete: bool,
}

/// C++ equivalent: `MainController::ParseArgs`.
pub fn parse_args<I, S>(args: I) -> Result<CommandContext>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut ctx = CommandContext { retries: 3, ..CommandContext::default() };
    let mut iter = args.into_iter().map(Into::into).peekable();
    let _program = iter.next();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--search" => {
                ctx.search = true;
                ctx.search_query = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--search requires a query".to_string()))?);
            }
            "--copy" => {
                ctx.input_file = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--copy requires a file".to_string()))?);
            }
            "--encrypt" => {
                ctx.encrypt = true;
                ctx.key_file = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--encrypt requires a key file".to_string()))?);
            }
            "--decrypt" => {
                ctx.decrypt = true;
                ctx.key_file = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--decrypt requires a key file".to_string()))?);
            }
            "--compress" => ctx.compress = true,
            "--decompress" => ctx.decompress = true,
            "--upload" => {
                ctx.upload = true;
                ctx.server_url = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--upload requires a URL".to_string()))?);
            }
            "--download" => {
                ctx.download = true;
                ctx.server_url = Some(iter.next().ok_or_else(|| SdmError::InvalidInput("--download requires a URL".to_string()))?);
            }
            "--retries" => {
                let value = iter.next().ok_or_else(|| SdmError::InvalidInput("--retries requires a number".to_string()))?;
                ctx.retries = value.parse().map_err(|_| SdmError::InvalidInput("--retries must be a number".to_string()))?;
            }
            "--shred" => ctx.shred = true,
            "--selfdelete" => ctx.selfdelete = true,
            _ => ctx.args.push(arg),
        }
    }
    Ok(ctx)
}

/// C++ equivalent: `CommandDispatcher::Dispatch`, represented as an execution plan.
pub fn dispatch_plan(ctx: &CommandContext) -> Vec<String> {
    let mut plan = Vec::new();
    if ctx.search { plan.push(format!("search: {}", ctx.search_query.as_deref().unwrap_or(""))); }
    if ctx.encrypt { plan.push(format!("encrypt using key: {}", ctx.key_file.as_deref().unwrap_or(""))); }
    if ctx.decrypt { plan.push(format!("decrypt using key: {}", ctx.key_file.as_deref().unwrap_or(""))); }
    if ctx.compress { plan.push("compress file".to_string()); }
    if ctx.decompress { plan.push("decompress archive".to_string()); }
    if ctx.upload { plan.push(format!("upload to: {}", ctx.server_url.as_deref().unwrap_or(""))); }
    if ctx.download { plan.push(format!("download from: {}", ctx.server_url.as_deref().unwrap_or(""))); }
    if ctx.shred { plan.push("shred file".to_string()); }
    if ctx.selfdelete { plan.push("self-delete requested (blocked)".to_string()); }
    plan
}

/// C++ equivalent: `Ghost_UI_CLI::showBanner`.
pub fn ghost_banner() -> &'static str {
    r#"
   ________               __    ______          __
  / ____/ /_  ___  ____  / /__ / ____/__  _____/ /__  _____
 / /   / __ \/ _ \/ __ \/ / _ \\__ \/ _ \/ ___/ / _ \/ ___/
/ /___/ / / /  __/ /_/ / /  __/__/ /  __/ /__/ /  __/ /
\____/_/ /_/\___/ .___/_/\___/____/\___/\___/_/\___/_/
               /_/
"#
}

/// C++ equivalent: `Ghost_UI_CLI::listCommands`.
pub fn ghost_command_list() -> &'static [&'static str] {
    &["help", "commands", "rotate_identity", "shred", "encrypt", "decrypt", "compress", "upload", "selfdelete"]
}

/// C++ equivalent: `printHelp` / `PrintHelp`.
pub fn help_text() -> &'static str {
    "SecureDiskManager Rust Ops\n\
     --search <query>\n\
     --copy <file>\n\
     --encrypt <keyfile>\n\
     --decrypt <keyfile>\n\
     --compress\n\
     --decompress\n\
     --upload <url>\n\
     --download <url>\n\
     --retries <n>\n\
     --shred\n\
     --selfdelete (blocked)\n"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GhostCommandAction {
    Help,
    ListCommands,
    Exit,
    Execute(String),
}

/// C++ equivalent: `Ghost_UI_CLI::handleUserCommand`.
pub fn handle_ghost_user_command(input: &str) -> GhostCommandAction {
    match input.trim() {
        "help" => GhostCommandAction::Help,
        "commands" => GhostCommandAction::ListCommands,
        "exit" => GhostCommandAction::Exit,
        other => GhostCommandAction::Execute(other.to_string()),
    }
}

/// C++ equivalent: `Ghost_UI_CLI::executeCommand`, represented as a stub string.
pub fn execute_ghost_command_stub(command: &str) -> String {
    format!("[stub] would execute: {command}")
}

/// C++ equivalent shape for `handle_stealthmailer_cli`; returns blocked plans only.
pub fn handle_stealthmailer_cli(args: &[String]) -> Result<Vec<String>> {
    if args.is_empty() {
        return Ok(vec!["no stealth-mailer command supplied".to_string()]);
    }
    Err(SdmError::Blocked("stealth-mailer CLI routines were not ported"))
}

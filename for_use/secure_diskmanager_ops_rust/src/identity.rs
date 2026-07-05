use crate::error::{Result, SdmError};
use crate::gpg_wrapper;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Persona {
    pub name: String,
    pub mac_address: String,
    pub gpg_key_id: String,
    pub gpg_key_file: String,
}

#[derive(Debug, Clone)]
pub struct IdentitySwitcher {
    base_path: PathBuf,
}

impl IdentitySwitcher {
    /// C++ equivalent: `IdentitySwitcher::IdentitySwitcher`.
    pub fn new(identity_base_path: impl AsRef<Path>) -> Self {
        Self { base_path: identity_base_path.as_ref().to_path_buf() }
    }

    /// C++ equivalent: `IdentitySwitcher::switchToIdentity`.
    pub fn switch_to_identity(&self, identity_name: &str) -> Result<()> {
        let target = self.base_path.join(identity_name);
        if !target.is_dir() {
            return Err(SdmError::InvalidInput(format!("identity not found: {}", target.display())));
        }
        let active = self.base_path.join("active");
        if fs::symlink_metadata(&active).is_ok() {
            if active.is_dir() && !fs::symlink_metadata(&active)?.file_type().is_symlink() {
                return Err(SdmError::InvalidInput(format!("active path exists and is not a symlink: {}", active.display())));
            }
            let _ = fs::remove_file(&active).or_else(|_| fs::remove_dir(&active));
        }
        create_dir_symlink(&target, &active)?;
        Ok(())
    }

    /// C++ equivalent: `IdentitySwitcher::listIdentities`.
    pub fn list_identities(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() && entry.file_name().to_string_lossy() != "active" {
                out.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
        out.sort();
        Ok(out)
    }
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, active: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, active)
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, active: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, active)
}

#[derive(Debug, Clone)]
pub struct PersonaSwitcher {
    persona_root: PathBuf,
    active_persona: Option<String>,
}

impl Default for PersonaSwitcher {
    fn default() -> Self {
        Self { persona_root: PathBuf::from("personas"), active_persona: None }
    }
}

impl PersonaSwitcher {
    /// C++ equivalent: `Persona_Switcher::Persona_Switcher`.
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `Persona_Switcher::setPersonaRoot`.
    pub fn set_persona_root(&mut self, root_path: impl AsRef<Path>) {
        self.persona_root = root_path.as_ref().to_path_buf();
    }

    /// C++ equivalent: `Persona_Switcher::listPersonas`.
    pub fn list_personas(&self) -> Result<Vec<String>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.persona_root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                out.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
        out.sort();
        Ok(out)
    }

    /// C++ equivalent: `Persona_Switcher::activatePersona`.
    /// Records activation only; no MAC/browser mutation.
    pub fn activate_persona(&mut self, name: &str) -> Result<()> {
        let path = self.persona_root.join(name);
        if !path.exists() {
            return Err(SdmError::InvalidInput(format!("persona not found: {}", path.display())));
        }
        self.active_persona = Some(name.to_string());
        Ok(())
    }

    pub fn active_persona(&self) -> Option<&str> {
        self.active_persona.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct GhostIdentityManager {
    active_identity: Option<String>,
    identities_root: PathBuf,
    identity_paths: Vec<(String, PathBuf)>,
}

impl Default for GhostIdentityManager {
    fn default() -> Self {
        Self { active_identity: None, identities_root: PathBuf::from("identities"), identity_paths: Vec::new() }
    }
}

impl GhostIdentityManager {
    /// C++ equivalent: `Ghost_Identity_Manager::Ghost_Identity_Manager`.
    pub fn new() -> Self { Self::default() }

    /// C++ equivalent: `Ghost_Identity_Manager::loadIdentities`.
    pub fn load_identities(&mut self, directory: impl AsRef<Path>) -> Result<()> {
        self.identities_root = directory.as_ref().to_path_buf();
        self.identity_paths.clear();
        for entry in fs::read_dir(&self.identities_root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                self.identity_paths.push((entry.file_name().to_string_lossy().into_owned(), entry.path()));
            }
        }
        self.identity_paths.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(())
    }

    /// C++ equivalent: `Ghost_Identity_Manager::listIdentities`.
    pub fn list_identities(&self) -> Vec<String> {
        self.identity_paths.iter().map(|(name, _)| name.clone()).collect()
    }

    /// C++ equivalent: `Ghost_Identity_Manager::switchToIdentity`.
    ///
    /// The C++ version spoofed MAC, imported GPG and copied browser profiles.
    /// This safe Rust version only validates files and records active identity.
    pub fn switch_to_identity(&mut self, identity_name: &str) -> Result<IdentityActivationPlan> {
        let (_, base_path) = self.identity_paths.iter()
            .find(|(name, _)| name == identity_name)
            .ok_or_else(|| SdmError::InvalidInput(format!("identity not found: {identity_name}")))?;

        let plan = IdentityActivationPlan {
            identity_name: identity_name.to_string(),
            mac_file: base_path.join("mac.txt"),
            gpg_key: base_path.join("gpg-key.asc"),
            firefox_profile: base_path.join("firefox-profile"),
        };
        self.active_identity = Some(identity_name.to_string());
        Ok(plan)
    }

    /// C++ equivalent: `Ghost_Identity_Manager::getCurrentIdentity`.
    pub fn get_current_identity(&self) -> Option<&str> {
        self.active_identity.as_deref()
    }
}

#[derive(Debug, Clone)]
pub struct IdentityActivationPlan {
    pub identity_name: String,
    pub mac_file: PathBuf,
    pub gpg_key: PathBuf,
    pub firefox_profile: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProfileRotator {
    interface: String,
    personas: Vec<Persona>,
    current_index: usize,
}

impl ProfileRotator {
    /// C++ equivalent: `ProfileRotator::ProfileRotator`.
    pub fn new(interface: impl Into<String>) -> Self {
        Self { interface: interface.into(), personas: Vec::new(), current_index: 0 }
    }

    /// C++ equivalent: `ProfileRotator::loadPersonas`.
    pub fn load_personas(&mut self, config_file: impl AsRef<Path>) -> Result<()> {
        self.personas.clear();
        let content = fs::read_to_string(config_file)?;
        for (line_idx, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() != 4 {
                return Err(SdmError::InvalidFormat(format!("malformed persona line {}: {}", line_idx + 1, line)));
            }
            self.personas.push(Persona {
                name: parts[0].to_string(),
                mac_address: parts[1].to_string(),
                gpg_key_id: parts[2].to_string(),
                gpg_key_file: parts[3].to_string(),
            });
        }
        if self.personas.is_empty() {
            return Err(SdmError::InvalidFormat("no personas loaded".to_string()));
        }
        self.current_index = 0;
        Ok(())
    }

    /// C++ equivalent: `ProfileRotator::rotateToNext`.
    pub fn rotate_to_next(&mut self) -> Result<&Persona> {
        if self.personas.is_empty() {
            return Err(SdmError::InvalidInput("no personas loaded".to_string()));
        }
        self.current_index = (self.current_index + 1) % self.personas.len();
        let persona_clone = self.personas[self.current_index].clone();
        self.apply_persona(&persona_clone)?;
        Ok(&self.personas[self.current_index])
    }

    /// C++ equivalent: `ProfileRotator::getCurrentPersona`.
    pub fn get_current_persona(&self) -> Result<&Persona> {
        self.personas.get(self.current_index).ok_or_else(|| SdmError::InvalidInput("no personas loaded".to_string()))
    }

    /// C++ equivalent: `ProfileRotator::applyPersona`.
    /// Imports/checks GPG key only; MAC mutation is blocked elsewhere.
    pub fn apply_persona(&self, persona: &Persona) -> Result<()> {
        let _ = &self.interface;
        gpg_wrapper::import_key(&persona.gpg_key_file)?;
        if !gpg_wrapper::key_exists(&persona.gpg_key_id)? {
            return Err(SdmError::InvalidInput(format!("GPG key not found after import: {}", persona.gpg_key_id)));
        }
        Ok(())
    }
}

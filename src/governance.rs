use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GovernanceState {
    pub state_id: u32,
    pub execution_permitted: bool,
    pub immutable_lock: bool,
    pub jurisdiction: String,
    pub audit_sequence: u64,
}

impl Default for GovernanceState {
    fn default() -> Self {
        Self {
            state_id: 1,
            execution_permitted: true,
            immutable_lock: true,
            jurisdiction: "IEC62304".to_string(),
            audit_sequence: 1000,
        }
    }
}

pub fn load_governance_state<P: AsRef<Path>>(
    filepath: P,
) -> Result<GovernanceState, std::io::Error> {
    let path = filepath.as_ref();
    if !path.exists() {
        let default_state = GovernanceState::default();
        save_governance_state(&default_state, path)?;
        return Ok(default_state);
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let state: GovernanceState = serde_json::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(state)
}

pub fn save_governance_state<P: AsRef<Path>>(
    state: &GovernanceState,
    filepath: P,
) -> Result<(), std::io::Error> {
    let path = filepath.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    let contents = serde_json::to_string_pretty(state)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

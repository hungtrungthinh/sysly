use std::collections::HashMap;
use std::process::Command;

/// Process information containing priority and nice values
#[derive(Debug, Clone)]
pub struct ProcessPriority {
    pub priority: String,
    pub nice: String,
}

/// Process memory information containing virtual and resident memory
#[derive(Debug, Clone)]
pub struct ProcessMemory {
    pub virtual_memory: u64,
    pub resident_memory: u64,
}

/// Fetch priority and nice values for all processes on macOS
///
/// Uses the `ps` command to get accurate PRI/NI values that sysinfo doesn't provide
///
/// # Returns
/// HashMap mapping PID to (priority, nice) values
#[cfg(target_os = "macos")]
pub fn fetch_priority_map() -> HashMap<u32, ProcessPriority> {
    let mut map = HashMap::new();

    let output = Command::new("ps").args(&["-axo", "pid,pri,ni"]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines().skip(1) {
            // Skip header line
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 3 {
                if let Ok(pid) = parts[0].parse::<u32>() {
                    let priority = ProcessPriority {
                        priority: parts[1].to_string(),
                        nice: parts[2].to_string(),
                    };
                    map.insert(pid, priority);
                }
            }
        }
    }

    map
}

/// Fetch memory information for all processes on macOS
///
/// Uses the `ps` command to get accurate VIRT/RES values that sysinfo doesn't provide
///
/// # Returns
/// HashMap mapping PID to (virtual_memory, resident_memory) values in KB
#[cfg(target_os = "macos")]
pub fn fetch_memory_map() -> HashMap<u32, ProcessMemory> {
    let mut map = HashMap::new();

    let output = Command::new("ps").args(&["-axo", "pid,vsz,rss"]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines().skip(1) {
            // Skip header line
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 3 {
                if let (Ok(pid), Ok(vsz), Ok(rss)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<u64>(),
                    parts[2].parse::<u64>(),
                ) {
                    let memory = ProcessMemory {
                        virtual_memory: vsz,
                        resident_memory: rss,
                    };
                    map.insert(pid, memory);
                }
            }
        }
    }

    map
}

/// Get process priority information for a specific PID
///
/// # Arguments
/// * `pid` - Process ID
/// * `priority_map` - HashMap containing priority data
///
/// # Returns
/// ProcessPriority with priority and nice values, or default values if not found
#[cfg(target_os = "macos")]
pub fn get_process_priority(
    pid: u32,
    priority_map: &HashMap<u32, ProcessPriority>,
) -> ProcessPriority {
    priority_map
        .get(&pid)
        .cloned()
        .unwrap_or_else(|| ProcessPriority {
            priority: "?".to_string(),
            nice: "?".to_string(),
        })
}

/// Get process memory information for a specific PID
///
/// # Arguments
/// * `pid` - Process ID
/// * `memory_map` - HashMap containing memory data
/// * `fallback_virt` - Fallback virtual memory value from sysinfo
/// * `fallback_res` - Fallback resident memory value from sysinfo
///
/// # Returns
/// ProcessMemory with virtual and resident memory values
#[cfg(target_os = "macos")]
pub fn get_process_memory(
    pid: u32,
    memory_map: &HashMap<u32, ProcessMemory>,
    fallback_virt: u64,
    fallback_res: u64,
) -> ProcessMemory {
    memory_map
        .get(&pid)
        .cloned()
        .unwrap_or_else(|| ProcessMemory {
            virtual_memory: fallback_virt,
            resident_memory: fallback_res,
        })
}

/// Stub implementations for non-macOS platforms
#[cfg(not(target_os = "macos"))]
pub fn fetch_priority_map() -> HashMap<u32, ProcessPriority> {
    HashMap::new()
}

#[cfg(not(target_os = "macos"))]
pub fn fetch_memory_map() -> HashMap<u32, ProcessMemory> {
    HashMap::new()
}

#[cfg(not(target_os = "macos"))]
pub fn get_process_priority(
    _pid: u32,
    _priority_map: &HashMap<u32, ProcessPriority>,
) -> ProcessPriority {
    ProcessPriority {
        priority: "N/A".to_string(),
        nice: "N/A".to_string(),
    }
}

#[cfg(not(target_os = "macos"))]
pub fn get_process_memory(
    _pid: u32,
    _memory_map: &HashMap<u32, ProcessMemory>,
    fallback_virt: u64,
    fallback_res: u64,
) -> ProcessMemory {
    ProcessMemory {
        virtual_memory: fallback_virt,
        resident_memory: fallback_res,
    }
}

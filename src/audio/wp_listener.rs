use anyhow::{Result, anyhow, bail};
use async_channel::Sender;
use nix::libc;
use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

#[derive(Debug, PartialEq, Clone)]
pub struct CurrentSink {
    pub volume: f32,
    pub muted: bool,
}

impl CurrentSink {
    pub fn new(s: String) -> Result<Self> {
        let muted = s.contains("[MUTED]");
        let volume = s
            .split_once("Volume: ")
            .map(|(_, val)| val.replace("[MUTED]", ""))
            .and_then(|val| val.trim().parse::<f32>().ok())
            .ok_or_else(|| anyhow!("Error parsing volume of: {}", s))?;

        Ok(Self { muted, volume })
    }
}

pub fn get_current_volume() -> Result<CurrentSink> {
    let output = Command::new("wpctl")
        .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
        .output()?;

    if output.status.success() {
        let sink_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        CurrentSink::new(sink_str)
    } else {
        bail!("Error starting \"wpctl\"")
    }
}

pub fn watch_volume_changes(sender: Sender<CurrentSink>) {
    std::thread::spawn(move || unsafe {
        let mut child = Command::new("pactl")
            .arg("subscribe")
            .stdout(Stdio::piped())
            .pre_exec(|| {
                {
                    libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM)
                };
                Ok(())
            })
            .spawn()
            .expect("Error executing \"pactl\"");

        let stdout = child.stdout.take().unwrap();
        let reader = BufReader::new(stdout);
        let mut last_sink: Option<CurrentSink> = None;

        for line in reader.lines().flatten() {
            if line.contains("on sink") || line.contains("on server") {
                if let Ok(sink) = get_current_volume() {
                    if Some(&sink) != last_sink.as_ref() {
                        last_sink = Some(sink.clone());
                        let _ = sender.send_blocking(sink);
                    }
                }
            }
        }
    });
}

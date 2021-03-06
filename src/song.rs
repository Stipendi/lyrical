use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

pub const BREAKPOINTS: [Breakpoint; 5] = [
    Breakpoint::new('#', 4, 1),
    Breakpoint::new('$', 4, 2),
    Breakpoint::new('%', 4, 4),
    Breakpoint::new('&', 4, 8),
    Breakpoint::new('?', 4, 16),
];

pub struct Breakpoint {
    pub character: char,
    pub signature: u32,
    pub length: u32,
}

impl Breakpoint {
    pub const fn new(character: char, signature: u32, length: u32) -> Self {
        Breakpoint {
            character,
            signature,
            length,
        }
    }
}

pub struct Song {
    pub data: File,
    pub bpm: u32,
    pub name: String,
    pub lyrics: String,

    scan_state: ScanState,
}

impl Song {
    pub fn new(data: File, name: String, bpm: u32, lyrics: String) -> Self {
        Song {
            data,
            name,
            bpm,
            lyrics,
            scan_state: ScanState::default(),
        }
    }

    // TODO: Figure out how to add a hyperlink
    /// A method that returns a Vec of things to do.
    pub fn parse_lyrics(&mut self) -> Vec<Operation> {
        let mut operations: Vec<Operation> = Vec::with_capacity(self.lyrics.len());
        // Iterate over every character in the lyrics
        for char in self.lyrics.chars() {
            if self.scan_state.escaped {
                self.scan_state.escaped = false;
                operations.push(Operation::Print(char));
                continue;
            } else if char == '\\' {
                self.scan_state.escaped = true;
                continue;
            }

            let search_for_break = BREAKPOINTS.iter().find(|value| value.character == char);
            match search_for_break {
                Some(breakpoint) => {
                    self.scan_state.ignore_spaces = true;
                    let pause_in_secs = 60.0 * breakpoint.length as f64
                        / breakpoint.signature as f64
                        / self.bpm as f64;
                    let duration = Duration::from_secs_f64(pause_in_secs);
                    operations.push(Operation::Pause(duration));
                }
                None => {
                    if self.scan_state.ignore_spaces && char == ' ' {
                        continue;
                    } else {
                        self.scan_state.ignore_spaces = false;
                        operations.push(Operation::Print(char));
                    }
                }
            };
        }

        operations
    }
}

pub enum Operation {
    Print(char),
    Pause(Duration),
}

pub fn try_get_song_from_file(audio_data: File, song_path: PathBuf) -> Option<Song> {
    let song_data = match fs::read_to_string(song_path) {
        Ok(value) => value,
        Err(_) => return None,
    };

    let mut name_buffer = String::new();
    let mut finding_name = true;

    let mut bpm_buffer = String::new();
    let mut finding_bpm = false;
    let mut bpm: Option<u32> = None;
    let mut wait_until_newline = false;

    let mut finding_lyrics = false;
    let mut lyrics_buffer = String::new();

    let mut ignore_whitespace = false;

    for char in song_data.chars() {
        if ignore_whitespace && (char == '\n' || char == ' ' || char == '\t') {
            continue;
        }

        if wait_until_newline {
            if char == '\n' {
                wait_until_newline = false;
                ignore_whitespace = true;
            }
            continue;
        }

        if finding_name && char == '\n' {
            if name_buffer.is_empty() {
                return None;
            }
            finding_name = false;
            finding_bpm = true;
            ignore_whitespace = true;
            continue;
        }

        if finding_name {
            name_buffer.push(char);
        }

        if finding_bpm {
            ignore_whitespace = false;
            if char.is_numeric() {
                bpm_buffer.push(char);
            } else {
                if bpm_buffer.is_empty() {
                    return None;
                }
                finding_bpm = false;
                match bpm_buffer.parse::<u32>() {
                    Ok(value) => {
                        bpm = Some(value);
                        wait_until_newline = true;
                        finding_lyrics = true;
                        continue;
                    }
                    Err(_) => return None,
                }
            }
        }

        if finding_lyrics {
            ignore_whitespace = false;
            lyrics_buffer.push(char);
        }
    }

    if let None = bpm {
        return None;
    }

    Some(Song::new(
        audio_data,
        name_buffer,
        bpm.unwrap(),
        lyrics_buffer,
    ))
}

struct ScanState {
    ignore_spaces: bool,
    escaped: bool,
}

impl Default for ScanState {
    fn default() -> Self {
        ScanState {
            ignore_spaces: false,
            escaped: false,
        }
    }
}

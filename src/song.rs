use std::fs::File;

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
}

impl Song {
    pub fn new(data: File, name: String, bpm: u32, lyrics: String) -> Self {
        Song {
            data,
            name,
            bpm,
            lyrics,
        }
    }
}

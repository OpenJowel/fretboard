use midir::{MidiOutput, MidiOutputConnection};
use std::cell::RefCell;
use std::{thread, time};
use std::error::Error;


pub struct MidiPlayer {
  connection: Option<RefCell<MidiOutputConnection>>,
  instrument: u8,
  fundamental: u8
}

impl MidiPlayer{

  pub fn new() -> Result<Self, Box<dyn Error>> {
    let mut new_midi_player = Self{
      connection:None,
      instrument:27, // Electric guitar
      fundamental: 40 // E1 (Lowest guitar string in standard tuning)
    };
    
      new_midi_player.connect()?;
      Ok(new_midi_player)
  }

  pub fn connect(&mut self) -> Result<(), Box<dyn Error>>{
    let midi_output = MidiOutput::new("MIDI Output")?;
    let outputs = midi_output.ports();

    if outputs.is_empty() {
      println!("No MIDI outputs available.");
      return Ok(())
    }

    for (i, port) in outputs.iter().enumerate() {
      let name = midi_output.port_name(port)?;
      println!("{}: {}", i, name);
    }

    // Use the first available output
    let port = &outputs[0]; // Get the first MIDI output
    let connection = midi_output.connect(port, "midi-output")?;
    self.connection = Some(connection.into());

    Ok(())
  }


  pub fn play_notes(&self, notes: &[i32]) {
    // TODO : play the chords in a separate thread. We don't want this to delay code execution !

    if let Some(ref connection) = self.connection {
      let mut conn = connection.borrow_mut(); // Borrow mutably
      conn.send(&[0xC0, self.instrument]); // Select instrument

      for &note in notes {
        conn.send(&[0x90, self.fundamental + note as u8, 127]); // Note On
        thread::sleep(time::Duration::from_millis(250)); // Hold the note for 500 ms
      }

      thread::sleep(time::Duration::from_millis(500)); // Hold the note for 500 ms

      for &note in notes {
        conn.send(&[0x80, self.fundamental + note as u8, 0]); // Note Off
      }
    }
  }
}
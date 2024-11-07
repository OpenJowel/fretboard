use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::error::Error;


pub struct MidiPlayer {
  connection: Option<Arc<Mutex<MidiOutputConnection>>>,
  instrument: u8,
  lowest_open_note: u8
}

impl MidiPlayer{

  pub fn new() -> Result<Self, Box<dyn Error>> {
    let midi_out = MidiOutput::new("MIDI Output")?;

    let ports = midi_out.ports();
    let port = ports.get(0).ok_or("No MIDI output ports available")?;
    let connection = Some(Arc::new(Mutex::new(midi_out.connect(port, "midi-output")?)));

    Ok(Self {
      connection,
      instrument: 27, // Electric guitar
      lowest_open_note: 40 // E1 (Lowest guitar string in standard tuning)
    })
  }


  pub fn play_notes(&self, notes: Vec<i32>) {
    if let Some(connection) = &self.connection {

      let connection = Arc::clone(connection);
      let instrument = self.instrument;
      let lowest_open_note = self.lowest_open_note;

      thread::spawn(move || {
        if let Ok(mut conn) = connection.lock(){
          conn.send(&[0xC0, instrument]).ok(); // Select instrument

          for note in &notes {
            conn.send(&[0x90, lowest_open_note + *note as u8, 127]).ok(); // Note On
            thread::sleep(time::Duration::from_millis(120));
          }

          thread::sleep(time::Duration::from_millis(200));

          for note in &notes {
            conn.send(&[0x80, lowest_open_note + *note as u8, 0]).ok(); // Note Off
          }
        }
      });
    }
  }
}
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::error::Error;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum GuitarType {
  AcousticNylon = 24,
  AcousticSteel = 25,
  ElectricJazz = 26,
  ElectricClean = 27,
  ElectricMuted = 28,
  Overdriven = 29,
  Distortion = 30,
  Harmonics = 31,
}


pub static GUITAR_TYPES: Lazy<HashMap<GuitarType, &'static str>> = Lazy::new(|| {
  let mut map = HashMap::new();
  map.insert(GuitarType::AcousticNylon, "Acoustic Nylon");
  map.insert(GuitarType::AcousticSteel, "Acoustic Steel");
  map.insert(GuitarType::ElectricJazz, "Electric Jazz");
  map.insert(GuitarType::ElectricClean, "Electric Clean");
  map.insert(GuitarType::ElectricMuted, "Electric Muted");
  map.insert(GuitarType::Overdriven, "Overdriven");
  map.insert(GuitarType::Distortion, "Distortion");
  map.insert(GuitarType::Harmonics, "Harmonics");
  map
});


impl GuitarType {
  pub fn available_guitar_types() -> &'static HashMap<GuitarType, &'static str> {
    &GUITAR_TYPES
  }
}


pub struct MidiPlayer {
  connection: Option<Arc<Mutex<MidiOutputConnection>>>,
  guitar_type: GuitarType,
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
      guitar_type: GuitarType::AcousticNylon,
      lowest_open_note: 40 // E1 (Lowest guitar string in standard tuning)
    })
  }


  pub fn get_guitar_type(&self) -> &GuitarType {
    return &self.guitar_type;
  }


  pub fn set_guitar_type(&mut self, guitar_type: GuitarType) {
    self.guitar_type = guitar_type;
  }


  pub fn play_notes(&self, notes: Vec<i32>) {
    if let Some(connection) = &self.connection {

      let connection = Arc::clone(connection);
      let guitar_type = self.guitar_type;
      let lowest_open_note = self.lowest_open_note;

      thread::spawn(move || {
        if let Ok(mut conn) = connection.lock(){
          conn.send(&[0xC0, guitar_type as u8]).ok(); // Select guitar sound type

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
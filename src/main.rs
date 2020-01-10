const REFERENCE_FREQUENCY: f64 = 261.6255653005986;

/// Structure to hold data relating to a note
/// Accepts an interval and an octave which will be used to calculate the note and frequency
struct NoteFrequency {
    reference_frequency: f64,
    interval: i32,
    octave: i32,
}

/// Implementation of NoteFrequency
impl NoteFrequency {
    /// Calculate the frequency from the interval and octave
    fn frequency (&self) -> f64 {
        let two_f64: f64 = 2.0;
        return self.reference_frequency * two_f64.powf(
            (self.octave as f64 - 4.0) + (self.interval as f64 / 12.0)
        );
    }
    /// Calculate the note from the interval and octave
    fn note (&self) -> String {
        const NOTES: [&'static str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        return format!("{}{}", NOTES[self.interval as usize], self.octave);
    }
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}

fn main() {
    let octave: i32 = simple_user_input::get_input("Input Octave: ").parse().unwrap();
    let interval: i32 = simple_user_input::get_input("Input Interval: ").parse().unwrap();

    let note = NoteFrequency {
        reference_frequency: REFERENCE_FREQUENCY,
        octave: octave,
        interval: interval
    };

    println!("The Frequency of the note is: {:.2}", note.frequency());
    println!("The note is {}", note.note());
}

#[test]
fn ensure_note_and_frequency_correct_for_a4() {
    let note = NoteFrequency {
        reference_frequency: REFERENCE_FREQUENCY,
        octave: 4,
        interval: 9
    };

    assert_eq!(439.99999999999994, note.frequency());
}
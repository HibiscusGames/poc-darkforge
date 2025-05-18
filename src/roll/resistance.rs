use super::ResistanceOutcome;

trait Resistance {
    fn roll(&self, n: u8) -> ResistanceOutcome;
}

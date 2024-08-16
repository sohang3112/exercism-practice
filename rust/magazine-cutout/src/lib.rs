use std::collections::HashMap;
use std::array::IntoIter;
use counter::Counter;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mag_counter = IntoIter::into_iter(magazine).collect::<Counter<_>>();
    let note_counter = note.collect::<Counter<_>>();
    mag_counter.zip(note_counter).all(u32::lt)
    
}

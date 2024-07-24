use std::collections::HashMap;
use crate::maj_event::{MajEvent, ToMajEvent};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct EventEmitter<R: BufRead> {
    xml_reader: Reader<R>,
    buf: Vec<u8>,
}

impl<'a, R: BufRead> Iterator for EventEmitter<R> {
    type Item = MajEvent;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.xml_reader.read_event_into(&mut self.buf) {
                Err(e) => {
                    panic!(
                        "Error reading XML at position {}: {:?}",
                        self.xml_reader.buffer_position(),
                        e
                    );
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => return None,
                Ok(Event::Start(ref e)) => {
                    assert_eq!(e.name().as_ref(), b"mjloggm");
                    let ver = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key.as_ref() == b"ver")
                        .unwrap()
                        .unwrap();
                    let ver = ver.value.as_ref();
                    if ver != b"2.3" {
                        panic!(
                            "Unsupported mjlog version: {:?}. The only supported version is 2.3",
                            ver
                        );
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    if let Some(ev) = e.to_maj_event() {
                        if let MajEvent::Go { r#type } = ev {
                            if !r#type.applicable() {
                                return None;
                            }
                        }
                        return Some(ev);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn parse_file_iter(path: impl AsRef<Path>) -> EventEmitter<BufReader<File>> {
    let xml_reader: Reader<BufReader<File>> = quick_xml::Reader::from_file(path).unwrap();
    EventEmitter {
        xml_reader,
        buf: Vec::new(),
    }
}

pub fn guess_user_id(path: impl AsRef<Path>) -> Option<String> {
    let mut user_id_counter: HashMap<String, u64> = HashMap::new();
    path.as_ref().read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let _path_str = path.to_str().unwrap().to_string();
        if path.is_file() {
            parse_file_iter(path).for_each(|event| {
                if let MajEvent::UN { id, .. } = event {
                    for id in id {
                        *user_id_counter.entry(id).or_insert(0) += 1;
                    }
                }
            });
        }
    });
    user_id_counter
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(id, _)| id)
}

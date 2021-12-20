extern crate quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
struct LibraryProperties {
  major_version: String,         // Major Version
  minor_version: String,         // Minor Version
  application_version: String,   // Application Version
  date: String,                  // Date
  features: String,              // Features
  show_content_ratings: bool,    // Show Content Ratings
  library_persistent_id: String, // Library Persistent ID
  music_folder: String,          // Music Folder
}

fn parse_library(
  r: &mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
) -> Result<(), quick_xml::Error> {
  let mut library_properties = LibraryProperties {
    major_version: String::new(),
    minor_version: String::new(),
    application_version: String::new(),
    date: String::new(),
    features: String::new(),
    show_content_ratings: false,
    library_persistent_id: String::new(),
    music_folder: String::new(),
  };

  // buffer for nested reader
  let mut skip_buf = Vec::new();

  loop {
    skip_buf.clear();

    match &r.read_event(&mut skip_buf)? {
      Event::Start(element) => {
        assert_eq!(
          element.name(),
          b"dict",
          "First open tag should be dict, got {:?}",
          String::from_utf8_lossy(element.name())
        );
        break;
      }
      Event::Eof => {
        panic!("Unexpected end of file, failed to find opening dict tag");
      }
      _ => (),
    }
  }

  let mut key_stack = Vec::new();

  loop {
    skip_buf.clear();

    match &r.read_event(&mut skip_buf)? {
      Event::Start(element) | Event::Empty(element) => match element.name() {
        b"key" => {
          // get key text
          let key_text = r.read_text(b"key", &mut skip_buf)?;
          key_stack.push(key_text);
        }
        _ => {
          let tag_type = element.name();

          let tag_text;
          if tag_type == b"true" {
            tag_text = "true".to_string();
          } else if tag_type == b"false" {
            tag_text = "false".to_string();
          } else {
            tag_text = r.read_text(tag_type, &mut Vec::new())?;
          }

          let key = key_stack.pop().expect("No key found for tag");

          println!(
            "\n[Property: {:?}]\n|-- Type: {:?}\n|-- Value: {:?}\n",
            key,
            String::from_utf8_lossy(tag_type),
            tag_text
          );

          match key.as_str() {
            "Major Version" => library_properties.major_version = tag_text,
            "Minor Version" => library_properties.minor_version = tag_text,
            "Application Version" => library_properties.application_version = tag_text,
            "Date" => library_properties.date = tag_text,
            "Features" => library_properties.features = tag_text,
            "Show Content Ratings" => library_properties.show_content_ratings = tag_text == "true",
            "Library Persistent ID" => library_properties.library_persistent_id = tag_text,
            "Music Folder" => library_properties.music_folder = tag_text,
            _ => (),
          }
        }
      },
      Event::End(element) => match element.name() {
        b"dict" => {
          println!("dict closing tag found, ending library parsing");
          break;
        }
        _ => {
          println!(
            "unexpected end tag found: {:?}",
            String::from_utf8_lossy(element.name())
          );
        }
      },
      _ => {
        println!("unexpected event found");
      }
    }
  }

  println!("parsed library properties: {:?}", library_properties);

  return Ok(());
}

fn parse_itunes_file(filepath: &str) -> Result<(), quick_xml::Error> {
  println!("Parsing file: \"{}\"", filepath);

  let mut reader = Reader::from_file(filepath)?;

  let mut buf = Vec::new();

  loop {
    buf.clear();

    match reader.read_event(&mut buf)? {
      Event::Start(element) => {
        // Take action on specific start tags
        match element.name() {
          b"plist" => {
            println!("plist opening tag found, beginning library parsing");

            let res = parse_library(&mut reader);
            res.expect("Error parsing library");
          }
          _ => {}
        }
      }
      Event::End(element) => match element.name() {
        b"plist" => {
          println!("plist close tag found, finished library parsing");
        }
        _ => {}
      },
      Event::Eof => break,
      _ => {}
    }
  }

  Ok(())
}

fn main() {
  parse_itunes_file("iTunesLibrarySample.xml").expect("ohno");
}

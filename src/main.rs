/*
Major Version
Minor Version
Application Version
Date
Features
Show Content Ratings
Library Persistent ID
Music Folder
*/
#[derive(Debug)]
struct LibraryProperties {
  major_version: String,
  minor_version: String,
  application_version: String,
  date: String,
  features: String,
  show_content_ratings: bool,
  music_folder: String,
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
    music_folder: String::new(),
  };

  // buffer for nested reader
  let mut skip_buf = Vec::new();

  loop {
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

extern crate quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;

// demonstrate how to nest readers
// This is useful for when you need to traverse
// a few levels of a document to extract things.
fn parse_itunes_file(filepath: &str) -> Result<(), quick_xml::Error> {
  println!("Parsing file: \"{}\"", filepath);

  let mut reader = Reader::from_file(filepath)?;

  let mut buf = Vec::new();

  loop {
    buf.clear();

    match reader.read_event(&mut buf)? {
      Event::Start(element) => {
        // let tag_name = String::from_utf8(element.name().to_vec()).unwrap();

        // Take action on specific start tags
        match element.name() {
          b"plist" => {
            println!("plist opening tag found, beginning library parsing");

            let res = parse_library(&mut reader);
            res.expect("Error parsing library");
          }
          // b"key" => {
          //   let key_value = reader
          //     .read_text(b"key", &mut Vec::new())
          //     .expect("Cannot decode text value");

          //   println!("key: {}", key_value);
          //   key_stack.push(key_value)
          // }
          // b"dict" => match key_stack.last() {
          //   Some(key) => {
          //     println!("found start dict for key {}", key);
          //     match key.as_str() {
          //       "Tracks" => {
          //         println!("found start dict for key Tracks");
          //         let mut tracks_buf = Vec::new();
          //         loop {
          //           match reader.read_event(&mut tracks_buf)? {
          //             Event::Start(element) => {
          //               let tag_name = String::from_utf8(element.name().to_vec()).unwrap();
          //               println!("found start dict for key Tracks: {}", tag_name);
          //             }
          //             Event::End(element) => {
          //               let tag_name = String::from_utf8(element.name().to_vec()).unwrap();
          //               println!("found end dict for key Tracks: {}", tag_name);
          //               break;
          //             }
          //             _ => (),
          //           }
          //         }
          //       }
          //       _ => (),
          //     }
          //   }
          //   _ => println!("found start dict"),
          // },
          _ => {}
        }
      }
      Event::End(element) => {
        // let tag_name = String::from_utf8(element.name().to_vec()).unwrap();

        // let tabs = key_stack.len() - 1;
        // for _ in 0..tabs {
        //   print!("--");
        // }
        // println!("-| Closing tag: {:?}", tag_name);

        // assert_eq!(key_stack.pop(), Some(tag_name));

        match element.name() {
          b"plist" => {
            println!("plist close tag found, finished library parsing");
          }
          // b"dict" => {
          //   println!("found end dict for key {}", key_stack.last().unwrap());
          // }
          _ => {}
        }
      }
      Event::Eof => break,
      _ => {}
    }
  }
  //     }
  //     Event::Eof => break,
  //     _ => {}
  //   }
  //   buf.clear();
  // }

  Ok(())
}

fn main() {
  parse_itunes_file("iTunesLibrarySample.xml").expect("shit");
}

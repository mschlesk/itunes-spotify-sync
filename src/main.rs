// // fn main() {
// //     println!("Hello, world!");
// // }

// extern crate quick_xml;

// fn main() {
//     use quick_xml::events::Event;
//     use quick_xml::Reader;

//     let xml = "<tag1>text1</tag1><tag1>text2</tag1>\
//                <tag1>text3</tag1><tag1><tag2>text4</tag2></tag1>";

//     // let mut reader = Reader::from_str(xml);
//     let mut reader = Reader::from_file("../itunesLibrary.xml")?;
//     // reader.trim_text(true);

//     let mut txt = Vec::new();
//     let mut buf = Vec::new();

//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(ref e)) if e.name() == b"tag2" => {
//                 txt.push(
//                     reader
//                         .read_text(b"tag2", &mut Vec::new())
//                         .expect("Cannot decode text value"),
//                 );
//                 println!("{:?}", txt);
//             }
//             Ok(Event::Eof) => break, // exits the loop when reaching end of file
//             Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//             _ => (), // There are several other `Event`s we do not consider here
//         }
//         buf.clear();
//     }
// }

extern crate quick_xml;
use quick_xml::events::Event;
use quick_xml::Reader;

// demonstrate how to nest readers
// This is useful for when you need to traverse
// a few levels of a document to extract things.
fn parse_itunes_file(filepath: &str) -> Result<(), quick_xml::Error> {
  println!("Parsing file: \"{}\"", filepath);

  let mut reader = Reader::from_file(filepath)?;

  // let mut buf = Vec::new();

  // buffer for nested reader
  let mut skip_buf = Vec::new();

  let mut key_stack: Vec<String> = Vec::new();

  // Main loop
  // loop {
  //   match reader.read_event(&mut buf)? {
  //     Event::Start(element) if element.name() == b"plist" => {
  loop {
    skip_buf.clear();

    match reader.read_event(&mut skip_buf)? {
      Event::Start(element) => {
        // let tag_name = String::from_utf8(element.name().to_vec()).unwrap();

        // let tabs = key_stack.len();
        // for _ in 0..tabs {
        //   print!("--");
        // }
        // println!("-| Opening tag: {:?}", tag_name);

        // key_stack.push(tag_name);

        // Take action on specific start tags
        match element.name() {
          b"key" => {
            let key_value = reader
              .read_text(b"key", &mut Vec::new())
              .expect("Cannot decode text value");

            println!("key: {}", key_value);
            key_stack.push(key_value)
          }
          b"dict" => match key_stack.last() {
            Some(key) => {
              println!("found start dict for key {}", key);
              match key.as_str() {
                "Tracks" => {
                  println!("found start dict for key Tracks");
                  let mut tracks_buf = Vec::new();
                  loop {
                    match reader.read_event(&mut tracks_buf)? {
                      Event::Start(element) => {
                        let tag_name = String::from_utf8(element.name().to_vec()).unwrap();
                        println!("found start dict for key Tracks: {}", tag_name);
                      }
                      Event::End(element) => {
                        let tag_name = String::from_utf8(element.name().to_vec()).unwrap();
                        println!("found end dict for key Tracks: {}", tag_name);
                        break;
                      }
                      _ => (),
                    }
                  }
                }
                _ => (),
              }
            }
            _ => println!("found start dict"),
          },
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
            println!("Finished parsing");
          }
          b"dict" => {
            println!("found end dict for key {}", key_stack.last().unwrap());
          }
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

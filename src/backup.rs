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
// a structure to capture the rows we've extracted
// from a ECMA-376 table in document.xml
#[derive(Debug, Clone)]
struct TableStat {
  index: u8,
  rows: Vec<Vec<String>>,
}
// demonstrate how to nest readers
// This is useful for when you need to traverse
// a few levels of a document to extract things.
fn main() -> Result<(), quick_xml::Error> {
  let mut buf = Vec::new();
  // buffer for nested reader
  // let mut skip_buf = Vec::new();
  // let mut count = 0;
  // let mut reader = Reader::from_file("tests/documents/document.xml")?;
  let mut reader = Reader::from_file("iTunesLibrarySample.xml")?;
  // let mut found_tables = Vec::new();

  // let mut txt = Vec::new();
  loop {
    match reader.read_event(&mut buf)? {
      Event::Start(ref e) => match e.name() {
        b"key" => {
          println!(
            "found start key: {}",
            String::from_utf8(e.name().to_vec()).unwrap()
          );

          println!(
            "{:?}",
            reader
              .read_text(e.name(), &mut Vec::new())
              .expect("Cannot decode text value")
          );
        }
        _ => (),
      },
      // Event::Text(ref e) => {
      //   // println!(
      //   //   "text value: {}",
      //   //   e.unescape_and_decode_with_custom_entities(&reader, &custom_entities)
      //   //     .unwrap()
      //   // );
      // }
      Event::Eof => break,
      _ => (),
    }
    buf.clear();
  }
  Ok(())
}

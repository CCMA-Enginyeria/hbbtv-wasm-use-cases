use std::borrow::Borrow;

use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "tt")]
struct Item {
    name: String,
    source: String,
    #[serde(rename = "@xmlns:tt")]
    pub name_attr: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "tt")]
pub struct TT {
    #[serde(rename = "@xmlns:tt")]
    pub xmlns_tt: String,
    #[serde(rename = "@xmlns:ttp")]
    pub xmlns_ttp: String,
    #[serde(rename = "@xmlns:tts")]
    pub xmlns_tts: String,
    #[serde(rename = "@xmlns:ebuttm")]
    pub xmlns_ebuttm: String,
    #[serde(rename = "@xmlns:ebutts")]
    pub xmlns_ebutts: String,
    #[serde(rename = "@timeBase")]
    pub ttp_time_base: String,
    #[serde(rename = "@lang")]
    pub xml_lang: String,
    #[serde(rename = "@cellResolution")]
    pub ttp_cell_resolution: String,
    pub head: Head,
    pub body: Body,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    pub metadata: Metadata,
    pub styling: Styling,
    pub layout: Layout,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "@ppd")]
    pub ppd: String,
    #[serde(rename = "documentMetadata")]
    pub document_metadata: DocumentMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentMetadata {
    #[serde(rename = "conformsToStandard")]
    pub conforms_to_standard: String,
    #[serde(rename = "documentCountryOfOrigin")]
    pub document_country_of_origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Styling {
    #[serde(rename = "style")]
    pub styles: Vec<Style>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Style {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@fontFamily")]
    pub font_family: Option<String>,
    #[serde(rename = "@fontStyle")]
    pub font_style: Option<String>,
    #[serde(rename = "@fontWeight")]
    pub font_weight: Option<String>,
    #[serde(rename = "@textDecoration")]
    pub text_decoration: Option<String>,
    #[serde(rename = "@color")]
    pub color: Option<String>,
    #[serde(rename = "@textAlign")]
    pub text_align: Option<String>,
    #[serde(rename = "@backgroundColor")]
    pub background_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layout {
    #[serde(rename = "region")]
    pub regions: Vec<Region>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@origin")]
    pub origin: Option<String>,
    #[serde(rename = "@extent")]
    pub extent: Option<String>,
    #[serde(rename = "@padding")]
    pub padding: Option<String>,
    #[serde(rename = "@displayAlign")]
    pub display_align: Option<String>,
    #[serde(rename = "@writingMode")]
    pub writing_mode: Option<String>,
    #[serde(rename = "@showBackground")]
    pub show_background: Option<String>,
    #[serde(rename = "@overflow")]
    pub overflow: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub div: Div,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Div {
    pub p: Vec<P>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct P {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@region")]
    pub region: Option<String>,
    #[serde(rename = "@begin")]
    pub begin: Option<String>,
    #[serde(rename = "@end")]
    pub end: Option<String>,
    #[serde(rename = "$value")]
    children: Option<Vec<Choice>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Choice {
    #[serde(rename = "span")]
    Span(Span),
    #[serde(rename = "br")]
    Br(Br),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Span {
    #[serde(rename = "@style")]
    pub style: String,
    #[serde(rename = "$value")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Br {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subtitles {}

impl Subtitles {
    pub fn new() -> Subtitles {
        Subtitles {}
    }

    pub fn load(&mut self, xml: &str) {
        //log(&format!("hola? {}", xml));
        let object: TT = from_str(xml).unwrap();
        //let object: TT = from_str(&xml).unwrap();

        log(&format!("object {}", object.body.div.p.len()));
        for (index, p) in object.body.div.p.iter().enumerate() {
            let x = Vec::new();
            for (index, child) in p.children.as_ref().unwrap_or(&x).iter().enumerate() {
                if let Choice::Span(span) = child {
                    log(&format!(
                        "Flipa {}",
                        span.text.as_ref().unwrap_or(&"".to_string())
                    ));
                } else if let Choice::Br(br) = child {
                    log("no Flipa");
                }
                // if let Choice::Span(span) = child {
                //     Some(span.style.clone())
                // } else {
                //     None
                // }
            }
        }

        //log(&format!("object {}", object.name_attr));

        /*
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut count = 0;
        let mut txt = Vec::new();
        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"tt:tt" => {
                        let x = e
                            .attributes()
                            .map(|a| a.unwrap().value.as_ref())
                            .collect::<Vec<_>>()
                            .to_string();
                        //let y = x.collect::<Vec<_>>();
                        //log(&format!("attributes values: {}", y.join(","));
                    }
                    // b"tt:tt" => log(
                    //     "attributes values: {:?}",
                    //     e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>(),
                    // ),
                    b"tag2" => count += 1,
                    _ => (),
                },
                Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        */
    }
}

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Index};
use wasm_bindgen::prelude::*;
mod cuepoints;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen()]
    fn showSubtitle(s: &str, text: &str);
    fn hideSubtitle(s: &str);
    fn existSubtitle(s: &str) -> bool;
    fn startDebugTimer(s: &str);
    fn endDebugTimer(s: &str);
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
    #[serde(rename = "@fontSize")]
    pub font_size: Option<String>,
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
    #[serde(rename = "@style")]
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    #[serde(rename = "@style")]
    pub style: Option<String>,
    pub div: Div,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Div {
    #[serde(rename = "@style")]
    pub style: Option<String>,
    pub p: Vec<P>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct P {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@region")]
    pub region: Option<String>,
    #[serde(rename = "@begin")]
    pub begin: String,
    #[serde(rename = "@end")]
    pub end: String,
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

fn time_to_ms(time: &str) -> Result<i32, String> {
    let blocks: Vec<&str> = time.split(':').collect();
    if blocks.len() != 3 {
        return Err("Format de temps incorrecte".to_string());
    }

    let hours = blocks[0]
        .parse::<i32>()
        .map_err(|_| "Error en parsejar les hores".to_string())?;
    let minutes = blocks[1]
        .parse::<i32>()
        .map_err(|_| "Error en parsejar els minuts".to_string())?;

    let seconds_block: Vec<&str> = blocks[2].split('.').collect();
    let seconds = seconds_block[0]
        .parse::<i32>()
        .map_err(|_| "Error en parsejar els segons".to_string())?;

    let mut ms = (hours * 3600 + minutes * 60 + seconds) * 1000;

    if seconds_block.len() > 1 {
        let frames = seconds_block[1]
            .parse::<i32>()
            .map_err(|_| "Error en parsejar els mil·lisegons".to_string())?;
        ms += frames;
    }

    Ok(ms)
}

pub struct SubtilesAction {
    index: usize,
    is_show_action: bool,
}

pub struct Subtitles {
    pub tt: Option<TT>,
    pub cuepoints: cuepoints::Cuepoints,
    pub cuepoint_to_subtitles_action: HashMap<String, SubtilesAction>,
    pub styles_index: HashMap<String, usize>,
    pub region_index: HashMap<String, usize>,
}

impl Subtitles {
    pub fn new() -> Subtitles {
        Subtitles {
            tt: None,
            cuepoints: cuepoints::Cuepoints::new(),
            cuepoint_to_subtitles_action: HashMap::new(),
            styles_index: HashMap::new(),
            region_index: HashMap::new(),
        }
    }

    pub fn load(&mut self, xml: &str) {
        //log(&format!("hola? {}", xml));
        self.tt = from_str(xml).unwrap();
        self.add_cuepoints();
        self.get_styles();
        self.get_regions();
        //let object: TT = from_str(&xml).unwrap();

        // log(&format!("object {}", object.body.div.p.len()));
        // for (index, p) in object.body.div.p.iter().enumerate() {
        //     let x = Vec::new();
        //     for (index, child) in p.children.as_ref().unwrap_or(&x).iter().enumerate() {
        //         if let Choice::Span(span) = child {
        //             log(&format!(
        //                 "Flipa {}",
        //                 span.text.as_ref().unwrap_or(&"".to_string())
        //             ));
        //         } else if let Choice::Br(br) = child {
        //             log("no Flipa");
        //         }
        //         // if let Choice::Span(span) = child {
        //         //     Some(span.style.clone())
        //         // } else {
        //         //     None
        //         // }
        //     }
        // }

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
    fn get_styles(&mut self) {
        for (index, s) in self
            .tt
            .as_ref()
            .unwrap()
            .head
            .styling
            .styles
            .iter()
            .enumerate()
        {
            self.styles_index.insert(s.id.clone(), index);
        }
    }
    fn get_regions(&mut self) {
        for (index, r) in self
            .tt
            .as_ref()
            .unwrap()
            .head
            .layout
            .regions
            .iter()
            .enumerate()
        {
            self.region_index.insert(r.id.clone(), index);
        }
    }
    fn add_cuepoints(&mut self) {
        if (self.tt.is_some()) {
            for (index, p) in self.tt.as_ref().unwrap().body.div.p.iter().enumerate() {
                let in_key = format!("in-{}", p.id);
                let in_cuepoint = cuepoints::Cuepoint {
                    id: in_key.clone(),
                    ms: time_to_ms(&p.begin).unwrap_or(-1),
                    timestopass: 0,
                    //callback: None,
                    negativemargin: None,
                    positivemargin: None,
                    once: false,
                };
                self.cuepoints.add_cuepoint(in_cuepoint);
                self.cuepoint_to_subtitles_action.insert(
                    in_key,
                    SubtilesAction {
                        index: index,
                        is_show_action: true,
                    },
                );

                let out_key = format!("out-{}", p.id);
                let out_cuepoint = cuepoints::Cuepoint {
                    id: out_key.clone(),
                    ms: time_to_ms(&p.end).unwrap_or(-1),
                    timestopass: 0,
                    //callback: None,
                    negativemargin: None,
                    positivemargin: None,
                    once: false,
                };
                self.cuepoints.add_cuepoint(out_cuepoint);
                self.cuepoint_to_subtitles_action.insert(
                    out_key,
                    SubtilesAction {
                        index: index,
                        is_show_action: false,
                    },
                );
            }
        }
    }
    pub fn update_subtitles_for_ms(&self, ms: i32) {
        let cues = self.cuepoints.get_cuepoints_by_time(ms);
        for (_index, cue) in cues.iter().enumerate() {
            let subtitle_action = self.cuepoint_to_subtitles_action.get(&cue.id);
            if (subtitle_action.is_some()) {
                let subtitle_action_unwrap = subtitle_action.unwrap();
                let p = self
                    .tt
                    .as_ref()
                    .unwrap()
                    .body
                    .div
                    .p
                    .get(subtitle_action_unwrap.index);
                if (p.is_some()) {
                    let p_unwrap = p.unwrap();
                    if (subtitle_action_unwrap.is_show_action == true) {
                        self.show_subtile(p_unwrap);
                    } else {
                        self.hide_subtile(p_unwrap);
                    }
                }
            }
        }
    }
    fn get_default_styles(&self) -> String {
        let mut styles: Vec<&String> = Vec::new();
        if (self.tt.is_some()) {
            if (self.tt.as_ref().unwrap().body.style.is_some()) {
                styles.push(self.tt.as_ref().unwrap().body.style.as_ref().unwrap());
            }
            if (self.tt.as_ref().unwrap().body.div.style.is_some()) {
                styles.push(self.tt.as_ref().unwrap().body.div.style.as_ref().unwrap());
            }
        }
        return styles.iter().map(|s| s.as_str()).collect();
    }

    fn get_style_string(&self, style_id: &String) -> String {
        let mut styles: Vec<String> = Vec::new();
        let style_index = self.styles_index.get(style_id);
        if (style_index.is_some()) {
            let style_index = style_index.unwrap().clone();
            let style = self
                .tt
                .as_ref()
                .unwrap()
                .head
                .styling
                .styles
                .get(style_index);

            if (style.is_some()) {
                let style = style.unwrap();
                if style.background_color.is_some() {
                    styles.push(format!(
                        "background-color:{}",
                        style.background_color.as_ref().unwrap()
                    ));
                }
                if (style.font_family.is_some()) {
                    styles.push(format!(
                        "font-family:{}",
                        style.font_family.as_ref().unwrap()
                    ));
                }
                if (style.font_size.is_some()) {
                    styles.push(format!("font-size:{}", style.font_size.as_ref().unwrap()));
                }
                if (style.font_style.is_some()) {
                    styles.push(format!("font-style:{}", style.font_style.as_ref().unwrap()));
                }
                if (style.font_weight.is_some()) {
                    styles.push(format!(
                        "font-weight:{}",
                        style.font_weight.as_ref().unwrap()
                    ));
                }
                if (style.text_align.is_some()) {
                    styles.push(format!("text-align:{}", style.text_align.as_ref().unwrap()));
                }
                if (style.color.is_some()) {
                    styles.push(format!("color:{}", style.color.as_ref().unwrap()));
                }
            }
        }
        return styles.join(";");
    }

    fn get_region_styles(&self, region_id: &String) -> String {
        let mut styles: Vec<String> = Vec::new();
        if (self.tt.is_some()) {
            let region_index = self.region_index.get(region_id);
            if (region_index.is_some()) {
                let region_index = region_index.unwrap().clone();
                let region = self
                    .tt
                    .as_ref()
                    .unwrap()
                    .head
                    .layout
                    .regions
                    .get(region_index);
                if (region.is_some()) {
                    let region = region.unwrap();
                    if (region.origin.is_some()) {
                        let region_splitted: Vec<&str> =
                            region.origin.as_ref().unwrap().split(" ").collect();
                        if region_splitted.len() == 2 {
                            styles.push(format!("left:{}", region_splitted[0]));
                            styles.push(format!("top:{}", region_splitted[1]));
                        }
                    }
                    if (region.extent.is_some()) {
                        let region_splitted: Vec<&str> =
                            region.extent.as_ref().unwrap().split(" ").collect();
                        if region_splitted.len() == 2 {
                            styles.push(format!("width:{}", region_splitted[0]));
                            styles.push(format!("height:{}", region_splitted[1]));
                        }
                    }
                    if (region.style.is_some()) {
                        styles.push(self.get_style_string(region.style.as_ref().unwrap()));
                    }

                    //style
                }

                /*
                var ttsPositionArray = nodeValue.split(" "),
                    top = ttsPositionArray[1],
                    left = ttsPositionArray[0],
                    height;
                el.style.top = top;
                el.style.left = left;
                 */

                // self.parse_tt_attributes("origin", region.origin);
                // self.parse_tt_attributes("extent", region.extent);
                // self.parse_tt_attributes("padding", region.padding);
                // self.parse_tt_attributes("display_align", region.display_align);
                // self.parse_tt_attributes("writing_mode", region.writing_mode);
                // self.parse_tt_attributes("show_background", region.show_background);
                // self.parse_tt_attributes("overflow", region.overflow);
                // self.parse_tt_attributes("style", region.style);

                //let style = self.parse_tt_attributes();
                //styles.push(parse_tt_attributes
            }
        }
        return styles.join(";");
        //return styles.iter().map(|s| s.as_str()).collect();
    }

    fn get_rows_for_p(&self, p: &P) -> String {
        let br_string = "<br/>".to_string();
        let x = Vec::new();
        let mut texts: Vec<String> = Vec::new();
        for (index, child) in p.children.as_ref().unwrap_or(&x).iter().enumerate() {
            //texts.push(self.tt.as_ref().unwrap().body.style.as_ref().unwrap());
            if let Choice::Span(span) = child {
                if (span.text.is_some()) {
                    //span.text
                    //let styles = self.get_style_string(style)
                    let styles = self.get_style_string(&span.style);
                    texts.push(format!(
                        "<span class='span-subtitle' style='{}'>{}</span>",
                        styles,
                        span.text.as_ref().unwrap()
                    ));
                }
            } else if let Choice::Br(br) = child {
                texts.push(br_string.clone());
            }
        }
        return texts.iter().map(|s| s.as_str()).collect();
    }

    fn parse_tt_attributes(&self, node_name: &str, node_value: &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        match node_name {
            "ttp:cellResolution" => {
                //let cell_resolution_array: Vec<&str> = node_value.split(" ").collect();
                //attributes.insert("columns".to_string(), cell_resolution_array[0].to_string());
                //attributes.insert("rows".to_string(), cell_resolution_array[1].to_string());
            }
            "tts:backgroundColor" | "tts:color" => {
                let rgba_color = self.hex_to_rgba(node_value);
                if (rgba_color.is_some()) {
                    attributes.insert(node_name[4..].to_string(), rgba_color.unwrap());
                }
            }
            "tts:fontFamily" | "tts:fontStyle" | "tts:fontWeight" | "tts:lineHeight"
            | "tts:textDecoration" | "tts:textAlign" | "tts:overflow" => {
                attributes.insert(node_name[4..].to_string(), node_value.to_string());
            }
            "tts:fontSize" => {
                let font_size = node_value.to_string();
                attributes.insert("fontSize".to_string(), font_size);
            }
            "tts:wrapOption" => {
                attributes.insert(
                    "whiteSpace".to_string(),
                    if node_value == "wrap" {
                        "normal".to_string()
                    } else {
                        "nowrap".to_string()
                    },
                );
            }
            _ => {}
        }

        attributes
    }

    fn show_subtile(&self, p: &P) {
        if (self.tt.is_some()) {
            if (existSubtitle(&p.id) == false) {
                startDebugTimer(&format!("show subtitle {}", &p.id));
                let default_styles = self.get_default_styles();
                let mut region_styles = "".to_string();
                if (p.region.is_some() == true) {
                    region_styles = self.get_region_styles(&p.region.as_ref().unwrap());
                }
                let text = self.get_rows_for_p(p);
                let text = format!(
                    "\
                    <div data-test-id='default-style-wrapper' style='{}'>\
                        <div class='regionContainer' data-test-id='region-style' style='{}' id='{}'>\
                            <div class='displayAlign regionPadding'>
                                <div class='paragraphContainer' data-test-id='paragraphContainer'>\
                                    <div class='multiRowAlign'>\
                                    {}\
                                    </div>\
                                </div>\
                            </div>\
                        </div>\
                    </div>\
                    ",
                    default_styles, region_styles, p.id, text
                );
                showSubtitle(&p.id, &text);
                endDebugTimer(&format!("show subtitle {}", &p.id));
            }
        }

        /*
          var defaultStyleWrapper, paragraphContainer, regionContainer;

          if (that.el.innerHTML.indexOf('id="' + subt.id + '"') !== -1) {
            return;
          }
          defaultStyleWrapper = document.createElement("div");

          _.each(
            that.defaultStyles,
            function (style) {
              defaultStyleWrapper = that.applyTTMLStyleIdToHTMLElement(
                defaultStyleWrapper,
                style
              );
            },
            that
          );

          paragraphContainer = document.createElement("div");
          paragraphContainer.className = "paragraphContainer";
          paragraphContainer.innerHTML =
            '<div class="multiRowAlign">' + subt.text + "</div>";
          paragraphContainer = that.applyTTMLStyleIdToHTMLElement(
            paragraphContainer,
            subt.style
          );

          regionContainer = document.createElement("div");
          regionContainer.id = subt.id;
          regionContainer.className = "regionContainer";
          regionContainer.innerHTML =
            '<div class="displayAlign regionPadding">' +
            paragraphContainer.outerHTML +
            "</div>";

          if (that.personalizationEnabled) {
            if (that.getBackgroundPersonalizedValue() === "active") {
              regionContainer.className += " user-force-background";
            } else {
              regionContainer.className += " user-no-background";
            }
          }

          regionContainer = that.applyTTMLRegionIdToHTMLElement(
            regionContainer,
            subt.region
          );

          defaultStyleWrapper.appendChild(regionContainer);
          that.el.appendChild(defaultStyleWrapper);
        };
           */
    }
    fn hex_to_rgba(&self, hex: &str) -> Option<String> {
        let hex = hex.trim_start_matches('#');

        match hex.len() {
            // Color HEX sense transparència
            6 => {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                ) {
                    Some(format!("rgb({}, {}, {})", r, g, b))
                } else {
                    None
                }
            }
            // Color HEX amb transparència
            8 => {
                if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                    u8::from_str_radix(&hex[0..2], 16),
                    u8::from_str_radix(&hex[2..4], 16),
                    u8::from_str_radix(&hex[4..6], 16),
                    u8::from_str_radix(&hex[6..8], 16),
                ) {
                    let alfa_in_float = (a as f32 / 255.0).to_string();
                    Some(format!("rgba({}, {}, {}, {})", r, g, b, alfa_in_float))
                } else {
                    None
                }
            }
            // En cas que hex no tingui una longitud vàlida
            _ => None,
        }
    }
    fn hide_subtile(&self, p: &P) {
        hideSubtitle(&p.id);
    }
}

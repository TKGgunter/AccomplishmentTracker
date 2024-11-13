#[allow(dead_code)]
use chrono::{DateTime, Utc};
use core::slice::Iter;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use pulldown_cmark::{html, Options, Parser};

pub mod convert;

// TODO
// - add comments about the u32 and why we have so many
// - clean up comments and TODOs 
// - clean up expects
// - clean up unwraps
// - change the same of repo.

const SERIAL_CODE: &[u8] = b"ATS ";

// NOTE
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
// use the above link to debug using the expand function.
macro_rules! generate_lp { // TODO name
    ($($x:tt),+) => {
        #[repr(u8)]
        #[derive(PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
        pub enum LeadershipPrinciples{
        $(
            $x
        ),*
        }

        impl LeadershipPrinciples {
            pub fn iterator() -> Iter<'static, LeadershipPrinciples>{
                use LeadershipPrinciples::*;
                static LEADERSHIPPRINCIPLES : [LeadershipPrinciples; 17] = [$( $x ),*];
                LEADERSHIPPRINCIPLES.iter()
            }

            pub fn to_str(&self) -> &str {
                use LeadershipPrinciples::*;
                match self {
                    Ownership => { "Ownership" },
                    DisagreeAndCommit => { "Disagree and Commit" },
                    LearnAndBeCurious => { "Learn and be Curious" },
                    CustomerObsession => { "Customer Obsession" },
                    InventAndSimplify => { "Invent and Simplify" },
                    AreRightALot => { "Are right alot" },
                    HireAndDevelopTheBest => { "Hire and Develop the Best" },
                    InsistOnTheHighestStandards => { "Insist on the Highest Standards" },
                    ThinkBig => { "Think Big" },
                    BiasForAction => { "Bias for Action" },
                    Frugality => { "Frugality" },
                    EarnTrust => { "Earn Trust" },
                    DiveDeep => { "Dive Deep" },
                    HaveBackboneAndCommit => { "Have backbone and Commit" },
                    DeliverResults => { "Deliver Results" },
                    SuccessAndScaleBringBroadResponsibility => {"Success and Scale bring Broad Responsibility" },
                    Empty => {"NA - Empty" } // TODO
                }
            }

            pub fn to_u32(&self) -> u32 {
                use LeadershipPrinciples::*;
                match self {
                    Ownership => { 0 },
                    DisagreeAndCommit => { 1 },
                    LearnAndBeCurious => { 2 },
                    CustomerObsession => { 3 },
                    InventAndSimplify => { 4 },
                    AreRightALot => { 5 },
                    HireAndDevelopTheBest => { 6 },
                    InsistOnTheHighestStandards => { 7 },
                    ThinkBig => { 8 },
                    BiasForAction => { 9 },
                    Frugality => { 10 },
                    EarnTrust => { 11 },
                    DiveDeep => { 12 },
                    HaveBackboneAndCommit => { 13 },
                    DeliverResults => { 14 },
                    SuccessAndScaleBringBroadResponsibility => { 15 },
                    Empty => { 16 } // TODO
                }
            }

            pub fn from_str(input: &str) -> LeadershipPrinciples {
                use LeadershipPrinciples::*;
                match input {
                    "Ownership" => { Ownership },
                    "Disagree and Commit" => { DisagreeAndCommit },
                    "Learn and be Curious" => { LearnAndBeCurious },
                    "Customer Obsession" => { CustomerObsession },
                    "Invent and Simplify" => { InventAndSimplify },
                    "Are right alot" => { AreRightALot },
                    "Hire and Develop the Best" => { HireAndDevelopTheBest },
                    "Insist on the Highest Standards" => { InsistOnTheHighestStandards },
                    "Think Big" => { ThinkBig },
                    "Bias for Action" => { BiasForAction },
                    "Frugality" => { Frugality },
                    "Earn Trust" => { EarnTrust },
                    "Dive Deep" => { DiveDeep },
                    "Have backbone and Commit" => { HaveBackboneAndCommit },
                    "Deliver Results" => { DeliverResults },
                    "Success and Scale bring Broad Responsibility" => { SuccessAndScaleBringBroadResponsibility },
                    _=> { Empty }
                }
            }
        }
    }
}
generate_lp!(
    Ownership,
    DisagreeAndCommit,
    LearnAndBeCurious,
    CustomerObsession,
    InventAndSimplify,
    AreRightALot,
    HireAndDevelopTheBest,
    InsistOnTheHighestStandards,
    ThinkBig,
    BiasForAction,
    Frugality,
    EarnTrust,
    DiveDeep,
    HaveBackboneAndCommit,
    DeliverResults,
    SuccessAndScaleBringBroadResponsibility,
    Empty

);

impl Default for LeadershipPrinciples {
    fn default() -> Self {
        LeadershipPrinciples::Empty
    }
}

#[repr(u16)]
#[derive(Default, Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum EventType {
    BarRaise,
    InvestInYourSelf,
    #[default]
    None
}


macro_rules! string_functions {
    ($x:ty) => {
        impl CustomStringTrait for $x {
            fn length(&self) -> u32 {
                self.length
            }

            fn set_length(&mut self, l: u32) {
                self.length = l;
            }

            fn buffer(&self) -> &[u8] {
                &self.buffer
            }

            fn buffer_mut(&mut self) -> &mut [u8] {
                &mut self.buffer
            }
        }
    };
}

pub trait CustomStringTrait: Default {
    fn length(&self) -> u32;
    fn set_length(&mut self, l: u32);
    fn buffer(&self) -> &[u8];
    fn buffer_mut(&mut self) -> &mut [u8];

    fn write(&mut self, input: &[u8]) {
        let l = usize::min(input.len(), self.buffer().len()) as u32;
        self.set_length(l);
        let length = self.length() as usize;
        self.buffer_mut()[..length].copy_from_slice(&input[..length]);
    }

    fn build_write(mut self, input: &[u8]) -> Self
    where
        Self: Sized,
    {
        self.write(input);
        self
    }

    fn from_str(input: &str) -> Self
    where
        Self: Sized,
    {
        Self::default().build_write(input.as_bytes())
    }

    fn as_str(&self) -> &str {
        let length = self.length() as usize;
        std::str::from_utf8(&self.buffer()[..length]).expect("Not a good string.")
    }
}

const TINYSTRING_LENGTH: usize = 104;
#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct TinyString {
    pub length: u32,
    pub buffer: [u8; TINYSTRING_LENGTH],
}

impl Default for TinyString {
    fn default() -> TinyString {
        TinyString {
            length: 0,
            buffer: [0u8; TINYSTRING_LENGTH],
        }
    }
}
string_functions!(TinyString);

const LARGESTRING_LEN: usize = 400;
#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct LargeString {
    pub length: u32,
    pub buffer: [u8; LARGESTRING_LEN],
}

impl Default for LargeString {
    fn default() -> LargeString {
        LargeString {
            length: 0,
            buffer: [0u8; LARGESTRING_LEN],
        }
    }
}
string_functions!(LargeString);

const EVENT_VERSION: u32 = 0;
#[repr(C)]
#[derive(Default, PartialEq, Debug)]
pub struct Event {
    pub date: DateTime<Utc>, // NOTE 12 bytes 
    pub leadership_principles: [LeadershipPrinciples; 2], // TODO Maybe an array or combine also how big
    pub event_type: EventType,
    pub summary: TinyString,
    pub details: LargeString,
}


#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct TomlEvent {
    pub date: toml::value::Datetime, // NOTE 12 bytes 
    pub leadership_principles: Vec<LeadershipPrinciples>, // TODO Maybe an array or combine also how big
    pub event_type: EventType,
    pub summary: String,
    pub details: String,
}

impl TomlEvent {
    pub fn to_event(&self) -> Event {
        use chrono::prelude::*;
        // TODO
        // throw an error if the date is wrong.
        let toml::value::Date{year, month, day} = self.date.date.unwrap();
        let mut lp = [LeadershipPrinciples::Empty; 2];
        for (i, it) in self.leadership_principles.iter().enumerate() {
            if i > 1 {
                break;
            }
            lp[i] = *it;
        }

        Event {
            date: Utc.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0).unwrap(),
            leadership_principles: lp,
            event_type: self.event_type,
            summary: TinyString::from_str(&markdown_to_html(&self.summary)),  // TODO throw an
                                                                              // error is the
                                                                              // markdown output is
                                                                              // too long.
            details: LargeString::from_str(&markdown_to_html(&self.details))
        }
    }
}

fn markdown_to_html(input: &str)->String { 
    //NOTE                       
    //This is an example from    
    //https://github.com/raphlinus/pulldown-cmark/blob/master/examples/string-to-string.rs
                                 
                                 
                                 
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();    
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, options);
                                 
                                 
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
                                 
    return html_output;          
}  

const DATA_VERSION: u32 = 0;
#[derive(Default)]
pub struct AccomplishmentData {
    pub events: Vec<Event>,
}

impl AccomplishmentData {
    pub fn sort(&mut self) {
        self.events
            .sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap()); // TODO there should be a
                                                                   // default
    }
}

pub struct _AccomplishmentData<'a> {
    pub events: &'a [Event],
}
#[derive(Deserialize, Serialize)]
pub struct TomlAccomplishmentData {
    pub events: Vec<TomlEvent>,
}

// TODO
impl TomlAccomplishmentData {
    pub fn to_accomplishmentdata(self) -> AccomplishmentData {
        AccomplishmentData::default()
    }
}


/// 3 byte identifier code
/// u32 Data struct version
/// usize Data struct size
/// u32 Event struct version
/// usize Event struct size
/// usize number of elements in buffer
/// continues block of memory containing Event structs
pub fn serialize_to_file(data: &AccomplishmentData, filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;

    file.write_all(SERIAL_CODE)?; // NOTE Struct tag for data type
    file.write_all(&(std::mem::size_of::<usize>() as u32).to_le_bytes())?;
    file.write_all(&DATA_VERSION.to_le_bytes())?;
    file.write_all(&(std::mem::size_of::<AccomplishmentData>() as u32).to_le_bytes())?;
    file.write_all(&EVENT_VERSION.to_le_bytes())?;
    file.write_all(&(std::mem::size_of::<Event>() as u32).to_le_bytes())?;
    file.write_all(&(data.events.len() as u32).to_le_bytes())?;

    unsafe {
        let ptr = data.events.as_ptr() as *const u8;
        let slice =
            std::slice::from_raw_parts(ptr, data.events.len() * std::mem::size_of::<Event>());
        file.write_all(slice)?;
    }

    return Ok(());
}

// TODO input should be a trait "AsRef<[u8]>"
pub fn deserialize(io: &[u8]) -> Result<_AccomplishmentData, String> {
    let mut cursor = 0;
    // check length
    for i in 0..4 {
        if io[i] != SERIAL_CODE[i] {
            return Err(format!(
                "Bad SerialCode {}: {} != {}",
                i, io[i] as char, SERIAL_CODE[i] as char
            ));
        }
        cursor += 1;
    }
    let author_ptr_size = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );

    cursor += std::mem::size_of::<u32>();

    let data_version = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );

    cursor += std::mem::size_of::<u32>();

    let data_size = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );
    cursor += std::mem::size_of::<u32>();

    if data_version != DATA_VERSION {
        return Err("TODO implement version differences.".to_string());
    } else {
        // TODO add code to handle when struct is expanded.
        if author_ptr_size == std::mem::size_of::<usize>() as u32
            && data_size as usize != std::mem::size_of::<AccomplishmentData>()
        {
            return Err(format!(
                "Size of Data struct not equal. {}: {} != {}",
                cursor,
                data_size,
                std::mem::size_of::<AccomplishmentData>()
            ));
        }
    }

    let event_version = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );
    cursor += std::mem::size_of::<u32>();

    let event_size = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );
    cursor += std::mem::size_of::<u32>();

    let events_len = u32::from_le_bytes(
        io[cursor..cursor + std::mem::size_of::<u32>()]
            .try_into()
            .unwrap(),
    );
    cursor += std::mem::size_of::<u32>();

    let data; // TODO move this to the top and allow the function to fill in anything that's
              // missing
    if event_version != EVENT_VERSION {
        return Err("TODO when deserial event version is different.".to_string());
    } else {
        if event_size as usize != std::mem::size_of::<Event>() {
            return Err(format!(
                "Size of Event struct not equal. {} != {}",
                event_size,
                std::mem::size_of::<Event>()
            ));
        }

        let (_, r) = io.split_at(cursor);
        if r.len() as u32 != events_len * event_size {
            return Err(format!(
                "Size of event buffer is not equal to expected. {}: {} != {}",
                cursor,
                r.len(),
                events_len * event_size
            ));
        }

        data = {
            let ptr = r.as_ptr() as *const Event;
            unsafe {
                if ptr.is_null() || !ptr.is_aligned(){
                    let c = core::mem::align_of_val(&Event::default());
                    println!("{} {:?}", c, ptr);
                    _AccomplishmentData { events: &[]}
                } else {
                    let _events_len = events_len as usize;
                    if _events_len >= isize::MAX as usize {
                        _AccomplishmentData { events: &[] }
                    } else {
                        let slice = std::slice::from_raw_parts(ptr, _events_len);
                        _AccomplishmentData { events: slice }
                    }
                }
            }
        };
    }

    if data.events.len() != events_len as usize {
        return Err(format!(
            "Number of events are not equal. {} != {}",
            data.events.len(),
            events_len
        ));
    }

    Ok(data)
}

pub fn run(contents: String, output_file_path: PathBuf) {

    let toml_deserialization: TomlAccomplishmentData = toml::from_str(&contents).unwrap();

    let mut accomplishmentdata_format= AccomplishmentData::default();
    for it in toml_deserialization.events.iter() {
        accomplishmentdata_format.events.push(it.to_event());
    }
    accomplishmentdata_format.sort();

    // TODO should be removed from this function.
    serialize_to_file(&accomplishmentdata_format, output_file_path.to_str().unwrap()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use std::io::Read;
    use std::str::FromStr;

    #[test]
    fn test_binary_serial_deserial() {
        
        let mut data = AccomplishmentData::default();
        data.events.push(Event {
            date: Utc.with_ymd_and_hms(2021, 3, 1, 0, 0, 0).unwrap(),
            leadership_principles: [LeadershipPrinciples::Ownership; 2],
            summary: TinyString::default().build_write(b"abc"),
            details: LargeString::default().build_write(b"124"),
            event_type: EventType::BarRaise,
        });
        data.events.push(Event {
            date: Utc.with_ymd_and_hms(2021, 4, 1, 0, 0, 0).unwrap(),
            leadership_principles: [LeadershipPrinciples::Ownership; 2],
            summary: TinyString::default().build_write(b"abc"),
            details: LargeString::default().build_write(b"124"),
            event_type: EventType::None,
        });
        data.events.push(Event {
            date: Utc.with_ymd_and_hms(2020, 3, 1, 0, 0, 0).unwrap(),
            leadership_principles: [LeadershipPrinciples::DisagreeAndCommit; 2],
            summary: TinyString::default().build_write(b"I hate dev!"),
            details: LargeString::default().build_write(b"I will not explain."),
            event_type: EventType::None,
        }); 
        data.events.push(Event {
            date: Utc.with_ymd_and_hms(2020, 2, 1, 0, 0, 0).unwrap(), 
            leadership_principles: [LeadershipPrinciples::DisagreeAndCommit; 2],
            summary: TinyString::default().build_write(b"I hate dev!"),
            details: LargeString::default().build_write(b"## Description\nI will not explain."),
            event_type: EventType::InvestInYourSelf,
        });

        data.sort();
        serialize_to_file(&data, "temp.serialize").unwrap();
        let mut file = File::open("temp.serialize").unwrap();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let result = deserialize(&buffer).unwrap();

        for i in 0..result.events.len() {
            assert!(
                data.events[i] == result.events[i],
                "{}; {:?} != {:?}",
                i,
                data.events[i],
                result.events[i]
            )
        }
    }
    
    #[test]
    fn test_toml_deserialize() {

        let toml_str = r#"
        [[events]]
        date = 2021-03-01
        leadership_principles = ["Ownership", "Ownership"]
        summary = "This is a test."
        details = """This is a test and here are some details."""
        event_type = "None"
        "#;

        let decode: TomlAccomplishmentData = toml::from_str(toml_str).unwrap();
        for it in decode.events.iter() {
            println!("{:?}", it);

            let e = TomlEvent {
                date: toml::value::Datetime::from_str("2021-03-01").unwrap(),
                leadership_principles: vec![LeadershipPrinciples::Ownership; 2],
                summary: "This is a test.".to_string(),
                details: "This is a test and here are some details.".to_string(),
                event_type: EventType::None,
            };
            assert!(*it == e)
        }
    }
    
    #[test]
    fn test_tomlevent_to_event() {
        let te = TomlEvent {
            date: toml::value::Datetime::from_str("2021-03-01").unwrap(),
            leadership_principles: vec![LeadershipPrinciples::Ownership; 2],
            summary: "This is a test.".to_string(),
            details: "This is a test and here are some details.".to_string(),
            event_type: EventType::None,
        };
        let e = Event {
            date: Utc.with_ymd_and_hms(2021, 3, 1, 0, 0, 0).unwrap(),
            leadership_principles: [LeadershipPrinciples::Ownership; 2],
            summary: TinyString::from_str(&markdown_to_html("This is a test.")),
            details: LargeString::from_str(&markdown_to_html("This is a test and here are some details.")),
            event_type: EventType::None,
        };

        let te_to_e = te.to_event();

        assert!(te_to_e == e, "{:?} {:?}", te_to_e.summary, e.summary)
    }
    
    #[test]
    fn test_tomlevent_to_event_markdown() {
        let te = TomlEvent {
            date: toml::value::Datetime::from_str("2021-03-01").unwrap(),
            leadership_principles: vec![LeadershipPrinciples::Ownership; 2],
            summary: "This is a test.".to_string(),
            details: "## Description\nThis is a test and here are some details.".to_string(),
            event_type: EventType::None,
        };
        let e = Event {
            date: Utc.with_ymd_and_hms(2021, 3, 1, 0, 0, 0).unwrap(),
            leadership_principles: [LeadershipPrinciples::Ownership; 2],
            summary: TinyString::from_str(&markdown_to_html("This is a test.")),
            details: LargeString::from_str(&markdown_to_html("## Description\nThis is a test and here are some details.")),
            event_type: EventType::None,
        };

        let te_to_e = te.to_event();

        assert!(te_to_e == e, "{:?} {:?}", te_to_e.summary, e.summary)
    }

    #[test]
    fn test_toml_to_serial() {
        let toml_str = r#"
        [[events]]
        date = 2021-03-01
        leadership_principles = ["Ownership"]
        summary = "This is a test."
        details = "\n## Description\nThis is a test and here are some details."
        event_type = "None"
        "#;

        run(toml_str.into(), "temp_1.serialize".into());
    }
}

use accomplishment_tracker_shared::{
    _AccomplishmentData, deserialize, CustomStringTrait, Event, EventType, LeadershipPrinciples,
};
use bytes;
use chrono::{Datelike, Month};
use js_sys;
use reqwest_wasm;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

pub mod levenshtein_distance;
#[macro_use]
mod log;
use log::*;

mod helper;
use helper::*;

// NOTE
// Example can be found below
// Document/Rust/Sandbox/dom

// DONE
// - [X] test get ability to pull in that display the data.
// - [X] get the code from previous accomplishment tracker
// - [X] render every element in the Event struct
// - [X] test with multiple events
// - [X] repeat style of old code
//   - [x] Month header
//   - [x] year drop down
//   - [X] Leadership principle summary
//   - [X] Render Legend
//   - [X] Render event data
//   - [X] Get LP summary from data.
//   - [X] Rendering should be done on a monthly level
//         div id should include month indicator
//   - [X] Render invisible all other months and years
// - Add event call backs
//   - [X] Month based call back
//   - [X] Year based call back
// - [X] log macro
// - Add graphs
//   - [X] Summary div
//   - [X] function to get data to graph tool.
// - [X] bug year select does not up date properly.
// - [X] search render bug
// - [X] handle markdown
// - Search for things
//   - [X] make font larger
//   - [X] make search faster
//   - [X] is this the right search function?
// - [X] event_type is not being set after toml conversion
//

// TODO
// - summary is broken - on large site.
// - year drop down menu ordering
// - search is not good.
// - show more feature
// - [~] Move functionality into different files.
// - Clean up TODOs
// - Clean up warnings
// - Add descriptive notes
// - Clean up naming for ids
// - Clean up naming used in css
// - Use a Css document instead of setting the style manually.
// - Clean up naming used for call backs
// - set the url for different buttons and pages and search
// - render the correct page when the url is a specific way
// - do we want to remove reqwest? it seems to take longer to compile.
//
//
// How to run.
// wasm-pack build --target web
// python3 -m http.server

// TODO 
// can we run this like a server, have the page reload once it is built

const C_BAR_RAISING: &'static str = "#1d6860";
const C_INVEST_IN_YOURSELF: &'static str = "#555577";
static mut DATA: Option<bytes::Bytes> = None;
const N_LEADERSHIP_PRINCIPLES: usize = 17;

// TODO add consts for id, and class names.
fn get_data() -> Option<&'static bytes::Bytes> {
    unsafe {
        console_log!("Getting data");
        DATA.as_ref()
    }
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // NOTE I want to set the screen color as soon as possible.
    let _ = body.set_attribute(
        "style",
        "background-color:#3b3c3d; color:#ccc; font-family:Sans-Serif",
    );

    // NOTE reqwest_wasm seem to take a long time to compile test w/out.
    // Instead we can use web_sys instead but it will take some work to get the bytes
    // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
    //
    // TODO address needs to be generalized.
    console_log!("Document url: {}", document.url().unwrap());
    let document_url = document.url().expect("Document does not have a url.");
    let data_bytes = reqwest_wasm::get(document_url + "/temp.serialize")
        .await
        .expect("Failed to retrieve serialized data.")
        .bytes()
        .await
        .expect("Failed to obtain bytes.");

    console_log!("Downloaded serialized data: N-Bytes {} ", data_bytes.len());
    {
        let _data;
        match deserialize(&data_bytes) {
            Ok(a) => {
                _data = a;
            }
            Err(a) => {
                console_log!("{}", &a);
                return Ok(());
            }
        }

        console_log!("Attempting to render tracker.");
        match render_accomplishment_tracker(&document, &body, &_data) {
            Err(_) => {
                console_log!("Tracker render failure.");
                return Ok(());
            }
            _ => {}
        }
    }

    unsafe {
        DATA = Some(data_bytes);
        console_log!("Storing data.");
    }

    Ok(())
}

pub fn render_accomplishment_tracker(
    document: &web_sys::Document,
    body: &web_sys::HtmlElement,
    at_data: &_AccomplishmentData,
) -> Result<(), JsValue> {
    let years = collect_unique_years(at_data);
    let active_year = *years.iter().max().unwrap();
    let select_year_div = document.create_element("div")?;

    console_log!("{} {:?}", active_year, years);
    let _ = render_year_dropdown(document, &select_year_div, &years, active_year);
    let _ = body.append_child(&select_year_div);
    for year in years.iter() {

        let div_year = document.create_element("div")?;
        div_year.set_class_name("div_year");
        div_year.set_id(&format!("div_{}", year));
        if active_year != *year {
            let _ = div_year.set_attribute("style", "display: none");
        }
        let _ = body.append_child(&div_year);

        let uq_months = collect_unique_months_by_year(at_data, *year);
        let active_month = {
            let mut rt = 0;
            for (m, mt) in uq_months.iter().enumerate() {
                if *mt {
                    rt = m;
                }
            }
            rt
        };

        render_selection_by_date_menu(document, &div_year, &uq_months, active_month, *year)?;
        console_log!("Rendering date menu for years {:?}", years);

        for (month, status) in uq_months.iter().enumerate() {
            if !status {
                console_log!("Skipping month");
                continue;
            }
            let month_str = Month::try_from(month as u8 + 1).unwrap().name();
            let div_month = document.create_element("div")?;
            div_month.set_id(&format!("month_{}_{}", month_str, year));
            // TODO set a class so that we can turn off months
            div_month.set_class_name("month_class"); // TODO class name is a misnomer -- make this
                                                     // a global then change.
            if active_month != month {
                let _ = div_month.set_attribute("style", "display: none");
            }

            render_legend(document, &div_month)?;

            let data = collect_leadership_statistic(&at_data, *year, month);
            let div = document.create_element("div")?;
            div.set_class_name("tabcontent");
            div.set_id(&format!("monthly_log_{}_{}", month_str, year)); // TODO this name should be associated with an actual month.
            render_leadership_summary(document, &div, &data)?;
            render_events_table(document, &div, at_data, *year, month)?;

            let _ = div_month.append_child(&div);
            let _ = div_year.append_child(&div_month);
        }

        {
            let div_summary = document.create_element("div")?;
            div_summary.set_id(&format!("summary_{}", year));
            div_summary.set_class_name("month_class");
            let _ = div_summary.set_attribute("style", "display: none");
            {
                console_log!("Creating canvas ctx.");
                let canvas_principles_by_year = document.create_element("canvas")?;
                canvas_principles_by_year.set_id(&format!("canvas_principles_{}", year));
                let _ = div_summary.append_child(&canvas_principles_by_year);

                let canvas_principles_by_year = document.create_element("canvas")?;
                canvas_principles_by_year.set_id(&format!("canvas_principle_month_{}", year));
                let _ = div_summary.append_child(&canvas_principles_by_year);
            }
            let _ = div_year.append_child(&div_summary);
        }

        {
            // TODO we are creating multiple searches. There should only be one.
            let div_search = document.create_element("div")?;
            div_search.set_id(&format!("search_{}", year));
            div_search.set_class_name("month_class");
            let _ = div_search.set_attribute("style", "display: none");

            {
                let _year = *year;
                let a = Closure::<dyn Fn(JsValue)>::new(move |input| {
                    update_search_results(input, _year)
                }); //TODO
                let textbox = document.create_element("input")?;
                let _ = textbox.set_attribute("placeholder", "Type word or phrase.");
                let _ = textbox.set_attribute("style", "font-size: 150%; align-items: center");
                let _ = textbox.set_attribute("type", "search");
                let _ = textbox.set_attribute("size", "32");
                textbox.set_id(&format!("search_textbox_{}", year));

                let _ =
                    textbox.add_event_listener_with_callback("input", a.as_ref().unchecked_ref());
                let _ = div_search.append_child(&textbox);

                let result_div = document.create_element("div")?;
                result_div.set_id(&format!("search_results_{}", year));
                result_div.set_class_name("search_class");
                let _ = result_div.set_attribute("style", "display: block");

                let _ = div_search.append_child(&result_div);
                a.forget();
            }
            let _ = div_year.append_child(&div_search);
        }
    }

    Ok(())
}

pub fn render_events_table_fn(
    document: &web_sys::Document,
    div: &web_sys::Element,
    at_data: &_AccomplishmentData,
    fn_pointer: &dyn Fn(&Event) -> bool,
) -> Result<(), JsValue> {
    let table = document.create_element("table")?;
    {
        // NOTE: Header
        let tr = document.create_element("tr")?;
        {
            let th = document.create_element("th")?;
            th.set_class_name("smallColumn");
            th.set_text_content(Some("Date"));
            let _ = tr.append_child(&th);
        }
        {
            let th = document.create_element("th")?;
            let _ = th.set_attribute("width", "22%");
            th.set_text_content(Some("Summary"));
            let _ = tr.append_child(&th);
        }
        {
            let th = document.create_element("th")?;
            th.set_text_content(Some("Details"));
            let _ = tr.append_child(&th);
        }
        {
            let th = document.create_element("th")?;
            let _ = th.set_attribute("width", "20%");
            th.set_text_content(Some("Amazon Leadership Principles"));
            let _ = tr.append_child(&th);
        }
        let _ = table.append_child(&tr);
    }

    {
        let tbody = document.create_element("tbody")?;
        for it in at_data.events.iter() {
            if fn_pointer(it) == false {
                continue;
            }
            let tr = document.create_element("tr")?;
            match it.event_type {
                EventType::BarRaise => {
                    let _ = tr.set_attribute("bgcolor", C_BAR_RAISING);
                }
                EventType::InvestInYourSelf => {
                    let _ = tr.set_attribute("bgcolor", C_INVEST_IN_YOURSELF);
                }
                EventType::None => {}
            }

            {
                let td = document.create_element("td")?;
                td.set_class_name("valueCells");
                let date_string = it.date.format("%Y-%m-%d").to_string();
                td.set_text_content(Some(&date_string));
                tr.append_child(&td);
            }

            {
                let td = document.create_element("td")?;
                td.set_inner_html(&it.summary.as_str());
                let _ = tr.append_child(&td);
            }

            {
                // TODO call backs show more show less
                let td = document.create_element("td")?;
                td.set_inner_html(&it.details.as_str());
                let _ = tr.append_child(&td);
            }

            tbody.append_child(&tr);

            {
                // TODO call backs show more/show less
                let td = document.create_element("td")?;
                td.set_class_name("valueCells");
                for jt in it.leadership_principles.iter() {
                    // TODO remove duplets
                    if *jt == LeadershipPrinciples::Empty {
                        continue;
                    }
                    let string = format!("{}, ", jt.to_str());
                    td.append_with_str_1(&string);
                }
                tr.append_child(&td);
            }

            tbody.append_child(&tr);
        }
        table.append_child(&tbody);
    }
    div.append_child(&table);

    Ok(())
}

pub fn render_events_table(
    document: &web_sys::Document,
    div: &web_sys::Element,
    at_data: &_AccomplishmentData,
    year: usize,
    month: usize,
) -> Result<(), JsValue> {
    let filter = |it: &Event| it.date.year() as usize == year && it.date.month0() as usize == month;
    render_events_table_fn(document, div, at_data, &filter)
}

pub fn render_selection_by_date_menu(
    document: &web_sys::Document,
    body: &web_sys::Element,
    data: &[bool],
    active_month: usize,
    year: usize,
) -> Result<(), JsValue> {
    let menu_div = document.create_element("div")?; // TODO move outside and replace body
    menu_div.set_class_name("tab");
    menu_div.set_id(&format!("month_year_menu_{}", year));

    for (i, it) in data.iter().enumerate() {
        if *it {
            let month = Month::try_from(i as u8 + 1).unwrap().name();

            let button = document.create_element("button")?;
            button.set_id(&format!("button_{}_{}", month, year));
            if i == active_month {
                button.set_class_name("tablinks active"); // TODO we need better names
            } else {
                button.set_class_name("tablinks"); // TODO we need better names
            }
            let a = Closure::<dyn Fn()>::new(move || set_month_tab(&month, year)); // TODO
            let _ = button.add_event_listener_with_callback("click", a.as_ref().unchecked_ref());
            button.set_text_content(Some(&month));
            menu_div.append_child(&button)?;

            a.forget(); // TODO explain why this is need. https://github.com/rustwasm/wasm-bindgen/issues/843
            console_log!("completed rendering month: {}", &month);
        }
    }

    {
        let button = document.create_element("button")?;
        button.set_id(&format!("button_summary_{}", year));
        button.set_text_content(Some("Summary"));

        let a = Closure::<dyn Fn()>::new(move || set_month_tab("summary", year)); // TODO
        let _ = button.add_event_listener_with_callback("click", a.as_ref().unchecked_ref());

        menu_div.append_child(&button)?;
        a.forget(); // TODO explain why this is need. https://github.com/rustwasm/wasm-bindgen/issues/843
    }

    {
        let button = document.create_element("button")?;
        button.set_id(&format!("button_search_{}", year));
        button.set_text_content(Some("Search"));

        let a = Closure::<dyn Fn()>::new(move || set_month_tab("search", year)); // TODO
        let _ = button.add_event_listener_with_callback("click", a.as_ref().unchecked_ref());

        menu_div.append_child(&button)?;
        a.forget(); // TODO explain why this is need. https://github.com/rustwasm/wasm-bindgen/issues/843
    }

    // TODO There should only be one date menu
    //render_year_dropdown(document, &menu_div, &years, year)?;
    body.append_child(&menu_div)?;
    console_log!("completed month menu");
    Ok(())
}

pub fn render_year_dropdown(
    document: &web_sys::Document,
    menu_div: &web_sys::Element,
    years: &HashSet<usize>,
    input_year: usize,
) -> Result<(), JsValue> {
    console_log!("render year dropdown.");

    let max_year = years.iter().max().unwrap();
    let a = Closure::<dyn Fn()>::new(move || set_year_option(input_year));
    let year_selection = {
        match document.get_element_by_id("year_selection") {
            Some(ys) => ys,
            None => {
                let year_selection = document.create_element("select")?;
                let _ = year_selection.set_attribute(
                    "style",
                    "font-size: 2em; float: right; padding-right: 50px; padding-top: 8px",
                );
                year_selection.set_id("year_selection"); // TODO pass the id?
                let _ = year_selection
                    .add_event_listener_with_callback("change", a.as_ref().unchecked_ref());
                for year in years.iter() {
                    if *year == 0 {
                        break;
                    }
                    let year_option = document.create_element("option")?;
                    let year_string = format!("{}", year);
                    year_option.set_text_content(Some(&year_string));

                    if *year == *max_year {
                        let _ = year_option.set_attribute("selected", "selected");
                    }
                    let _ = year_selection.append_child(&year_option);
                }
                console_log!("Create drop down. {}", input_year);
                year_selection
            }
        }
    };

    console_log!("{:?}", year_selection);
    let _ = menu_div.append_child(&year_selection);
    a.forget(); // TODO explain why this is need. https://github.com/rustwasm/wasm-bindgen/issues/843

    console_log!("completed dropdown. {}", input_year);
    Ok(())
}

pub fn render_leadership_summary(
    document: &web_sys::Document,
    div: &web_sys::Element,
    data: &[usize; N_LEADERSHIP_PRINCIPLES],
) -> Result<(), JsValue> {
    console_log!("begin summary.");
    // TODO move div outside and input the div in place of body.
    {
        let table = document.create_element("table")?;
        let _ = table.set_attribute("style", "font-size:12px"); // TODO
        {
            let tr = document.create_element("tr")?;
            {
                let th = document.create_element("th")?;
                let _ = th.set_attribute("style", "font-size:150%"); // TODO
                let _ = th.set_attribute("colspan", "16"); // TODO
                let _ = th.set_text_content(Some("Monthly Summary"));

                let _ = tr.append_child(&th);
            }
            let _ = table.append_child(&tr);

            // TODO: describe what happening here.
            let tr = document.create_element("tr")?;
            tr.set_class_name("summary");
            for it in LeadershipPrinciples::iterator() {
                if *it == LeadershipPrinciples::Empty {
                    continue;
                }

                let th = document.create_element("th")?;
                th.set_text_content(Some(it.to_str()));

                let _ = tr.append_child(&th);
            }
            let _ = table.append_child(&tr);

            // TODO: describe what happening here.
            let tr = document.create_element("tr")?;
            tr.set_class_name("summary");
            for it in LeadershipPrinciples::iterator() {
                if *it == LeadershipPrinciples::Empty {
                    continue;
                }

                let th = document.create_element("th")?;
                let value = data[*it as usize];
                match value {
                    0 => {}
                    1..=3 => {
                        // TODO
                        let _ = th.set_attribute("style", "background-color: #7d5a0c");
                    }
                    _ => {
                        let _ = th.set_attribute("style", "background-color: #892e44");
                    }
                }
                let string = format!("{}", value);
                th.set_text_content(Some(&string));

                let _ = tr.append_child(&th);
            }

            let _ = table.append_child(&tr);
        }
        let _ = div.append_child(&table);
    }

    console_log!("end summary.");
    Ok(())
}

pub fn render_legend(document: &web_sys::Document, body: &web_sys::Element) -> Result<(), JsValue> {
    let p = document.create_element("p")?;
    {
        p.set_class_name("legend");
        p.set_text_content(Some("Bar raising moment - "));
        let font = document.create_element("font")?;
        let _ = font.set_attribute("color", C_BAR_RAISING);
        font.set_text_content(Some("  █"));
        let _ = p.append_child(&font);

        let br = document.create_element("br")?;
        let _ = p.append_child(&br);

        p.append_with_str_1("Invest in yourself - ")?;
        let font = document.create_element("font")?;
        let _ = font.set_attribute("color", C_INVEST_IN_YOURSELF);
        font.set_text_content(Some("  █"));
        let _ = p.append_child(&font);
    }
    body.append_child(&p)?;

    let p = document.create_element("p")?;
    {
        p.set_class_name("legend");
        p.set_text_content(Some("If you have feedback please follow this "));
        let a = document.create_element("a")?;
        let _ = a.set_attribute("href", "https://quip-amazon.com/yil8AxIlg78u/Accomplishment-and-Invest-in-Yourself-Tracker-Thoth#temp:C:LfXc7ddf386401743d0a4944584c"); // TODO this link url should be global
        a.set_text_content(Some("link"));
        p.append_child(&a)?;
        p.append_with_str_1(".")?;
    }
    body.append_child(&p)?;
    Ok(())
}

// TODO better naming
fn set_month_tab(month: &str, year: usize) {
    // TODO add description
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let div_year = document
        .get_element_by_id(&format!("div_{}", year))
        .unwrap();

    let collection = div_year.get_elements_by_class_name("month_class");
    for i in 0..collection.length() {
        let element = collection.item(i).unwrap();
        let _ = element.set_attribute("style", "display:  none");
    }

    let collection = div_year.get_elements_by_class_name("tablinks");
    for i in 0..collection.length() {
        let element = collection.item(i).unwrap();
        // let string = element.class_name(); FIXME
        element.set_class_name("tablinks");
    }

    // TODO we shouldn't need this branch it the elements were named appropriately
    // And maybe this should be a match.
    if month == "summary" {
        let opt_element = document.get_element_by_id(&format!("summary_{}", year)); //TODO
        match opt_element {
            Some(element) => {
                let _ = element.set_attribute("style", "display: block");
            }
            None => {
                console_log!("Error Summary information has not been implemented.");
            }
        }
    } else if month == "search" {
        console_log!("pushing the search button.");
        let opt_element = document.get_element_by_id(&format!("search_{}", year)); //TODO
        match opt_element {
            Some(element) => {
                let _ = element.set_attribute("style", "display: block");
            }
            None => {
                console_log!("Error Summary information has not been implemented.");
            }
        }
    } else {
        let opt_element = document.get_element_by_id(&format!("month_{}_{}", month, year)); //TODO
        match opt_element {
            Some(element) => {
                let _ = element.set_attribute("style", "display: block");
            }
            None => {
                console_log!("Some error");
            }
        }
        let tabcontent = document
            .get_element_by_id(&format!("monthly_log_{}_{}", month, year))
            .unwrap();
        let _ = tabcontent.set_attribute("style", "display: block");
    }

    let tabcontent = document
        .get_element_by_id(&format!("button_{}_{}", month, year))
        .unwrap();
    tabcontent.set_class_name("tablinks active");
}

fn set_year_option(year_selection_id: usize) {
    console_log!("onchange activated.");
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let collection = document.get_elements_by_class_name("div_year");
    for i in 0..collection.length() {
        let _ = collection
            .item(i)
            .unwrap()
            .set_attribute("style", "display: none");
    }

    // TODO clear selected attribute then set the new attribute as selected.
    //for collection_years in document.get_elements_by_class_name("");

    let element = document.get_element_by_id("year_selection").unwrap();
    let selection_element = element.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
    let selected_index = selection_element.selected_index();
    let value = selection_element.value();
    console_log!("year selection id {}", year_selection_id);
    console_log!("{}", selected_index);
    console_log!("{}", &value);

    let element = document
        .get_element_by_id(&format!("div_{}", value))
        .unwrap();
    let _ = element.set_attribute("style", "display: block");
    console_log!("onchange complete");
}

#[wasm_bindgen]
pub fn get_leadership_data_by_year(_year: &JsValue) -> js_sys::Array {
    let year = match _year.as_f64() {
        Some(f) => f as i32,
        None => {
            console_log!("Unexpected input for year. {:?}", _year);
            0
        }
    };

    let array = js_sys::Array::new_with_length(N_LEADERSHIP_PRINCIPLES as u32);
    array.fill(&JsValue::from(0.0), 0, 16);
    let data_bytes = match get_data() {
        Some(data) => data,
        None => {
            console_log!("Data is not instantiated.");
            return array;
        }
    };

    let _data;
    match deserialize(data_bytes) {
        Ok(a) => {
            _data = a;
        }
        Err(a) => {
            console_log!("{}", &a);
            return array;
        }
    }

    for it in _data.events.iter() {
        if it.date.year() != year {
            continue;
        }
        for lp in it.leadership_principles.iter() {
            let index = lp.to_u32();
            let value = match array.get(index).as_f64() {
                Some(a) => a,
                None => {
                    console_log!("Could not get data for leadership. Index: {}", index);
                    0f64
                }
            };
            let v = wasm_bindgen::JsValue::from(value + 1.0f64);
            array.set(index, v);
        }
    }
    array
}

#[wasm_bindgen]
pub fn get_leadership_labels() -> js_sys::Array {
    let array = js_sys::Array::new_with_length(N_LEADERSHIP_PRINCIPLES as u32);
    array.fill(&JsValue::from(""), 0, 16);

    // TODO this should be static. There is no reason why we want to
    // recreate this time and time again.
    for (i, it) in LeadershipPrinciples::iterator().enumerate() {
        let v = JsValue::from(it.to_str());
        array.set(i as u32, v);
    }
    array
}

#[wasm_bindgen]
pub fn get_leadership_data_by_lp_year(_lp: &JsValue, _year: &JsValue) -> js_sys::Array {
    let year = match _year.as_f64() {
        Some(f) => f as i32,
        None => {
            console_log!("Unexpected input for year. {:?}", _year);
            0
        }
    };

    let lp = match _lp.as_string() {
        Some(f) => f,
        None => {
            console_log!("Unexpected input for lp. {:?}", _lp);
            String::new()
        }
    };

    let array = js_sys::Array::new_with_length(12);
    array.fill(&JsValue::from(0.0), 0, 11);
    let data_bytes = match get_data() {
        Some(data) => data,
        None => {
            console_log!("Data is not instantiated.");
            return array;
        }
    };

    let _data;
    match deserialize(data_bytes) {
        Ok(a) => {
            _data = a;
        }
        Err(a) => {
            console_log!("{}", &a);
            return array;
        }
    }

    for it in _data.events.iter() {
        if it.date.year() != year {
            continue;
        }
        let index = it.date.month() as u32;
        for it_lp in it.leadership_principles.iter() {
            // TODO user year to filter data and leadership principle
            if it_lp.to_str() != lp {
                continue;
            }
            let value = match array.get(index).as_f64() {
                Some(a) => a,
                None => {
                    console_log!("Could not get data for leadership. Index: {}", index);
                    0f64
                }
            };
            let v = wasm_bindgen::JsValue::from(value + 1.0f64);
            array.set(index, v);
        }
    }
    array
}
#[wasm_bindgen]
pub fn get_years() -> js_sys::Array {
    let array = js_sys::Array::new();
    let data_bytes = match get_data() {
        Some(data) => data,
        None => {
            console_log!("Data is not instantiated.");
            return array;
        }
    };

    let _data;
    match deserialize(data_bytes) {
        Ok(a) => {
            _data = a;
        }
        Err(a) => {
            console_log!("{}", &a);
            return array;
        }
    }

    let set = collect_unique_years(&_data);
    for it in set.iter() {
        array.push(&JsValue::from(*it));
    }
    array
}

/// TODO how does this work?
/// Function scores how well the query matches the text in the event.
/// It takes the minimum levenshtein distance for each word in the query compared to each word in
/// the summary and details entry of the event.
/// Scores from details and summary are weighed equally.
/// It does not attempt to match complete phrases.
fn score_event(e: &Event, _query: &str) -> f32 {
    if _query.len() == 0 {
        return 0f32;
    }

    let mut query = _query.to_string();
    query.make_ascii_lowercase();

    let mut _text_set = HashSet::new();

    // TODO this should be done in a separate fast function.
    let mut _summary = e.summary.as_str().to_string();
    _summary.make_ascii_lowercase();
    _summary = _summary.replace("</", " ")
        .replace("<h3>", " ")
        .replace("<h2>", " ")
        .replace("<p>", " ")
        .replace("\n", " ")
        .replace(" and ", " ")
        .replace(" a ", " ")
        .replace(" be ", " ")
        .replace(" let ", " ")
        .replace("\t", " ");

    for it in _summary.split(" ") {
        _text_set.insert(it);
    }

    // TODO this should be done in a separate fast function.
    let mut _details = e.details.as_str().to_string();
    _details.make_ascii_lowercase();
    _details= _details.replace("</", " ")
        .replace("<h3>", " ")
        .replace("<h2>", " ")
        .replace("<p>", " ")
        .replace("\n", " ")
        .replace(" and ", " ")
        .replace(" a ", " ")
        .replace(" be ", " ")
        .replace(" let ", " ")
        .replace("\t", " ");

    for it in _details.split(" ") {
        _text_set.insert(it);
    }

    let mut score = f32::MAX;
    for jt in query.split(" ") {
        // TODO skip simple words

        let mut _score = f32::MAX;
        for it in _text_set.iter() {
            let l = levenshtein_distance::levenshtein_dist_word(jt, it) as f32;
            _score = l / (1f32 + (jt.len() as f32 - it.len() as f32).abs());
        }
        score = score.min(_score);
    }
    // TODO turn off for production.
    console_log!("Score: {} {:?}", score, _text_set);
    return score;
}

fn update_search_results(input: JsValue, year: usize) {
    // TODO
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let textbox = document
        .get_element_by_id(&format!("search_textbox_{}", year))
        .unwrap(); // TODO add name

    let query = textbox
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value();

    let data_bytes = match get_data() {
        Some(data) => data,
        None => {
            console_log!("Data is not instantiated.");
            panic!();
        }
    };

    let accomplishment_data;
    match deserialize(data_bytes) {
        Ok(a) => {
            accomplishment_data = a;
        }
        Err(a) => {
            console_log!("{}", &a);
            panic!();
        }
    }
    // TODO we should sort the data.
    console_log!("Update the search results. {:?}", query);
    //let v = search_score_events(&accomplishment_data, &query);
    //console_log!("Got scores {:?}", v);

    // Clears the contents of the search_results html element.
    let result_div = document
        .get_element_by_id(&format!("search_results_{}", year))
        .unwrap();
    result_div.set_inner_html("");

    // TODO what should the score threshold be.
    // Prehaps it should be relative to the number of characters in the entries.
    //
    let filter = move |it: &Event| score_event(it, &query) < 1.5;
    let _ = render_events_table_fn(&document, &result_div, &accomplishment_data, &filter);
}


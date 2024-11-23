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

mod search;
use search::{construct_document_token_map, query_document_token_map};

mod dom_helper;
use dom_helper::{
    create_anchor, create_br, create_div, create_font, create_input, create_option,
    create_paragraph, create_select, create_table, create_th, create_tr, create_td,
    create_button, create_canvas
};

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
// - [X] search is a little better.
// - [X] use dyn_into::<web_sys::HtmlElement> for better type handling.
//

// TODO
// - summary is broken - on large site.
// - year drop down menu ordering
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
            Err(e) => {
                console_log!("Tracker render failure. {:?}", e);
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
    // TODO
    let select_year_div = create_div(document)?;

    console_log!("{} {:?}", active_year, years);
    let _ = render_year_dropdown(document, &select_year_div, &years, active_year);
    let _ = body.append_child(&select_year_div);
    for year in years.iter() {
        let div_year = create_div(document)?;
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
            let div_month = create_div(document)?;
            div_month.set_id(&format!("month_{}_{}", month_str, year));
            // TODO set a class so that we can turn off months
            div_month.set_class_name("month_class"); // TODO class name is a misnomer -- make this
                                                     // a global then change.
            if active_month != month {
                let _ = div_month.set_attribute("style", "display: none");
            }

            render_legend(document, &div_month)?;

            let data = collect_leadership_statistic(&at_data, *year, month);
            let div = create_div(document)?;
            div.set_class_name("tabcontent");
            div.set_id(&format!("monthly_log_{}_{}", month_str, year)); // TODO this name should be associated with an actual month.
            render_leadership_summary(document, &div, &data)?;
            render_events_table(document, &div, at_data, *year, month)?;

            let _ = div_month.append_child(&div);
            let _ = div_year.append_child(&div_month);
        }

        {
            let div_summary = create_div(document)?;
            div_summary.set_id(&format!("summary_{}", year));
            div_summary.set_class_name("month_class");
            let _ = div_summary.set_attribute("style", "display: none");
            {
                console_log!("Creating canvas ctx.");
                let canvas_principles_by_year = create_canvas(document)?;
                canvas_principles_by_year.set_id(&format!("canvas_principles_{}", year));
                let _ = div_summary.append_child(&canvas_principles_by_year);

                let canvas_principles_by_year = create_canvas(document)?;
                canvas_principles_by_year.set_id(&format!("canvas_principle_month_{}", year));
                let _ = div_summary.append_child(&canvas_principles_by_year);
            }
            let _ = div_year.append_child(&div_summary);
        }

        {
            // TODO we are creating multiple searches. There should only be one.
            let div_search = create_div(document)?;
            div_search.set_id(&format!("search_{}", year));
            div_search.set_class_name("month_class");
            let _ = div_search.set_attribute("style", "display: none");

            {
                let _year = *year;
                let a = Closure::<dyn Fn(JsValue)>::new(move |input| {
                    update_search_results(input, _year)
                }); //TODO
                let textbox = create_input(document)?;
                let _ = textbox.set_placeholder("Type word or phrase.");
                let _ = textbox.set_type("search");
                let _ = textbox.set_size(32);
                let _ = textbox.set_attribute("style", "font-size: 150%; align-items: center");
                textbox.set_id(&format!("search_textbox_{}", year));

                let _ =
                    textbox.add_event_listener_with_callback("input", a.as_ref().unchecked_ref());
                let _ = div_search.append_child(&textbox);

                let result_div = create_div(document)?;
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
    fn_pointer: &dyn Fn(&Event, usize) -> bool,
) -> Result<(), JsValue> {
    let table = create_table(document)?;
    {
        // NOTE: Header
        let tr = create_tr(document)?;
        {
            let th = create_th(document)?;
            th.set_class_name("smallColumn");
            th.set_text_content(Some("Date"));
            let _ = tr.append_child(&th);
        }
        {
            let th = create_th(document)?;
            let _ = th.set_width("22%");
            th.set_text_content(Some("Summary"));
            let _ = tr.append_child(&th);
        }
        {
            let th = create_th(document)?;
            th.set_text_content(Some("Details"));
            let _ = tr.append_child(&th);
        }
        {
            let th = create_th(document)?;
            let _ = th.set_width("20%");
            th.set_text_content(Some("Amazon Leadership Principles"));
            let _ = tr.append_child(&th);
        }
        let _ = table.append_child(&tr);
    }

    {
        // TODO what html element is this associated with.
        let tbody = document.create_element("tbody")?;
        for (i, it) in at_data.events.iter().enumerate() {
            if fn_pointer(it, i) == false {
                continue;
            }
            let tr = create_tr(document)?;
            match it.event_type {
                EventType::BarRaise => {
                    let _ = tr.set_bg_color(C_BAR_RAISING);
                }
                EventType::InvestInYourSelf => {
                    let _ = tr.set_bg_color(C_INVEST_IN_YOURSELF);
                }
                EventType::None => {}
            }

            {
                let td = create_td(document)?;
                td.set_class_name("valueCells");
                let date_string = it.date.format("%Y-%m-%d").to_string();
                td.set_text_content(Some(&date_string));
                tr.append_child(&td);
            }

            {
                let td = create_td(document)?;
                td.set_inner_html(&it.summary.as_str());
                let _ = tr.append_child(&td);
            }

            {
                // TODO call backs show more show less
                let td = create_td(document)?;
                td.set_inner_html(&it.details.as_str());
                let _ = tr.append_child(&td);
            }

            let _ = tbody.append_child(&tr);

            {
                // TODO call backs show more/show less
                let td = create_td(document)?;
                td.set_class_name("valueCells");
                for jt in it.leadership_principles.iter() {
                    // TODO remove duplets
                    if *jt == LeadershipPrinciples::Empty {
                        continue;
                    }
                    let string = format!("{}, ", jt.to_str());
                    let _ = td.append_with_str_1(&string);
                }
                let _ = tr.append_child(&td);
            }

            let _ = tbody.append_child(&tr);
        }
        let _ = table.append_child(&tbody);
    }
    let _ = div.append_child(&table);

    Ok(())
}

pub fn render_events_table(
    document: &web_sys::Document,
    div: &web_sys::Element,
    at_data: &_AccomplishmentData,
    year: usize,
    month: usize,
) -> Result<(), JsValue> {
    let filter = |it: &Event, _: usize| {
        it.date.year() as usize == year && it.date.month0() as usize == month
    };
    render_events_table_fn(document, div, at_data, &filter)
}

pub fn render_selection_by_date_menu(
    document: &web_sys::Document,
    body: &web_sys::HtmlDivElement,
    data: &[bool],
    active_month: usize,
    year: usize,
) -> Result<(), JsValue> {
    let menu_div = create_div(document)?; // TODO move outside and replace body
    menu_div.set_class_name("tab");
    menu_div.set_id(&format!("month_year_menu_{}", year));

    for (i, it) in data.iter().enumerate() {
        if *it {
            let month = Month::try_from(i as u8 + 1).unwrap().name();

            let button = create_button(document)?;
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
        let button = create_button(document)?;
        button.set_id(&format!("button_summary_{}", year));
        button.set_text_content(Some("Summary"));

        let a = Closure::<dyn Fn()>::new(move || set_month_tab("summary", year)); // TODO
        let _ = button.add_event_listener_with_callback("click", a.as_ref().unchecked_ref());

        menu_div.append_child(&button)?;
        a.forget(); // TODO explain why this is need. https://github.com/rustwasm/wasm-bindgen/issues/843
    }

    {
        let button = create_button(document)?;
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
    menu_div: &web_sys::HtmlDivElement,
    years: &HashSet<usize>,
    input_year: usize,
) -> Result<(), JsValue> {
    console_log!("render year dropdown.");

    let max_year = years.iter().max().unwrap();
    let a = Closure::<dyn Fn()>::new(move || set_year_option(input_year));
    let year_selection = {
        match document.get_element_by_id("year_selection") {
            Some(ys) => ys.dyn_into::<web_sys::HtmlSelectElement>().unwrap(),
            None => {
                let year_selection = create_select(document)?;
                let _ = year_selection.set_attribute(
                    "style",
                    "font-size: 2em; float: right; padding-right: 50px; padding-top: 8px",
                );
                year_selection.set_id("year_selection"); // TODO pass the id?
                let _ = year_selection
                    .add_event_listener_with_callback("change", a.as_ref().unchecked_ref());
                for year in years.iter() {
                    // TODO I don't think we need to do this any more.
                    // The years are provided by a hashset now we no longer use a fixed sided
                    // array.
                    if *year == 0 {
                        break;
                    }
                    let year_option = create_option(document)?;
                    let year_string = format!("{}", year);
                    year_option.set_text_content(Some(&year_string));

                    if *year == *max_year {
                        let _ = year_option.set_selected(true);
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
        let table = create_table(document)?;
        let _ = table.set_attribute("style", "font-size:12px"); // TODO
        {
            let tr = create_tr(document)?;
            {
                let th = create_th(document)?;
                let _ = th.set_attribute("style", "font-size:150%"); // TODO
                let _ = th.set_col_span(16); // TODO
                let _ = th.set_text_content(Some("Monthly Summary"));

                let _ = tr.append_child(&th);
            }
            let _ = table.append_child(&tr);

            // TODO: describe what happening here.
            let tr = create_tr(document)?;
            tr.set_class_name("summary");
            for it in LeadershipPrinciples::iterator() {
                if *it == LeadershipPrinciples::Empty {
                    continue;
                }

                let th = create_th(document)?;
                th.set_text_content(Some(it.to_str()));

                let _ = tr.append_child(&th);
            }
            let _ = table.append_child(&tr);

            // TODO: describe what happening here.
            let tr = create_tr(document)?;
            tr.set_class_name("summary");
            for it in LeadershipPrinciples::iterator() {
                if *it == LeadershipPrinciples::Empty {
                    continue;
                }

                let th = create_th(document)?;
                let value = data[*it as usize];
                match value {
                    0 => {}
                    1..=3 => {
                        // TODO
                        let _ = th.set_bg_color("#7d5a0c");
                    }
                    _ => {
                        let _ = th.set_bg_color("#892e44");
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
    let p = create_paragraph(&document)?;
    {
        p.set_class_name("legend");
        p.set_text_content(Some("Bar raising moment - "));
        let font = create_font(&document)?;
        let _ = font.set_color(C_BAR_RAISING);
        font.set_text_content(Some("  █"));
        let _ = p.append_child(&font);

        let br = create_br(&document)?;
        let _ = p.append_child(&br);

        p.append_with_str_1("Invest in yourself - ")?;
        let font = create_font(&document)?;
        let _ = font.set_color(C_INVEST_IN_YOURSELF);
        font.set_text_content(Some("  █"));
        let _ = p.append_child(&font);
    }
    body.append_child(&p)?;

    let p = create_paragraph(&document)?;
    {
        p.set_class_name("legend");
        p.set_text_content(Some("If you have feedback please follow this "));
        let a = create_anchor(&document)?;
        let _ = a.set_href("https://quip-amazon.com/yil8AxIlg78u/Accomplishment-and-Invest-in-Yourself-Tracker-Thoth#temp:C:LfXc7ddf386401743d0a4944584c"); // TODO this link url should be global
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
    console_log!("Update the search results. {:?}", query);

    // Clears the contents of the search_results html element.
    let result_div = document
        .get_element_by_id(&format!("search_results_{}", year))
        .unwrap();
    result_div.set_inner_html("");

    // let filter = move |it: &Event, _: usize| score_event(it, &query) < 1.5;
    let searchmap = construct_document_token_map(accomplishment_data.events);
    console_log!("{:?}", searchmap);
    let q_results = query_document_token_map(&query, &searchmap);

    let filter = move |_: &Event, i: usize| q_results.contains(&i);
    let _ = render_events_table_fn(&document, &result_div, &accomplishment_data, &filter);
}

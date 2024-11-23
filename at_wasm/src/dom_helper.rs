use crate::{JsCast, JsValue};
use web_sys::{
    Document, HtmlAnchorElement, HtmlBrElement, HtmlDivElement, HtmlFontElement,
    HtmlInputElement, HtmlParagraphElement, HtmlTableCellElement, HtmlTableElement,
    HtmlTableRowElement, HtmlOptionElement, HtmlSelectElement, HtmlButtonElement,
    HtmlCanvasElement
};

pub fn create_div(document: &Document) -> Result<HtmlDivElement, JsValue> {
    let rv = document.create_element("div")?;
    Ok(rv.dyn_into::<HtmlDivElement>().unwrap())
}

pub fn create_input(document: &Document) -> Result<HtmlInputElement, JsValue> {
    let rv = document.create_element("input")?;
    Ok(rv.dyn_into::<HtmlInputElement>().unwrap())
}

pub fn create_font(document: &Document) -> Result<HtmlFontElement, JsValue> {
    let rv = document.create_element("font")?;
    Ok(rv.dyn_into::<HtmlFontElement>().unwrap())
}

pub fn create_paragraph(document: &Document) -> Result<HtmlParagraphElement, JsValue> {
    let rv = document.create_element("p")?;
    Ok(rv.dyn_into::<HtmlParagraphElement>().unwrap())
}

pub fn create_br(document: &Document) -> Result<HtmlBrElement, JsValue> {
    let rv = document.create_element("br")?;
    Ok(rv.dyn_into::<HtmlBrElement>().unwrap())
}

pub fn create_anchor(document: &Document) -> Result<HtmlAnchorElement, JsValue> {
    let rv = document.create_element("a")?;
    Ok(rv.dyn_into::<HtmlAnchorElement>().unwrap())
}

pub fn create_th(document: &Document) -> Result<HtmlTableCellElement, JsValue> {
    let rv = document.create_element("th")?;
    Ok(rv.dyn_into::<HtmlTableCellElement>().unwrap())
}

pub fn create_td(document: &Document) -> Result<HtmlTableCellElement, JsValue> {
    let rv = document.create_element("td")?;
    Ok(rv.dyn_into::<HtmlTableCellElement>().unwrap())
}

pub fn create_tr(document: &Document) -> Result<HtmlTableRowElement, JsValue> {
    let rv = document.create_element("tr")?;
    Ok(rv.dyn_into::<HtmlTableRowElement>().unwrap())
}

pub fn create_table(document: &Document) -> Result<HtmlTableElement, JsValue> {
    let rv = document.create_element("table")?;
    Ok(rv.dyn_into::<HtmlTableElement>().unwrap())
}

pub fn create_select(document: &Document) -> Result<HtmlSelectElement, JsValue> {
    let rv = document.create_element("select")?;
    Ok(rv.dyn_into::<HtmlSelectElement>().unwrap())
}

pub fn create_option(document: &Document) -> Result<HtmlOptionElement, JsValue> {
    let rv = document.create_element("option")?;
    Ok(rv.dyn_into::<HtmlOptionElement>().unwrap())
}

pub fn create_button(document: &Document) -> Result<HtmlButtonElement, JsValue> {
    let rv = document.create_element("button")?;
    Ok(rv.dyn_into::<HtmlButtonElement>().unwrap())
}

pub fn create_canvas(document: &Document) -> Result<HtmlCanvasElement, JsValue> {
    let rv = document.create_element("canvas")?;
    Ok(rv.dyn_into::<HtmlCanvasElement>().unwrap())
}

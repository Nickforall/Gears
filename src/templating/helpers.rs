use hbs::handlebars::*;

pub fn not_eq_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let left = h.param(0).unwrap().value();
    let right = h.param(1).unwrap().value();

    let result = left != right;

    if result {
        h.template().map(|t| t.render(r, rc)).unwrap_or(Ok(()));
    }

    Ok(())
}

pub fn eq_helper(h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let left = h.param(0).unwrap().value();
    let right = h.param(1).unwrap().value();

    let result = left == right;

    if result {
        h.template().map(|t| t.render(r, rc)).unwrap_or(Ok(()));
    }

    Ok(())
}

pub fn debug (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    // just for example, add error check for unwrap
    let arg = h.param(0).unwrap().value();

    rc.writer().write(format!("{:?}", arg).into_bytes().as_ref())?;

    Ok(())
}

pub fn fmt_time (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    use chrono::{NaiveDateTime};

    // just for example, add error check for unwrap
    let arg = h.param(0).unwrap().value();

    let dt = NaiveDateTime::from_timestamp(arg.as_i64().unwrap(), 0);

    rc.writer().write(dt.format("%Y-%m-%d %H:%M:%S").to_string().into_bytes().as_ref())?;

    Ok(())
}

pub fn to_string (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    // just for example, add error check for unwrap
    let arg = h.param(0).unwrap().value();

    rc.writer().write(to_json(&arg).to_string().into_bytes().as_ref())?;

    Ok(())
}

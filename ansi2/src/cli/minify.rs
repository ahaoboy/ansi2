use crate::cli::error::{Error, Result};

pub fn minify_svg(svg: &str) -> Result<String> {
    use oxvg_ast::{
        parse::roxmltree::parse,
        serialize::Node as _,
        visitor::Info,
        xmlwriter::{Options, Space},
    };
    use oxvg_optimiser::Jobs;

    let config = Jobs::default();
    let options = Options {
        trim_whitespace: Space::Never,
        ..Default::default()
    };

    let result = parse(svg, |dom, allocator| {
        config
            .run(dom, &Info::new(allocator))
            .map_err(|error| error.to_string())?;
        dom.serialize_with_options(options)
            .map_err(|error| error.to_string())
    })
    .map_err(|error| Error::Compress(error.to_string()))?;

    result.map_err(Error::Compress)
}

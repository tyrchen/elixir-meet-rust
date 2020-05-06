use rustler::{Encoder, Env, Error, Term};
use comrak::{markdown_to_html, ComrakOptions};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler::rustler_export_nifs! {
    "Elixir.Rmark",
    [
        ("to_html", 1, to_html)
    ],
    None
}

fn to_html<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let md: String = args[0].decode()?;

    Ok((atoms::ok(), markdown_to_html(&md, &ComrakOptions::default())).encode(env))
}

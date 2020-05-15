use comrak::{markdown_to_html, ComrakOptions};
use rustler::schedule::SchedulerFlags::DirtyCpu;
use rustler::{Encoder, Env, Error, Term};

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
        ("to_html", 1, to_html),
        ("to_html_spawn", 1, to_html_spawn),
        ("to_html_dirty", 1, to_html, DirtyCpu),
    ],
    None
}

fn to_html<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let md: String = args[0].decode()?;

    Ok((
        atoms::ok(),
        markdown_to_html(&md, &ComrakOptions::default()),
    )
        .encode(env))
}

fn to_html_spawn<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let md: String = args[0].decode()?;
    use rustler::thread;
    thread::spawn::<thread::ThreadSpawner, _>(env, move |env| {
        (
            atoms::ok(),
            markdown_to_html(&md, &ComrakOptions::default()),
        )
            .encode(env)
    });
    Ok(atoms::ok().encode(env))
}

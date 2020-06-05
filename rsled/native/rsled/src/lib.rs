use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, Term};
use sled::Db;
use std::str::from_utf8;

rustler::atoms! {
    ok,
    error,
    not_found,
    internal
}

struct SledResource(Db);

enum UpdateResult {
    Ok,
    InternalError,
}
enum GetResult {
    Ok(String),
    NotFound,
    InternalError,
}

impl<'a> Encoder for UpdateResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            UpdateResult::Ok => ok().encode(env),
            UpdateResult::InternalError => (error(), internal()).encode(env),
        }
    }
}

impl<'a> Encoder for GetResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            GetResult::Ok(s) => (ok(), s).encode(env),
            GetResult::NotFound => (error(), not_found()).encode(env),
            GetResult::InternalError => (error(), internal()).encode(env),
        }
    }
}

#[rustler::nif]
fn open(p: &str) -> ResourceArc<SledResource> {
    let tree = sled::open(p).unwrap();
    ResourceArc::new(SledResource(tree))
}

#[rustler::nif]
fn put(arc: ResourceArc<SledResource>, k: &str, v: &str) -> UpdateResult {
    let tree = &arc.0;
    match tree.insert(k, v) {
        Ok(_) => UpdateResult::Ok,
        _ => UpdateResult::InternalError,
    }
}

#[rustler::nif]
fn get(arc: ResourceArc<SledResource>, k: &str) -> GetResult {
    let tree = &arc.0;
    match tree.get(k) {
        Ok(Some(v)) => GetResult::Ok(from_utf8(&v).unwrap().to_owned()),
        Ok(None) => GetResult::NotFound,
        _ => GetResult::InternalError,
    }
}

#[rustler::nif]
fn has_key(arc: ResourceArc<SledResource>, k: &str) -> bool {
    let tree = &arc.0;
    match tree.contains_key(k) {
        Ok(true) => true,
        _ => false,
    }
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(SledResource, env);
    true
}

rustler::init!("Elixir.Rsled", [open, put, get, has_key], load = on_load);

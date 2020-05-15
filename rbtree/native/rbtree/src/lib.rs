use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, Term};
use std::collections::BTreeMap;
use std::sync::RwLock;

rustler::atoms! {
    ok,
    error,
    not_found,
    internal
}

struct BTreeResource(RwLock<BTreeMap<String, String>>);

enum UpdateResult {
    Ok,
    InternalError,
}
enum GetResult {
    Ok(String),
    NotFound,
    InternalError,
}

enum IterResult {
    Ok(Vec<(String, String)>),
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

impl<'a> Encoder for IterResult {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            IterResult::Ok(s) => (ok(), s).encode(env),
            IterResult::InternalError => (error(), internal()).encode(env),
        }
    }
}

#[rustler::nif]
fn new() -> ResourceArc<BTreeResource> {
    let tree: BTreeMap<String, String> = BTreeMap::new();
    let mutex = RwLock::new(tree);
    ResourceArc::new(BTreeResource(mutex))
}

#[rustler::nif]
fn put(arc: ResourceArc<BTreeResource>, key: String, value: String) -> UpdateResult {
    match arc.0.write() {
        Ok(mut map) => {
            map.insert(key, value);
            UpdateResult::Ok
        }
        Err(_) => UpdateResult::InternalError,
    }
}

#[rustler::nif]
fn get(arc: ResourceArc<BTreeResource>, key: String) -> GetResult {
    match arc.0.read() {
        Ok(map) => {
            if let Some(s) = map.get(&key) {
                GetResult::Ok(s.to_owned())
            } else {
                GetResult::NotFound
            }
        }
        Err(_) => GetResult::InternalError,
    }
}

#[rustler::nif]
fn has_key(arc: ResourceArc<BTreeResource>, key: String) -> bool {
    match arc.0.read() {
        Ok(map) => map.contains_key(&key),
        Err(_) => false,
    }
}

#[rustler::nif]
fn delete(arc: ResourceArc<BTreeResource>, key: String) -> UpdateResult {
    match arc.0.write() {
        Ok(mut map) => {
            map.remove(&key);
            UpdateResult::Ok
        }
        Err(_) => UpdateResult::InternalError,
    }
}

#[rustler::nif]
fn to_list(arc: ResourceArc<BTreeResource>) -> IterResult {
    match arc.0.read() {
        Ok(m) => {
            let v: Vec<(String, String)> = m
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect();
            IterResult::Ok(v)
        }
        Err(_) => IterResult::InternalError,
    }
}

#[rustler::nif]
fn crash_me_please(_arc: ResourceArc<BTreeResource>) {
    panic!("goodbye world!");
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(BTreeResource, env);
    true
}

rustler::init!(
    "Elixir.Rbtree",
    [new, put, get, has_key, delete, to_list, crash_me_please],
    load = on_load
);

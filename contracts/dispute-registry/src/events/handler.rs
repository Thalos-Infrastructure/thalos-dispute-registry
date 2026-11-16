use soroban_sdk::{symbol_short, Env, Symbol};

/// Topic helpers. Week 2 emits a topic per action (open, vote, expire, linked).
pub fn topic_opened() -> Symbol {
    symbol_short!("opened")
}

pub fn topic_voted() -> Symbol {
    symbol_short!("voted")
}

pub fn topic_linked() -> Symbol {
    symbol_short!("linked")
}

pub fn ping(env: &Env, topic: Symbol) {
    env.events().publish((topic,), ());
}

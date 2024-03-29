use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub const MAX_PAYLOAD_LEN: usize = 50_000;

pub fn alphanumeric(len: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

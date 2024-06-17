use pretty_env_logger::env_logger;

pub fn build_env_logger() {
    let env_log = env_logger::Env::new();
    let env_log = env_log.default_filter_or("info");
    env_logger::init_from_env(env_log);
}

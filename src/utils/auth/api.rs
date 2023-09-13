use rand::Rng;

pub async fn create_api_key() -> i64 {
    rand::thread_rng().gen_range(1000000000..9999999999)
}

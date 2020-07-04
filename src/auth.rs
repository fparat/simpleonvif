use crate::onvif;

pub fn get_auth_header(user: &str, password: &str) -> String {
    use rand::prelude::*;

    let mut nonce = [0u8; 22];
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_b64 = base64::encode(&nonce);

    let created = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let digest_b64 = get_digest_b64(&nonce_b64, &created, password);

    let auth = onvif::auth_header(&user, &digest_b64, &nonce_b64, &created);

    auth
}

fn get_digest_b64(nonce_b64: &str, date: &str, password: &str) -> String {
    let mut digest_base_buf = Vec::with_capacity(128);
    digest_base_buf.extend_from_slice(&base64::decode(nonce_b64).expect("not a base64 string"));
    digest_base_buf.extend_from_slice(date.as_bytes());
    digest_base_buf.extend_from_slice(password.as_bytes());
    let digest_base = &digest_base_buf[..];

    let mut digest_buf = [0u8; 20];
    let digest_size = sha1(&digest_base, &mut digest_buf);
    let digest = &digest_buf[..digest_size];

    let digest_b64 = base64::encode(&digest);
    digest_b64
}

fn sha1(buf: &[u8], out: &mut [u8]) -> usize {
    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
    let mut hasher = Sha1::new();
    hasher.input(buf);
    hasher.result(out);
    hasher.output_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_example() {
        let nonce = "LKqI6G/AikKCQrN0zqZFlg==";
        let date = "2010-09-16T07:50:45Z";
        let password = "userpassword";
        let expected = "tuOSpGlFlIXsozq4HFNeeGeFLEI=";

        assert_eq!(get_digest_b64(nonce, date, password), expected);
    }
}

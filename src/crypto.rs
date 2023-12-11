use ring::hmac::{Context, Key, Tag, HMAC_SHA512};

pub fn hmac512(data: &[u8], key: &[u8]) -> [u8; 64] {
    let tag: Tag = {
        let key = Key::new(HMAC_SHA512, key);
        let mut context = Context::with_key(&key);
        context.update(data);
        context.sign()
    };

    tag.as_ref().try_into().unwrap()
}

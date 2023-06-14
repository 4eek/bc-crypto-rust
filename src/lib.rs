#![doc(html_root_url = "https://docs.rs/bc-crypto/0.1.0")]
#![warn(rust_2018_idioms)]
#![feature(bigint_helper_methods)]

pub const SYMMETRIC_KEY_SIZE: usize = 32;
pub const SYMMETRIC_NONCE_SIZE: usize = 12;
pub const SYMMETRIC_AUTH_SIZE: usize = 16;
pub const ECDSA_PRIVATE_KEY_SIZE: usize = 32;
pub const ECDSA_PUBLIC_KEY_SIZE: usize = 33;
pub const ECDSA_UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = 65;
pub const ECDSA_MESSAGE_HASH_SIZE: usize = 32;
pub const ECDSA_SIGNATURE_SIZE : usize = 64;
pub const SCHNORR_PUBLIC_KEY_SIZE: usize = 32;
pub const SCHNORR_SIGNATURE_SIZE: usize = 64;
pub const X25519_PRIVATE_KEY_SIZE: usize = 32;
pub const X25519_PUBLIC_KEY_SIZE: usize = 32;

mod magnitude;
mod widening;

pub mod hash;

mod memzero;
pub use memzero::{memzero, memzero_vec_vec_u8};

mod symmetric_encryption;
pub use symmetric_encryption::{
    encrypt_aead_chacha20_poly1305_with_aad,
    encrypt_aead_chacha20_poly1305,
    decrypt_aead_chacha20_poly1305_with_aad,
    decrypt_aead_chacha20_poly1305
};

mod public_key_encryption;
pub use public_key_encryption:: {
    x25519_new_agreement_private_key_using,
    x25519_agreement_public_key_from_private_key,
    x25519_derive_agreement_private_key,
    x25519_derive_signing_private_key,
    x25519_shared_key,
};

mod ecdsa_keys;
pub use ecdsa_keys::{
    ecdsa_new_private_key_using,
    ecdsa_public_key_from_private_key,
    ecdsa_decompress_public_key,
    ecdsa_compress_public_key,
    ecdsa_derive_private_key,
    schnorr_public_key_from_private_key,
};

mod ecdsa_signing;
pub use ecdsa_signing::{
    ecdsa_sign,
    ecdsa_verify,
};

mod schnorr_signing;
pub use schnorr_signing::{
    schnorr_sign,
    schnorr_sign_using,
    schnorr_verify,
};

mod random_number_generator;
pub use random_number_generator::RandomNumberGenerator;

mod secure_random;
pub use secure_random::{
    SecureRandomNumberGenerator,
    random_data,
    fill_random_data
};

mod seeded_random;
pub use seeded_random::{
    SeededRandomNumberGenerator,
    fake_random_data,
    make_fake_random_number_generator
};

mod error;
pub use error::Error;

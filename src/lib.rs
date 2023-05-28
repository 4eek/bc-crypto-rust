#![feature(bigint_helper_methods)]

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

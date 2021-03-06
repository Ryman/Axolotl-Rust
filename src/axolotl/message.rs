use super::axolotl::{Axolotl};
use super::dh::{DHPublic};

pub struct AxolotlMessage<T> where T:Axolotl {
    pub message_number : u32,
    pub ratchet_key : DHPublic<T::RatchetKey>,
    pub ciphertext : T::CipherText,
}
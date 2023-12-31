#[cfg(not(test))]
use rand_chacha::rand_core::RngCore;
#[cfg(all(not(test), not(feature = "std")))]
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};

#[cfg(all(not(test), not(feature = "std")))]
use crate::seed;

use crate::{
    impl_command_display, impl_command_ops, impl_default, impl_encrypted_message_ops,
    impl_message_from_buf, len, AesKey, CommandOps, Error, MessageOps, Result, SequenceCount,
};

use super::{encrypted_index as index, WrappedEncryptedMessage};

/// Encrypted - Command (0x7E)
///
/// The encrypted packet is wrapped inside the data field of a standard SSP packet. The
/// encrypted section is constructed from the following fields.
///
/// | STEX | LENGTH | COUNT 0 | COUNT 1 | COUNT 2 | COUNT 3 | DATA 0 | ... | DATA N | PACKING 0 | ... | PACKING N | CRC_L | CRC_H |
/// |:----:|:------:|:-------:|:-------:|:-------:|:-------:|:------:|:---:|:------:|:---------:|:---:|:---------:|:-----:|:-----:|
/// | 0x7E | 0xnn   | 0xnn    | 0xnn    | 0xnn    | 0xnn    | 0xnn   | ... | 0xnn   | 0xnn      | ... | 0xnn      | 0xnn  | 0xnn  |
#[repr(C)]
#[derive(Clone, Debug, PartialEq, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct EncryptedCommand {
    buf: [u8; len::ENCRYPTED_COMMAND],
}

impl EncryptedCommand {
    /// Creates a new [EncryptedCommand].
    pub fn new() -> Self {
        let mut msg = Self {
            buf: [0u8; len::ENCRYPTED_COMMAND],
        };

        msg.init();

        msg
    }

    /// Gets the sequence count.
    pub fn count(&self) -> SequenceCount {
        self.count_buf().into()
    }

    fn count_buf(&self) -> &[u8] {
        self.buf[index::COUNT..index::COUNT_END].as_ref()
    }

    fn set_count(&mut self, count: SequenceCount) {
        self.buf[index::COUNT..index::COUNT_END]
            .copy_from_slice(count.as_inner().to_le_bytes().as_ref());
    }

    /// Builder function that sets the [SequenceCount].
    pub fn with_count(mut self, count: SequenceCount) -> Self {
        self.set_count(count);
        self
    }

    /// Gets the message data.
    pub fn message_data(&self) -> &[u8] {
        let start = self.data_start();
        let end = self.data_end();

        self.buf[start..end].as_ref()
    }

    fn data_start(&self) -> usize {
        index::DATA
    }

    fn data_end(&self) -> usize {
        self.data_start() + self.data_len()
    }

    /// Sets the message data.
    ///
    /// Returns `Err(_)` if the message data exceeds the maximum length.
    ///
    /// Maximum length: [MAX_ENCRYPTED_DATA](crate::len::MAX_ENCRYPTED_DATA)
    ///
    /// This maximum refers to the data field of the wrapped inner message.
    ///
    /// Encrypted messages wrap the data field of a standard SSP message, and then the entire
    /// encrypted message is wrapped in an outer standard SSP message.
    ///
    /// Matryoshka dolls all the way down...
    pub fn set_message_data(&mut self, message: &dyn CommandOps) -> Result<()> {
        let len = message.data_len();

        if message.data().len() != len {
            return Err(Error::InvalidDataLength((len, message.data().len())));
        }

        if (0..=len::MAX_ENCRYPTED_DATA).contains(&len) {
            self.set_data_len(len as u8);

            let start = self.data_start();
            let end = self.data_end();

            let data = message.data();

            log::trace!("Encrypted data: {data:x?}, length: {len}");

            self.buf[start..end].copy_from_slice(data);

            Ok(())
        } else {
            Err(Error::InvalidDataLength((len, len::MAX_ENCRYPTED_DATA)))
        }
    }

    /// Builder function that sets the message data.
    pub fn with_message_data(mut self, message: &dyn CommandOps) -> Result<Self> {
        self.set_message_data(message)?;
        Ok(self)
    }

    /// Gets random packing data used to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    pub fn packing(&self) -> &[u8] {
        let start = self.packing_start();
        let end = self.packing_end();

        self.buf[start..end].as_ref()
    }

    /// Adds random packing data to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    #[cfg(not(feature = "std"))]
    pub fn set_packing(&mut self) {
        if self.packing_len() == 0 {
            return;
        }

        // Less robust than using rand::thread_rng, but still better than PKCS#7 padding...
        #[cfg(not(test))]
        let mut rng = ChaCha20Rng::from_seed(seed(self.buf(), self.count_buf()));

        let start = self.packing_start();
        let end = self.packing_end();

        #[cfg(not(test))]
        rng.fill_bytes(&mut self.buf[start..end]);
        #[cfg(test)]
        self.buf[start..end].copy_from_slice([0; 255][..end - start].as_ref());
    }

    /// Adds random packing data to make the encrypted packet a mutliple of the [AES block
    /// length](crate::len::AES).
    ///
    /// The encrypted fields are:
    ///
    /// ```no_build,no_run
    /// LEN + COUNT + DATA + PACKING + CRC_L + CRC_H
    /// ```
    #[cfg(feature = "std")]
    pub fn set_packing(&mut self) {
        if self.packing_len() == 0 {
            return;
        }

        #[cfg(not(test))]
        let mut rng = rand::thread_rng();

        let start = self.packing_start();
        let end = self.packing_end();

        #[cfg(not(test))]
        rng.fill_bytes(&mut self.buf[start..end]);
        #[cfg(test)]
        self.buf[start..end].copy_from_slice([0; 255][..end - start].as_ref());
    }

    fn packing_start(&self) -> usize {
        self.data_end()
    }

    fn packing_end(&self) -> usize {
        self.packing_start() + self.packing_len()
    }

    /// Gets the length of packing bytes needed to make the packet's encrypted data a multiple of
    /// the [AES block length](crate::len::AES).
    pub fn packing_len(&self) -> usize {
        // count all metadata bytes except STEX
        let meta = len::METADATA - 1;
        let raw_len = meta + self.data_len();

        len::aes_packing_len(raw_len)
    }

    fn encrypt_data(&mut self) -> &mut [u8] {
        let len = self.len();
        self.buf[index::LEN..len].as_mut()
    }

    /// Encrypts and consumes the [EncryptedCommand] message.
    ///
    /// Converts the [EncryptedCommand] message into a standard [WrappedEncryptedMessage].
    pub fn encrypt(mut self, key: &AesKey) -> WrappedEncryptedMessage {
        //use crate::aes;
        use aes::cipher::{BlockEncrypt, KeyInit};

        self.set_packing();

        if super::sequence_count().as_inner() != 0 {
            self.set_count(super::sequence_count());
        }

        self.calculate_checksum();

        let mut enc_msg = WrappedEncryptedMessage::new();

        let enc_len = self.len();
        enc_msg.set_data_len(enc_len as u8);

        log::trace!("Encrypted message: {:x?}", self.buf());

        let plain_data = self.encrypt_data();
        let cipher_data = enc_msg.data_mut()[1..].as_mut();

        let ciph = aes::Aes128::new(key);

        for (pchunk, cchunk) in plain_data
            .chunks_exact(16)
            .zip(cipher_data.chunks_exact_mut(16))
        {
            ciph.encrypt_block_b2b(pchunk.into(), cchunk.into());
        }

        enc_msg.calculate_checksum();
        if let Err(err) = enc_msg.verify_checksum() {
            log::error!("error validating wrapped encrypted checksum: {err}");
        }

        if let Err(err) = enc_msg.stuff_encrypted_data() {
            log::error!("error stuffing encrypted command message: {err}");
        }

        let count = super::sequence_count();
        let next_count = super::increment_sequence_count();

        log::trace!("encryption sequence count: {count}");
        log::trace!("next encryption sequence count: {next_count}");

        enc_msg
    }

    /// Decrypts and consumes the [WrappedEncryptedMessage].
    ///
    /// Converts the [WrappedEncryptedMessage] into an [EncryptedCommand].
    ///
    /// **Note**: only useful if implementing a device-side binary, and/or testing host-side
    /// functionality.
    pub fn decrypt(key: &AesKey, mut message: WrappedEncryptedMessage) -> Self {
        use crate::aes;

        if let Err(err) = message.unstuff_encrypted_data() {
            log::error!("error unstuffing encrypted command message: {err}");
        }

        let mut dec_msg = Self::new();
        dec_msg.set_data_len(message.data_len().saturating_sub(len::ENCRYPTED_METADATA) as u8);

        // Skip the STEX (0x7E) byte, it's not encrypted/decrypted
        let cipher_data = message.data()[1..].as_ref();
        let plain_data = dec_msg.encrypt_data();

        if let Err(err) = aes::aes_decrypt_inplace(key.as_ref(), cipher_data, plain_data) {
            log::error!("error decrypting message: {err}");
        }

        super::increment_sequence_count();

        dec_msg
    }
}

impl_default!(EncryptedCommand);
impl_command_display!(EncryptedCommand);
impl_message_from_buf!(EncryptedCommand);
impl_encrypted_message_ops!(EncryptedCommand);
impl_command_ops!(EncryptedCommand);

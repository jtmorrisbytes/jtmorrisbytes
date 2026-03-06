use std::i32;

use anyhow::Context;
use openssl::cipher::Cipher;
use openssl::derive::Deriver;
use openssl::dh::Dh;
use openssl::error::ErrorStack;
use openssl::pkey::{Id, PKey, Private};

// pub fn generate_vault_primary_pkcs8_from_entropy(entropy: &[u8;32]) -> Result<Vec<u8>,openssl::error::ErrorStack> {
//     let p = PKey::private_key_from_raw_bytes(entropy.as_slice(),Id::X25519)?;

//     let e = p.private_key_to_pem_pkcs8_passphrase(
//         Cipher::aes_256_cbc(),
//         passphrase
//     )?
//     Ok(e)
// }

// workaround since hkdf interface not yet supported in main openssl
// this function requires  openssl 1.1.1

struct HkdfPkeyCtx(*mut openssl_sys::EVP_PKEY_CTX);
impl Drop for HkdfPkeyCtx {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                openssl_sys::EVP_PKEY_CTX_free(self.0);
            }
        }
    }
}
impl std::ops::Deref for HkdfPkeyCtx {
    type Target = *mut openssl_sys::EVP_PKEY_CTX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
#[error("openssl_sys::EVP_PKEY_CTX_set_hkdf_md() Error: {0}")]
pub enum OpenSSlEvpPkeyCtxSetHkdfModeError {
    #[error(
        "The software attempted to pass in a null context as the first argument to this function"
    )]
    NullContextError,
    #[error(
        "The software attempted to pass in a null mode pointer as the second argument to this function"
    )]
    NullModePointerError,
    #[error("The specified function returned a generic error status {0}")]
    Error(i32),
    #[error(
        "Unsupported algorythm. The specified mode is not suppported or not properly initialized"
    )]
    UnsupportedAlgorythm,
}
/// ctx helper function for hkdf 'openssl sys evp pkey context set hdkf mode (hash? mode)'
fn openssl_sys_evp_pkey_ctx_set_hkdf_md(
    context: &HkdfPkeyCtx,
    md: *const openssl_sys::EVP_MD,
) -> Result<(), OpenSSlEvpPkeyCtxSetHkdfModeError> {
    if context.is_null() {
        return Err(OpenSSlEvpPkeyCtxSetHkdfModeError::NullContextError);
    }
    if md.is_null() {
        return Err(OpenSSlEvpPkeyCtxSetHkdfModeError::NullModePointerError);
    }
    let status = unsafe { openssl_sys::EVP_PKEY_CTX_set_hkdf_md(context.0, md) };
    match status {
        i32::MIN..=-3 => Err(OpenSSlEvpPkeyCtxSetHkdfModeError::Error(status)),
        -2 => return Err(OpenSSlEvpPkeyCtxSetHkdfModeError::UnsupportedAlgorythm),
        -1..=0 => Err(OpenSSlEvpPkeyCtxSetHkdfModeError::Error(status)),
        1..=i32::MAX => Ok(()),
    }
}
#[derive(thiserror::Error, Debug)]
#[error("openssl_sys::EVP_PKEY_CTX_set1_hkdf_key() Error: {0}")]
pub enum OpenSslEvpPkeyCtxSet1HkdfKeyError {
    #[error("A null context pointer was passed as the first argument to this function")]
    NullContextError,
    #[error(
        "Failed to conver the length of the provided data buffer into an i32 because: '{0}' and performing this operation would cause a panic"
    )]
    KeyLenConversionError(std::num::TryFromIntError),
    #[error(
        "The software attempted to pass an empty key to this function and performing this operation may result in an error"
    )]
    EmptyKeyError,
    #[error("The function returned a non 1 status code {0}")]
    Error(i32),
}

fn openssl_sys_evp_pkey_ctx_set1_hkdf_key<B: AsRef<[u8]>>(
    context: &HkdfPkeyCtx,
    data: B,
) -> Result<(), OpenSslEvpPkeyCtxSet1HkdfKeyError> {
    if context.is_null() {
        return Err(OpenSslEvpPkeyCtxSet1HkdfKeyError::NullContextError);
    }
    let data = data.as_ref();
    if data.len() == 0 {
        return Err(OpenSslEvpPkeyCtxSet1HkdfKeyError::EmptyKeyError);
    }
    let data_len: i32 = data
        .len()
        .try_into()
        .map_err(|e| OpenSslEvpPkeyCtxSet1HkdfKeyError::KeyLenConversionError(e))?;
    let status =
        unsafe { openssl_sys::EVP_PKEY_CTX_set1_hkdf_key(context.0, data.as_ptr(), data_len) };
    if status != 1 {
        return Err(OpenSslEvpPkeyCtxSet1HkdfKeyError::Error(status));
    }
    Ok(())
}
#[derive(thiserror::Error, Debug)]
#[error("openssl_sys::EVP_PKEY_CTX_set1_hdkf_salt() error: {0}")]
pub enum OpenSslEvpPkeyCtxSet1HkdfSaltError {
    #[error("The sofware attempted to pass a null context as the first argument to this function")]
    NullContextError,
    #[error(
        "Failed to convert the length of the salt buffer to an i32, and performing this operation would cause a panic. Cause: {0}"
    )]
    SaltLenConversionError(std::num::TryFromIntError),
    #[error("The function returned a non one status code")]
    Error(i32),
}

/// helper function to set the salt
fn openssl_sys_evp_pkey_ctx_set1_hkdf_salt<Salt: AsRef<[u8]>>(
    ctx: &HkdfPkeyCtx,
    salt: Salt,
) -> Result<(), OpenSslEvpPkeyCtxSet1HkdfSaltError> {
    if ctx.is_null() {
        return Err(OpenSslEvpPkeyCtxSet1HkdfSaltError::NullContextError);
    }

    let salt = salt.as_ref();

    let salt_len: i32 = salt
        .len()
        .try_into()
        .map_err(|e| OpenSslEvpPkeyCtxSet1HkdfSaltError::SaltLenConversionError(e))?;

    let status =
        unsafe { openssl_sys::EVP_PKEY_CTX_set1_hkdf_salt(ctx.0, salt.as_ptr(), salt_len) };

    if status != 1 {
        return Err(OpenSslEvpPkeyCtxSet1HkdfSaltError::Error(status));
    }

    Ok(())
}

#[derive(thiserror::Error,Debug)]
#[error("openssl_sys::EVP_PKEY_CTX_add1_hkdf_info() Error: {0} ")]
pub enum OpenSslSysEvpPkeyCtxAdd1HkdfInfoError {
    #[error("The software attempted to pass a null context into this function")]
    NullContextError,
    #[error("Could not convert the length of the info buffer to an i32, and doing so would cause a panic. Cause: {0}")]
    InfoLenConversionError(std::num::TryFromIntError),
    #[error("The specified function returned a non-one error context")]
    Error(i32)
}

fn openssl_sys_evp_pkey_ctx_add1_hdkf_info<Info: AsRef<[u8]>>(context: &HkdfPkeyCtx,info: Info) -> Result<(),OpenSslSysEvpPkeyCtxAdd1HkdfInfoError> {
    if context.is_null() {
        return Err(OpenSslSysEvpPkeyCtxAdd1HkdfInfoError::NullContextError);
    }
    let data = info.as_ref();
    let data_len: i32 = data.len().try_into().unwrap();
    let status = unsafe {
        openssl_sys::EVP_PKEY_CTX_add1_hkdf_info(context.0, data.as_ptr(), data_len)
    };
    if status != 1 {
        return Err(OpenSslSysEvpPkeyCtxAdd1HkdfInfoError::Error(status))
    }
    Ok(())
}


#[derive(thiserror::Error, Debug)]
#[error("HkdfError: {0}")]
pub enum HkdfError {
    #[error("Openssl returned a null context pointer while trying to call EVP_PKEY_CTX_new_id")]
    NullContextDuringContextCreation,
    #[error(
        "Openssl reported that it failed to initialize the 'derive': numerical error status {0}"
    )]
    DeriveInitFailed(i32),
    #[error("Failed to set the hkdf hash mode: {0}")]
    SetModeFailure(OpenSSlEvpPkeyCtxSetHkdfModeError),
    #[error("failed to set the key material {0}")]
    SetKeyMaterialFailure(OpenSslEvpPkeyCtxSet1HkdfKeyError),
    #[error("Failed to set the salt {0}")]
    SetSaltFailure(OpenSslEvpPkeyCtxSet1HkdfSaltError),
    #[error("Failed to set the key info {0}")]
    SetInfoFailure(OpenSslSysEvpPkeyCtxAdd1HkdfInfoError),
    #[error("Derivation Failed")]
    DerivationFailed
}
impl std::convert::From<OpenSSlEvpPkeyCtxSetHkdfModeError> for HkdfError {
    fn from(value: OpenSSlEvpPkeyCtxSetHkdfModeError) -> Self {
        Self::SetModeFailure(value)
    }
}
impl std::convert::From<OpenSslEvpPkeyCtxSet1HkdfKeyError> for HkdfError {
    fn from(value: OpenSslEvpPkeyCtxSet1HkdfKeyError) -> Self {
        Self::SetKeyMaterialFailure(value)
    }
}
impl std::convert::From<OpenSslEvpPkeyCtxSet1HkdfSaltError> for HkdfError {
    fn from(value: OpenSslEvpPkeyCtxSet1HkdfSaltError) -> Self {
        Self::SetSaltFailure(value)
    }
}
impl std::convert::From<OpenSslSysEvpPkeyCtxAdd1HkdfInfoError> for HkdfError {
    fn from(value: OpenSslSysEvpPkeyCtxAdd1HkdfInfoError) -> Self {
        Self::SetInfoFailure(value)
    }
}

fn hdkf<PKeyBuf: AsRef<[u8]>, Salt: AsRef<[u8]>,Info:AsRef<[u8]>>(
    pkey: PKeyBuf,
    salt: Salt,
    info: Info
) -> Result<([u8;32],usize), HkdfError> {
    use openssl_sys::{EVP_PKEY_HKDF, EVP_sha256};
    let ctx = unsafe { openssl_sys::EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, core::ptr::null_mut()) };
    if ctx.is_null() {
        return Err(HkdfError::NullContextDuringContextCreation);
    }
    let ctx = HkdfPkeyCtx(ctx);

    // initialize derivation function
    let status = unsafe { openssl_sys::EVP_PKEY_derive_init(*ctx) };
    if status != 1 {
        return Err(HkdfError::DeriveInitFailed(status));
    }
    unsafe {
        openssl_sys_evp_pkey_ctx_set_hkdf_md(&ctx, EVP_sha256())?;
        openssl_sys_evp_pkey_ctx_set1_hkdf_key(&ctx, pkey.as_ref())?;
        openssl_sys_evp_pkey_ctx_set1_hkdf_salt(&ctx, salt.as_ref())?;
        openssl_sys_evp_pkey_ctx_add1_hdkf_info(&ctx,info.as_ref())?;
    }
    let mut out_len: usize = 32;
    let mut out = [0_u8;32];
    if unsafe {openssl_sys::EVP_PKEY_derive(ctx.0,out.as_mut_ptr(),&mut out_len)} <=0 {
        return Err(HkdfError::DerivationFailed)

    }
    Ok((out,out_len))
}
#[derive(Debug)]
pub struct VaultIntermediateKey{
    hkdf_bytes: [u8;32],
    salt: [u8;32]
}

pub(crate) fn gen_hkdf_intermediate_from_256_bits_entropy(entropy: &[u8; 32],app_name: &str,app_version: &str,purpose: &str) -> Result<VaultIntermediateKey, anyhow::Error> {
    // openssl
    // generate a random salt
    let mut salt_bytes = [0_u8;32];
    openssl::rand::rand_bytes(&mut salt_bytes).context("While attempting to generate a salt")?;
    let key_info = format!("{app_name} | {app_version} | {purpose}");
    let (out,len) = hdkf(entropy, &salt_bytes, key_info.as_bytes())?;
    let key = VaultIntermediateKey{
        hkdf_bytes:out,
        salt:salt_bytes
    };
    Ok(key)
}

#[test]
pub fn vault_crypto_test_intermediate_key() {
    let mut entropy = [0_u8;32];
    openssl::rand::rand_bytes(&mut entropy).expect("Entropy bytes");
    let _key = gen_hkdf_intermediate_from_256_bits_entropy(&entropy, "VAULT", "0.1.0", "SECRETS").unwrap();
    // dbg!(key);
}
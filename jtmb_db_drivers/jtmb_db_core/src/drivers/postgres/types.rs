pub struct Int2([u8; size_of::<i16>()]);

impl PgType for i16 {
    type PgTypeOf = Int2;
    type RustTypeOf = Self;
    const OID_U32: u32 = 21;
    // const OID_BYTES: [u8; size_of::<u32>()] = Self::OID.to_be_bytes();
}

impl std::convert::From<i16> for Int2 {
    fn from(value: i16) -> Self {
        Self(value.to_be_bytes())
    }
}
impl AsRef<[u8]> for Int2 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
impl std::convert::From<Int2> for i16 {
    fn from(value: Int2) -> Self {
        i16::from_be_bytes(value.0)
    }
}

pub struct Int4([u8; size_of::<i32>()]);

impl PgType for i32 {
    type PgTypeOf = Int4;
    type RustTypeOf = Self;
    const OID_U32: u32 = 23;
    // const OID_BYTES: [u8; size_of::<u32>()] = Self::OID.to_be_bytes();
}

impl std::convert::From<i32> for Int4 {
    fn from(value: i32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl AsRef<[u8]> for Int4 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::convert::From<Int4> for i32 {
    fn from(value: Int4) -> Self {
        i32::from_be_bytes(value.0)
    }
}

pub struct Int8([u8; size_of::<i64>()]);

impl PgType for i64 {
    type PgTypeOf = Int8;
    type RustTypeOf = Self;
    const OID_U32: u32 = 23;
    // const OID_BYTES: [u8; size_of::<u32>()] = Self::OID.to_be_bytes();
}
impl AsRef<[u8]> for Int8 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::convert::From<Int8> for i64 {
    fn from(value: Int8) -> i64 {
        // value.0
        i64::from_ne_bytes(value.0)
    }
}
impl std::convert::From<i64> for Int8 {
    fn from(value: i64) -> Self {
        Self(value.to_be_bytes())
    }
}

pub struct Float4([u8; size_of::<f32>()]);

impl PgType for f32 {
    type PgTypeOf = Float4;
    type RustTypeOf = Self;
    const OID_U32: u32 = 23;
    // const OID_BYTES: [u8; size_of::<u32>()] = Self::OID.to_be_bytes();
}

impl AsRef<[u8]> for Float4 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::convert::From<Float4> for f32 {
    fn from(value: Float4) -> Self {
        f32::from_be_bytes(value.0)
    }
}
impl std::convert::From<f32> for Float4 {
    fn from(value: f32) -> Self {
        Self(value.to_be_bytes())
    }
}

impl std::convert::From<Float8> for f64 {
    fn from(value: Float8) -> Self {
        Self::from_be_bytes(value.0)
    }
}

pub trait PgType {
    type PgTypeOf;
    type RustTypeOf;
    const OID_U32: u32;
    const OID_BYTES: [u8; size_of::<u32>()] = Self::OID_U32.to_be_bytes();
    const OID: Oid = Oid(Self::OID_BYTES);
}

pub struct Float8([u8; size_of::<f64>()]);

impl PgType for f64 {
    type PgTypeOf = Float8;
    type RustTypeOf = Self;
    const OID_U32: u32 = 23;
    // const OID_BYTES: [u8; size_of::<u32>()] = Self::OID.to_be_bytes();
}

impl std::convert::From<f64> for Float8 {
    fn from(value: f64) -> Self {
        Self(value.to_be_bytes())
    }
}
impl AsRef<[u8]> for Float8 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub struct Text(Vec<u8>);
impl PgType for Vec<u8> {
    type PgTypeOf = Text;
    type RustTypeOf = Self;
    const OID_U32: u32 = 0;
}
impl<'a> PgType for &'a str {
    type PgTypeOf = Text;
    type RustTypeOf = Self;
    const OID_U32: u32 = 0;
}

impl<'a> std::convert::From<&'a str> for Text {
    fn from(value: &'a str) -> Self {
        Self(value.as_bytes().to_owned())
    }
}
impl AsRef<[u8]> for Text {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl PgType for String {
    type PgTypeOf = Text;
    type RustTypeOf = Self;
    const OID_U32: u32 = 0;
}
impl std::convert::From<String> for Text {
    fn from(value: String) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

pub struct Boolean([u8;1]);

impl<'a> PgType for bool {
    type PgTypeOf = Boolean;
    type RustTypeOf = Self;
    const OID_U32: u32 = 0;
}

impl std::convert::From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean((value as u8).to_be_bytes() )
    }
}
impl AsRef<[u8]> for Boolean {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}


#[derive(Debug, PartialEq)]
pub struct Oid([u8; size_of::<u32>()]);
pub trait OidToType {
    type RustType;
}

pub trait PgOidOf {
    fn pg_oid_of() -> Oid;
}

pub struct Modem {
    query: String,
    blob: Vec<u8>,
    offsets: Vec<i32>,
    sizes: Vec<i32>,
    types: Vec<Oid>,
    offset: i32,
    oid_cursor: usize,
    data_cursor: usize,
}

impl Modem {
    pub fn query(s: &str) -> Self {
        Self {
            query: s.to_string(),
            blob: vec![],
            offset: 0,
            sizes: vec![],
            types: vec![],
            offsets: vec![],
            oid_cursor: 0,
            data_cursor: 0,
        }
    }
    pub fn encode_to<F, T>(f: F) -> T
    where
        T: From<F>,
    {
        T::from(f)
    }
    pub fn bind<'binder, T: PgType>(&mut self, t: T) -> &mut Self
    where
        <T as PgType>::PgTypeOf: From<T> + AsRef<[u8]>,
        // <T as PgType>::PgTypeOf: Into<&'binder [u8]>,
    {
        // push the oid defined on T's PgType impl
        self.types.push(T::OID);
        let pgtype: <T as PgType>::PgTypeOf = <T as PgType>::PgTypeOf::from(t);
        let slice = pgtype.as_ref();
        self.blob.extend_from_slice(&slice);

        // add the size
        let size = std::mem::size_of::<<T as PgType>::PgTypeOf>() as i32;
        self.sizes.push(size);

        self.offsets.push(self.offset);
        self.offset += size;
        // self.count +=1;
        self
    }
    pub fn oids<'slice>(&'slice self) -> &'slice [Oid] {
        self.types.as_slice()
    }
    pub fn arg_count(&self) -> usize {
        self.types.len()
    }
    // you must know EXACTLY what type this is.
    // if you get it wrong, it WILL NOT WORK
    // MAKE SURE you map the OID -> RUst type
    // you must check the length or count or this will die.
    //
    pub unsafe fn driver_extract<'slice, P: PgType>(&'slice mut self) -> &'slice [u8] {
        // let cursor = 0;
        let oid = unsafe { self.types.get_unchecked(self.oid_cursor) };
        #[cfg(debug_assertions)]
        {
            if P::OID != *oid {
                panic!("Runtime/Comptime OID Mismatch")
            }
        }
        self.oid_cursor += 1;

        let size = std::mem::size_of::<P::PgTypeOf>();
        let slice = &self.blob[self.data_cursor..self.data_cursor + size];
        self.data_cursor += size;
        slice
    }
    pub fn execute(self){ }

    pub fn reset_cursors(&mut self) {
        self.data_cursor = 0;
        self.oid_cursor = 0;
        self.offset = 0;
    }
}

#[cfg(test)]
pub fn test_modem() {
    let q = Modem::query("Select * from users where id = $1")
        .bind(0_i32)
        .bind(0_i16)
        .bind(0_i64)
        .bind::<f32>(0.0f32)
        .bind(0.0f64)
        .bind("hello world")
        .bind("hello_world".to_string())
        .bind(true);
}

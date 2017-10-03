use std::io::Write;

use byteorder::WriteBytesExt;
use serde::ser;
use serde::ser::Serialize;

use super::errors::{Error, Result};

/// Serializer for the XDR format.
///
/// Structure that holds a mutable borrow of the writer it serializes data to.
/// It has an implementation of [`serde::Serializer`][serde::Serializer] so that
/// it can serialize data into its XDR representation.
///
/// [serde::Serializer]: ../serde/ser/trait.Serializer.html
pub struct Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    writer: &'w mut W
}

impl<'w, W> Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    /// Create a new instance that serializes data into the given generic
    /// writer.
    pub fn new(writer: &'w mut W) -> Self {
        Serializer { writer }
    }
}

mod serializer;

impl<'w, W> ser::SerializeSeq for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("seq is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("seq is not supported")
    }
}

impl<'w, W> ser::SerializeTuple for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple is not supported")
    }
}

impl<'w, W> ser::SerializeTupleStruct for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple_struct is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple_struct is not supported")
    }
}

impl<'w, W> ser::SerializeTupleVariant for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("tuple_struct is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("tuple_variant is not supported")
    }
}

impl<'w, W> ser::SerializeMap for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("map is not supported")
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("map is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("map is not supported")
    }
}

impl<'w, W> ser::SerializeStructVariant for Serializer<'w, W>
where
    W: WriteBytesExt + 'w,
{
    type Ok = Self;
    type Error = Error;

    fn serialize_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("struct_variant is not supported")
    }

    fn end(self) -> Result<Self> {
        unreachable!("struct_variant is not supported")
    }
}

/// Serialize data into a vector of bytes.
///
/// Serializes a generic data type into a new instance of `Vec<u8>`.
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut bytes = Vec::new();

    value.serialize(Serializer::new(&mut bytes))?;

    Ok(bytes)
}

/// Serialize data through a generic writer.
///
/// Serializes a generic data type through a borrowed instance that implements
/// `Write`.
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    value.serialize(Serializer::new(writer))?;

    Ok(())
}

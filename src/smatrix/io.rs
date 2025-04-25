//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::Field;
//}}}
//{{{ std imports
use std::fmt;
use std::marker::PhantomData;
//}}}
//{{{ dep imports
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: Serialization/Deserializaton
//{{{ impl Serialize for SMatrix
impl<T, const N: usize, const M: usize> Serialize for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Serialize,
    [T; N * M]: Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SMatrix", 3)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("nrows", &self.nrows)?;
        state.serialize_field("ncols", &self.ncols)?;
        state.end()
    }
}
//}}}
//{{{ impl Deserialize for SMatrix

impl<'de, T, const N: usize, const M: usize> Deserialize<'de> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Deserialize<'de>,
    [T; N * M]: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum DeField
        {
            Data,
            Nrows,
            Ncols,
        }

        struct SMatrixVisitor<T, const N: usize, const M: usize>(PhantomData<T>);

        impl<'de, T, const N: usize, const M: usize> Visitor<'de> for SMatrixVisitor<T, N, M>
        where
            [(); N * M]:,
            T: Field + Default + Copy + fmt::Display + Deserialize<'de>,
            [T; N * M]: Deserialize<'de>,
        {
            type Value = SMatrix<T, N, M>;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result
            {
                formatter.write_str("struct SMatrix")
            }

            fn visit_map<V>(
                self,
                mut map: V,
            ) -> Result<SMatrix<T, N, M>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut data: Option<[T; N * M]> = None;
                let mut nrows: Option<usize> = None;
                let mut ncols: Option<usize> = None;

                while let Some(key) = map.next_key()?
                {
                    match key
                    {
                        DeField::Data =>
                        {
                            if data.is_some()
                            {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        DeField::Nrows =>
                        {
                            if nrows.is_some()
                            {
                                return Err(de::Error::duplicate_field("nrows"));
                            }
                            nrows = Some(map.next_value()?);
                        }
                        DeField::Ncols =>
                        {
                            if ncols.is_some()
                            {
                                return Err(de::Error::duplicate_field("ncols"));
                            }
                            ncols = Some(map.next_value()?);
                        }
                    }
                }

                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let nrows = nrows.ok_or_else(|| de::Error::missing_field("nrows"))?;
                let ncols = ncols.ok_or_else(|| de::Error::missing_field("ncols"))?;

                Ok(SMatrix { data, nrows, ncols })
            }
        }

        impl<'de> Deserialize<'de> for DeField
        {
            fn deserialize<D>(deserializer: D) -> Result<DeField, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor
                {
                    type Value = DeField;

                    fn expecting(
                        &self,
                        formatter: &mut fmt::Formatter,
                    ) -> fmt::Result
                    {
                        formatter.write_str("`data` or `nrows` or `ncols`")
                    }

                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> Result<DeField, E>
                    where
                        E: de::Error,
                    {
                        match value
                        {
                            "data" => Ok(DeField::Data),
                            "nrows" => Ok(DeField::Nrows),
                            "ncols" => Ok(DeField::Ncols),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        const FIELDS: &[&str] = &["data", "nrows", "ncols"];
        deserializer.deserialize_struct("SMatrix", FIELDS, SMatrixVisitor(PhantomData))
    }
}
//}}}
//}}}
//{{{ impl fmt::Display for SMatrix
impl<T, const N: usize, const M: usize> fmt::Display for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result
    {
        let max_width = self
            .data
            .iter()
            .map(|x| format!("{x}").len())
            .max()
            .unwrap_or(0);

        for i in 0..N
        {
            write!(f, "|")?;
            for j in 0..M
            {
                write!(f, " {:>width$}", self[(i, j)], width = max_width)?;
            }
            writeln!(f, " |")?;
        }

        Ok(())
    }
}

//}}}

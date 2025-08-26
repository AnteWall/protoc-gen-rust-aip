// @generated
impl serde::Serialize for TestResource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.optional_field.is_some() {
            len += 1;
        }
        if self.optional_number.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("example.optional.v1.TestResource", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.optional_field.as_ref() {
            struct_ser.serialize_field("optionalField", v)?;
        }
        if let Some(v) = self.optional_number.as_ref() {
            struct_ser.serialize_field("optionalNumber", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestResource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "optional_field",
            "optionalField",
            "optional_number",
            "optionalNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            OptionalField,
            OptionalNumber,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "optionalField" | "optional_field" => Ok(GeneratedField::OptionalField),
                            "optionalNumber" | "optional_number" => Ok(GeneratedField::OptionalNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestResource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct example.optional.v1.TestResource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TestResource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut optional_field__ = None;
                let mut optional_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::OptionalField => {
                            if optional_field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalField"));
                            }
                            optional_field__ = map.next_value()?;
                        }
                        GeneratedField::OptionalNumber => {
                            if optional_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("optionalNumber"));
                            }
                            optional_number__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(TestResource {
                    name: name__.unwrap_or_default(),
                    optional_field: optional_field__,
                    optional_number: optional_number__,
                })
            }
        }
        deserializer.deserialize_struct("example.optional.v1.TestResource", FIELDS, GeneratedVisitor)
    }
}

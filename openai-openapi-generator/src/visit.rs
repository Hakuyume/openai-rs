use crate::openapi;

macro_rules! iter {
    ($name:ident, $ty:ty, $as:ident, $iter:ident) => {
        pub fn $name(schema: $ty) -> impl Iterator<Item = (String, $ty)> {
            schema
                .additional_properties
                .$iter()
                .flat_map(IterExt::$iter)
                .map(|additional_properties| {
                    ("additionalProperties".to_owned(), additional_properties)
                })
                .chain(
                    schema
                        .all_of
                        .$iter()
                        .flatten()
                        .enumerate()
                        .map(|(i, all_of)| (format!("allOf[{i}]"), all_of)),
                )
                .chain(
                    schema
                        .any_of
                        .$iter()
                        .flatten()
                        .enumerate()
                        .map(|(i, any_of)| (format!("anyOf[{i}]"), any_of)),
                )
                .chain(
                    schema
                        .items
                        .$iter()
                        .map(|items| ("items".to_owned(), items.$as())),
                )
                .chain(
                    schema
                        .one_of
                        .$iter()
                        .flatten()
                        .enumerate()
                        .map(|(i, one_of)| (format!("oneOf[{i}]"), one_of)),
                )
                .chain(
                    schema
                        .properties
                        .$iter()
                        .flatten()
                        .map(|(property_name, property)| {
                            (format!("properties[{property_name:?}]"), property)
                        }),
                )
        }
    };
}
iter!(iter, &openapi::Schema, as_ref, iter);
iter!(iter_mut, &mut openapi::Schema, as_mut, iter_mut);

trait IterExt {
    fn iter(&self) -> impl Iterator<Item = &openapi::Schema>;
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut openapi::Schema>;
}

impl IterExt for openapi::AdditionalProperties {
    fn iter(&self) -> impl Iterator<Item = &openapi::Schema> {
        if let openapi::AdditionalProperties::Schema(additional_properties) = self {
            Some(additional_properties.as_ref())
        } else {
            None
        }
        .into_iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut openapi::Schema> {
        if let openapi::AdditionalProperties::Schema(additional_properties) = self {
            Some(additional_properties.as_mut())
        } else {
            None
        }
        .into_iter()
    }
}

use crate::openapi;

macro_rules! iter {
    ($name:ident, $ty:ty, $as:ident) => {
        pub fn $name(schema: $ty) -> impl Iterator<Item = (String, $ty)> {
            if let openapi::Schema {
                additional_properties:
                    Some(openapi::AdditionalProperties::Schema(additional_properties)),
                ..
            } = schema
            {
                Some(additional_properties.$as())
            } else {
                None
            }
            .into_iter()
            .map(|additional_properties| ("additionalProperties".to_owned(), additional_properties))
            .chain(
                schema
                    .all_of
                    .$as()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(i, all_of)| (format!("allOf[{i}]"), all_of)),
            )
            .chain(
                schema
                    .any_of
                    .$as()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(i, any_of)| (format!("anyOf[{i}]"), any_of)),
            )
            .chain(
                schema
                    .items
                    .$as()
                    .into_iter()
                    .map(|items| ("items".to_owned(), items.$as())),
            )
            .chain(
                schema
                    .one_of
                    .$as()
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(i, one_of)| (format!("oneOf[{i}]"), one_of)),
            )
            .chain(schema.properties.$as().into_iter().flatten().map(
                |(property_name, property)| (format!("properties[{property_name:?}]"), property),
            ))
        }
    };
}
// iter!(iter, &openapi::Schema, as_ref);
iter!(iter_mut, &mut openapi::Schema, as_mut);

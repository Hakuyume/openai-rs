use crate::openapi;

pub fn iter(schema: &openapi::Schema) -> impl Iterator<Item = (String, &openapi::Schema)> {
    if let Some(openapi::AdditionalProperties::Schema(additional_properties)) =
        &schema.additional_properties
    {
        Some((
            "additionalProperties".to_owned(),
            additional_properties.as_ref(),
        ))
    } else {
        None
    }
    .into_iter()
    .chain(
        schema
            .all_of
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, all_of)| (format!("allOf[{i}]"), all_of)),
    )
    .chain(
        schema
            .any_of
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, any_of)| (format!("anyOf[{i}]"), any_of)),
    )
    .chain(
        schema
            .items
            .iter()
            .map(|items| ("items".to_owned(), items.as_ref())),
    )
    .chain(
        schema
            .one_of
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, one_of)| (format!("oneOf[{i}]"), one_of)),
    )
    .chain(
        schema
            .properties
            .iter()
            .flatten()
            .map(|(property_name, property)| (format!("properties[{property_name:?}]"), property)),
    )
}

pub fn iter_mut(
    schema: &mut openapi::Schema,
) -> impl Iterator<Item = (String, &mut openapi::Schema)> {
    if let Some(openapi::AdditionalProperties::Schema(additional_properties)) =
        &mut schema.additional_properties
    {
        Some((
            "additionalProperties".to_owned(),
            additional_properties.as_mut(),
        ))
    } else {
        None
    }
    .into_iter()
    .chain(
        schema
            .all_of
            .iter_mut()
            .flatten()
            .enumerate()
            .map(|(i, all_of)| (format!("allOf[{i}]"), all_of)),
    )
    .chain(
        schema
            .any_of
            .iter_mut()
            .flatten()
            .enumerate()
            .map(|(i, any_of)| (format!("anyOf[{i}]"), any_of)),
    )
    .chain(
        schema
            .items
            .iter_mut()
            .map(|items| ("items".to_owned(), items.as_mut())),
    )
    .chain(
        schema
            .one_of
            .iter_mut()
            .flatten()
            .enumerate()
            .map(|(i, one_of)| (format!("oneOf[{i}]"), one_of)),
    )
    .chain(
        schema
            .properties
            .iter_mut()
            .flatten()
            .map(|(property_name, property)| (format!("properties[{property_name:?}]"), property)),
    )
}

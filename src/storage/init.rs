use tarantool::{
    index::{IndexOptions, Part},
    space::{Field, Space, SpaceCreateOptions, SpaceEngineType},
};

use crate::errors::MeteoError;

pub fn create_meteo_space() -> Result<(), MeteoError> {
    let space = Space::create(
        "meteo",
        &SpaceCreateOptions {
            if_not_exists: true,
            engine: SpaceEngineType::Memtx,
            format: Some(vec![
                Field::string("key"),
                Field::unsigned("bucket_id"),
                Field::string("value"),
                Field::unsigned("expires_at"),
            ]),
            ..SpaceCreateOptions::default()
        },
    )
    .map_err(|err| MeteoError::StorageError(format!("unable to create `meteo` space: {err:?}")))?;

    space
        .create_index(
            "primary",
            &IndexOptions {
                parts: Some(vec![Part::field("key")]),
                if_not_exists: Some(true),
                ..IndexOptions::default()
            },
        )
        .map_err(|err| {
            MeteoError::StorageError(format!(
                "unable to create `primary` index on `meteo` space: {err:?}"
            ))
        })?;

    Ok(())
}

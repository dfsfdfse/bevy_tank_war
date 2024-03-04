use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use serde::Deserialize;
use serde_ron::de::from_bytes;
use std::marker::PhantomData;
use thiserror::Error;
#[derive(Default)]
pub struct RonAssetPlugin<A> {
    _marker: PhantomData<A>,
}

struct RonAssetLoader<A> {
    _marker: PhantomData<A>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON Error](serde_ron::error::SpannedError)
    #[error("Could not parse RON: {0}")]
    RonError(#[from] serde_ron::error::SpannedError),
}

impl<A> AssetLoader for RonAssetLoader<A>
where
    for<'de> A: Asset + Deserialize<'de>,
{
    type Asset = A;
    type Settings = ();
    type Error = RonLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = from_bytes::<A>(&bytes)?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

impl<A> Plugin for RonAssetPlugin<A>
where
    for<'de> A: Asset + Deserialize<'de>,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<A>()
            .register_asset_loader(RonAssetLoader::<A> {
                _marker: PhantomData,
            });
    }
}

use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::client::{ApiKeySet, PayloadSet, StraicoClient, StraicoRequestBuilder};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ImageData {
    zip: String,
    images: Vec<String>,
    price: Price,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Price {
    price_per_image: u16,
    quantity_images: u8,
    total: u16,
}

#[derive(Serialize)]
pub struct ImageRequest {
    model: String,
    description: String,
    size: String,
    variations: u8,
}

#[allow(dead_code)]
pub struct ImageRequestBuilder<T, U, V, W> {
    model: T,
    description: U,
    size: V,
    variations: W,
}

pub struct ModelNotSet;
pub struct DescriptionNotSet;
pub struct SizeNotSet;
pub struct VariationsNotSet;
pub struct ModelSet(String);
pub struct DescriptionSet(String);
pub struct SizeSet(String);
pub struct VariationsSet(u8);

impl ImageRequest {
    pub fn new() -> ImageRequestBuilder<ModelNotSet, DescriptionNotSet, SizeNotSet, VariationsNotSet>
    {
        ImageRequestBuilder {
            model: ModelNotSet,
            description: DescriptionNotSet,
            size: SizeNotSet,
            variations: VariationsNotSet,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<ModelNotSet, T, U, V> {
    pub fn model(self, model: &str) -> ImageRequestBuilder<ModelSet, T, U, V> {
        ImageRequestBuilder {
            model: ModelSet(model.into()),
            description: self.description,
            size: self.size,
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, DescriptionNotSet, U, V> {
    pub fn description(self, description: &str) -> ImageRequestBuilder<T, DescriptionSet, U, V> {
        ImageRequestBuilder {
            model: self.model,
            description: DescriptionSet(description.into()),
            size: self.size,
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, U, SizeNotSet, V> {
    pub fn size(self, size: &str) -> ImageRequestBuilder<T, U, SizeSet, V> {
        ImageRequestBuilder {
            model: self.model,
            description: self.description,
            size: SizeSet(size.into()),
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, U, V, VariationsNotSet> {
    pub fn variations(self, variations: u8) -> ImageRequestBuilder<T, U, V, VariationsSet> {
        ImageRequestBuilder {
            model: self.model,
            description: self.description,
            size: self.size,
            variations: VariationsSet(variations),
        }
    }
}

impl ImageRequestBuilder<ModelSet, DescriptionSet, SizeSet, VariationsSet> {
    pub fn build(self) -> ImageRequest {
        ImageRequest {
            model: self.model.0,
            description: self.description.0,
            size: self.size.0,
            variations: self.variations.0,
        }
    }

    pub fn bearer_auth<T: Display>(
        self,
        key: T,
    ) -> StraicoRequestBuilder<ApiKeySet, PayloadSet, ImageData> {
        let payload = self.build();
        let client = StraicoClient::new();
        client.image().json(payload).bearer_auth(key)
    }
}

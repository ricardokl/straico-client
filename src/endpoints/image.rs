#![allow(dead_code)]
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::client::{ApiKeySet, IntoStraicoClient, PayloadSet, StraicoRequestBuilder};

#[derive(Deserialize)]
pub struct ImageData {
    pub zip: String,
    pub images: Vec<String>,
    pub price: Price,
}

#[derive(Deserialize)]
pub struct Price {
    pub price_per_image: u16,
    pub quantity_images: u8,
    pub total: u16,
}

#[derive(Debug, Serialize)]
pub struct ImageRequest {
    pub model: String,
    pub description: String,
    pub size: String,
    pub variations: u8,
}

struct ImageRequestBuilder<T, U, V, W> {
    model: T,
    description: U,
    size: V,
    variations: W,
}

struct ModelNotSet;
struct DescriptionNotSet;
struct SizeNotSet;
struct VariationsNotSet;
struct ModelSet(String);
struct DescriptionSet(String);
struct SizeSet(String);
struct VariationsSet(u8);

impl ImageRequest {
    fn new() -> ImageRequestBuilder<ModelNotSet, DescriptionNotSet, SizeNotSet, VariationsNotSet> {
        ImageRequestBuilder {
            model: ModelNotSet,
            description: DescriptionNotSet,
            size: SizeNotSet,
            variations: VariationsNotSet,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<ModelNotSet, T, U, V> {
    fn model(self, model: &str) -> ImageRequestBuilder<ModelSet, T, U, V> {
        ImageRequestBuilder {
            model: ModelSet(model.into()),
            description: self.description,
            size: self.size,
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, DescriptionNotSet, U, V> {
    fn description(self, description: &str) -> ImageRequestBuilder<T, DescriptionSet, U, V> {
        ImageRequestBuilder {
            model: self.model,
            description: DescriptionSet(description.into()),
            size: self.size,
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, U, SizeNotSet, V> {
    fn size(self, size: &str) -> ImageRequestBuilder<T, U, SizeSet, V> {
        ImageRequestBuilder {
            model: self.model,
            description: self.description,
            size: SizeSet(size.into()),
            variations: self.variations,
        }
    }
}

impl<T, U, V> ImageRequestBuilder<T, U, V, VariationsNotSet> {
    fn variations(self, variations: u8) -> ImageRequestBuilder<T, U, V, VariationsSet> {
        ImageRequestBuilder {
            model: self.model,
            description: self.description,
            size: self.size,
            variations: VariationsSet(variations),
        }
    }
}

impl ImageRequestBuilder<ModelSet, DescriptionSet, SizeSet, VariationsSet> {
    fn build(self) -> ImageRequest {
        ImageRequest {
            model: self.model.0,
            description: self.description.0,
            size: self.size.0,
            variations: self.variations.0,
        }
    }

    fn bearer_auth<T: Display>(
        self,
        key: T,
    ) -> StraicoRequestBuilder<ApiKeySet, PayloadSet<ImageRequest>> {
        let payload = self.build();
        ReqwestClient::new()
            .to_straico()
            .image()
            .json(&payload)
            .bearer_auth(key)
    }
}

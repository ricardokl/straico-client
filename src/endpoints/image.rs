use serde::{Deserialize, Serialize};

/// A structure representing data about an image generation request response.
///
/// # Fields
///
/// * `zip` - The URL or path to a ZIP file containing the generated images
/// * `images` - A vector of URLs or paths to individual generated images
/// * `price` - Pricing information including cost per image and totals
#[derive(Deserialize, Debug)]
pub struct ImageData {
    zip: String,
    images: Vec<String>,
    price: Price,
}

/// A structure representing pricing information for an image generation request.
///
/// # Fields
///
/// * `price_per_image` - The cost per individual generated image in credits/points
/// * `quantity_images` - The number of images generated in this request
/// * `total` - The total cost for all images in this request
#[derive(Deserialize, Debug)]
pub struct Price {
    price_per_image: u16,
    quantity_images: u8,
    total: u16,
}

/// A structure representing an image generation request.
///
/// # Fields
///
/// * `model` - The AI model identifier to use for image generation
/// * `description` - The text description/prompt for the desired image
/// * `size` - The desired output image dimensions (e.g. "1024x1024")
/// * `variations` - Number of image variations to generate
#[derive(Serialize)]
pub struct ImageRequest {
    model: String,
    description: String,
    size: String,
    variations: u8,
}

/// A builder pattern implementation for constructing `ImageRequest` objects.
///
/// # Type Parameters
///
/// * `T` - Type state for the model field (either `ModelNotSet` or `ModelSet`)
/// * `U` - Type state for the description field (either `DescriptionNotSet` or `DescriptionSet`)
/// * `V` - Type state for the size field (either `SizeNotSet` or `SizeSet`)
/// * `W` - Type state for the variations field (either `VariationsNotSet` or `VariationsSet`)
///
/// # Fields
///
/// * `model` - The AI model identifier state
/// * `description` - The image description/prompt state
/// * `size` - The image dimensions state
/// * `variations` - The number of variations state
#[derive(Default)]
pub struct ImageRequestBuilder<T, U, V, W> {
    model: T,
    description: U,
    size: V,
    variations: W,
}

/// Zero-sized type indicating that the model field has not been set in the builder
#[derive(Default)]
pub struct ModelNotSet;
/// Zero-sized type indicating that the description field has not been set in the builder
#[derive(Default)]
pub struct DescriptionNotSet;
/// Zero-sized type indicating that the size field has not been set in the builder
#[derive(Default)]
pub struct SizeNotSet;
/// Zero-sized type indicating that the variations field has not been set in the builder
#[derive(Default)]
pub struct VariationsNotSet;
/// Type indicating that the model field has been set in the builder, containing the model string
pub struct ModelSet(String);
/// Type indicating that the description field has been set in the builder, containing the description string
pub struct DescriptionSet(String);
/// Type indicating that the size field has been set in the builder, containing the size string
pub struct SizeSet(String);
/// Type indicating that the variations field has been set in the builder, containing the variations count
pub struct VariationsSet(u8);

impl ImageRequest {
    /// Creates a new `ImageRequestBuilder` with all type states set to their "not set" variants.
    ///
    /// This is the entry point for constructing an `ImageRequest` using the builder pattern.
    /// Each field must be set using the corresponding builder methods before calling `build()`.
    ///
    /// # Returns
    ///
    /// A new `ImageRequestBuilder` instance with no fields set.
    pub fn new() -> ImageRequestBuilder<ModelNotSet, DescriptionNotSet, SizeNotSet, VariationsNotSet>
    {
        ImageRequestBuilder::default()
    }
}

impl<T, U, V> ImageRequestBuilder<ModelNotSet, T, U, V> {
    /// Sets the AI model identifier for the image generation request.
    ///
    /// This method transitions the builder from having no model set to having a model set,
    /// while preserving the state of all other fields.
    ///
    /// # Arguments
    ///
    /// * `model` - A string slice containing the identifier for the AI model to use
    ///
    /// # Returns
    ///
    /// A new `ImageRequestBuilder` with the model field set and all other fields preserved
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
    /// Sets the text description/prompt for the image generation request.
    ///
    /// This method transitions the builder from having no description set to having a description set,
    /// while preserving the state of all other fields.
    ///
    /// # Arguments
    ///
    /// * `description` - A string slice containing the text prompt for the desired image
    ///
    /// # Returns
    ///
    /// A new `ImageRequestBuilder` with the description field set and all other fields preserved
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
    /// Sets the image dimensions for the image generation request.
    ///
    /// This method transitions the builder from having no size set to having a size set,
    /// while preserving the state of all other fields.
    ///
    /// # Arguments
    ///
    /// * `size` - A string slice containing the image dimensions (e.g. "1024x1024")
    ///
    /// # Returns
    ///
    /// A new `ImageRequestBuilder` with the size field set and all other fields preserved
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
    /// Sets the number of image variations for the image generation request.
    ///
    /// This method transitions the builder from having no variations set to having variations set,
    /// while preserving the state of all other fields.
    ///
    /// # Arguments
    ///
    /// * `variations` - The number of image variations to generate
    ///
    /// # Returns
    ///
    /// A new `ImageRequestBuilder` with the variations field set and all other fields preserved
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
    /// Constructs an `ImageRequest` from the builder once all fields have been set.
    ///
    /// This method can only be called on an `ImageRequestBuilder` instance where all type states
    /// indicate their respective fields have been set (ModelSet, DescriptionSet, SizeSet, VariationsSet).
    ///
    /// # Returns
    ///
    /// A fully constructed `ImageRequest` containing all the field values that were set during building.
    pub fn build(self) -> ImageRequest {
        ImageRequest {
            model: self.model.0,
            description: self.description.0,
            size: self.size.0,
            variations: self.variations.0,
        }
    }
}

impl ImageRequest {
    /// Returns a reference to the AI model identifier string.
    ///
    /// This getter method provides read-only access to the model field,
    /// which contains the identifier for the AI model selected for image generation.
    ///
    /// # Returns
    ///
    /// A reference to the String containing the model identifier
    pub fn model(&self) -> &String {
        &self.model
    }

    /// Returns a reference to the image description/prompt string.
    ///
    /// This getter method provides read-only access to the description field,
    /// which contains the text prompt used to generate the image.
    ///
    /// # Returns
    ///
    /// A reference to the String containing the image description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Returns a reference to the image dimensions string.
    ///
    /// This getter method provides read-only access to the size field,
    /// which contains the desired output dimensions for the generated image.
    ///
    /// # Returns
    ///
    /// A reference to the String containing the image dimensions
    pub fn size(&self) -> &String {
        &self.size
    }

    /// Returns the number of image variations requested.
    ///
    /// This getter method provides the number of different variations
    /// that were requested to be generated from the same prompt.
    ///
    /// # Returns
    ///
    /// The number of image variations as a u8
    pub fn variations(&self) -> u8 {
        self.variations
    }
}

impl ImageData {
    /// Returns a reference to the ZIP file URL/path.
    ///
    /// This getter method provides read-only access to the zip field,
    /// which contains the URL or path to the ZIP file containing all generated images.
    ///
    /// # Returns
    ///
    /// A reference to the String containing the ZIP file location
    pub fn zip(&self) -> &String {
        &self.zip
    }

    /// Returns a reference to the vector of image URLs/paths.
    ///
    /// This getter method provides read-only access to the images field,
    /// which contains URLs or paths to each individually generated image.
    ///
    /// # Returns
    ///
    /// A reference to the Vector of Strings containing image locations
    pub fn images(&self) -> &Vec<String> {
        &self.images
    }

    /// Returns a reference to the pricing information.
    ///
    /// This getter method provides read-only access to the price field,
    /// which contains details about costs per image and totals.
    ///
    /// # Returns
    ///
    /// A reference to the Price struct containing pricing details
    pub fn price(&self) -> &Price {
        &self.price
    }
}

impl Price {
    /// Returns the price per image in credits/points.
    ///
    /// This getter method provides access to the price charged for
    /// each individual generated image.
    ///
    /// # Returns
    ///
    /// The price per image as a u16
    pub fn price_per_image(&self) -> u16 {
        self.price_per_image
    }

    /// Returns the number of images that were generated.
    ///
    /// This getter method provides access to the quantity of
    /// images that were generated in this request.
    ///
    /// # Returns
    ///
    /// The number of images as a u8
    pub fn quantity_images(&self) -> u8 {
        self.quantity_images
    }

    /// Returns the total price for all generated images.
    ///
    /// This getter method provides access to the total cost
    /// in credits/points for all images in this request.
    ///
    /// # Returns
    ///
    /// The total price as a u16
    pub fn total(&self) -> u16 {
        self.total
    }
}

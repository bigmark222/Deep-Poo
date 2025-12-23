//! Minimal anchor-free detection head for Burn integration.
//!
//! Shapes (normalized coords):
//! - Input images: `[B, 3, H, W]`
//! - Objectness logits: `[B, 1, H, W]`
//! - Box offsets: `[B, 4, H, W]` (x_min, y_min, x_max, y_max in 0..1)

#![allow(unused)] // Gated behind `burn_runtime`.

use burn::module::{Ignored, Module};
use burn::nn::PaddingConfig2d;
use burn::nn::conv::{Conv2d, Conv2dConfig};
use burn::nn::loss::Reduction;
use burn::tensor::{Tensor, activation::sigmoid, backend::Backend};

/// Configuration for the detection head.
#[derive(Debug, Clone, Copy)]
pub struct TinyDetConfig {
    pub max_boxes: usize,
}

impl Default for TinyDetConfig {
    fn default() -> Self {
        Self { max_boxes: 16 }
    }
}

#[derive(Module, Debug)]
pub struct TinyDet<B: Backend> {
    stem: Conv2d<B>,
    head_obj: Conv2d<B>,
    head_box: Conv2d<B>,
    pub config: Ignored<TinyDetConfig>,
}

impl<B: Backend> TinyDet<B> {
    pub fn new(config: TinyDetConfig, device: &B::Device) -> Self {
        let stem = Conv2dConfig::new([3, 32], [3, 3])
            .with_padding(PaddingConfig2d::Same)
            .init(device);
        let head_obj = Conv2dConfig::new([32, 1], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);
        let head_box = Conv2dConfig::new([32, 4], [1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .init(device);

        Self {
            stem,
            head_obj,
            head_box,
            config: Ignored(config),
        }
    }

    /// Forward pass returning (objectness_logits, box_offsets).
    pub fn forward(&self, input: Tensor<B, 4>) -> (Tensor<B, 4>, Tensor<B, 4>) {
        let x = burn::tensor::activation::relu(self.stem.forward(input));
        let obj_logits = self.head_obj.forward(x.clone());
        let box_logits = self.head_box.forward(x);
        (obj_logits, box_logits)
    }

    /// Compute detection loss (objectness BCE + Huber boxes, masked).
    pub fn loss(
        &self,
        obj_logits: Tensor<B, 4>,
        box_preds: Tensor<B, 4>,
        target_obj: Tensor<B, 4>,
        target_boxes: Tensor<B, 4>,
        box_mask: Tensor<B, 4>,
        device: &B::Device,
    ) -> Tensor<B, 1> {
        // Objectness: BCE with logits.
        let one = Tensor::from_floats([1.0], device);
        let eps = Tensor::from_floats([1e-6], device);
        let prob = sigmoid(obj_logits.clone());
        let obj_loss = (target_obj.clone() * (prob.clone() + eps.clone()).log()
            + (one.clone() - target_obj.clone()) * (one.clone() - prob + eps.clone()).log())
        .neg()
        .mean();

        // Box regression: Huber masked by presence.
        let diff = (box_preds - target_boxes).abs();
        let delta = Tensor::from_floats([1.0], device);
        let mask_small = diff.clone().lower(delta.clone());
        let pow_two = Tensor::from_floats([2.0], device);
        let small = Tensor::from_floats([0.5], device) * diff.clone().powf(pow_two);
        let large = diff - Tensor::from_floats([0.5], device);
        let mask_float = mask_small.clone().float();
        let per_elem =
            small * mask_float.clone() + large * (Tensor::from_floats([1.0], device) - mask_float);

        let masked = per_elem * box_mask.clone();
        let denom = box_mask.sum() + Tensor::from_floats([1e-6], device);
        let box_loss = masked.sum() / denom;

        obj_loss + box_loss
    }
}

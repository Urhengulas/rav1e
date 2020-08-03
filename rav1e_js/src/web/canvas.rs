// Copyright (c) 2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use wasm_bindgen::JsCast;
use web_sys;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement};

use crate::web;
use crate::web::{Dimensions, Image, PixelData, Video};

#[derive(Debug)]
pub struct Canvas {
  html: HtmlCanvasElement,
  context: CanvasRenderingContext2d,
  dim: Dimensions,
}

impl Canvas {
  pub fn new(width: u32, height: u32) -> Self {
    let html = web::document()
      .create_element("canvas")
      .unwrap()
      .dyn_into::<HtmlCanvasElement>()
      .map_err(|e: Element| {
        panic!("Err while casting document.createElement(\"canvas\") to HtmlCanvasElement: {:?}", e)
      })
      .unwrap();
    html.set_width(width);
    html.set_height(height);

    let context = Self::create_context(&html);
    Self { html, context, dim: Dimensions { width, height } }
  }

  fn create_context(html: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    html
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap()
  }

  /// Draw an `HtmlImageElement` onto the canvas
  ///
  /// ## Read more
  /// * [`CanvasRenderingContext2D.drawImage()`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
  pub fn draw_image(&self, img: &Image) {
    // The x- and y-axis coordinates in the destination canvas at which to place the top-left
    // corner of the source image.
    let dx = 0.0;
    let dy = 0.0;

    self
      .context
      .draw_image_with_html_image_element(&img.html, dx, dy)
      .expect("failed to draw image onto HtmlImageElement");
  }

  /// Draw an `HtmlImageElement` onto the canvas
  ///
  /// ## Read more
  /// * [`CanvasRenderingContext2D.drawImage()`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)
  pub fn draw_video_frame(&self, video: &Video) {
    // The x- and y-axis coordinates in the destination canvas at which to place the top-left
    // corner of the source image.
    let dx = 0.0;
    let dy = 0.0;

    self
      .context
      .draw_image_with_html_video_element(&video.html, dx, dy)
      .expect("failed to draw video frame onto HtmlCanvasElement");
  }

  /// Get `CanvasRenderingContext2d.ImageData.data`
  pub fn pixel_data(&self) -> PixelData {
    let data = self
      .context
      .get_image_data(0.0, 0.0, self.dim.width as f64, self.dim.height as f64)
      .unwrap()
      .data()
      .0;

    PixelData::new(data, self.dim)
  }
}

impl From<&HtmlCanvasElement> for Canvas {
  fn from(html: &HtmlCanvasElement) -> Self {
    Self {
      html: html.clone(),
      context: Self::create_context(html),
      dim: Dimensions { width: html.width(), height: html.height() },
    }
  }
}

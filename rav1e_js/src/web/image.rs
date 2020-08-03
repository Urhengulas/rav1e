// Copyright (c) 2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use web_sys::HtmlImageElement;

use crate::web::Dimensions;

#[derive(Debug)]
pub struct Image {
  pub html: HtmlImageElement,
  pub dim: Dimensions,
}

impl Image {
  pub fn new(html: &HtmlImageElement) -> Self {
    Self {
      html: html.clone(),
      dim: Dimensions { width: html.width(), height: html.height() },
    }
  }
}

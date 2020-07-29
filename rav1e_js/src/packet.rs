// Copyright (c) 2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use wasm_bindgen::prelude::*;

use rav1e::prelude::Packet as Rav1ePacket;

/// Wrapper around `rav1e::api::util::Packet<u16>`.
///
/// A packet contains one shown frame together with zero or more additional frames.
#[wasm_bindgen]
pub struct Packet {
  pub(crate) p: Rav1ePacket<u16>,
}

#[wasm_bindgen]
impl Packet {
//   pub fn display(&self) -> String {
//     format!("{}", self.p)
//   }
//   pub fn debug(&self) -> String {
//     format!("{:?}", self.p)
//   }
}

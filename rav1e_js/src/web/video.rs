// Copyright (c) 2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlVideoElement};

use crate::web::Dimensions;

#[derive(Debug)]
pub struct Video {
  pub html: HtmlVideoElement,
  pub dim: Dimensions,
}

impl Video {
  pub fn new(html: &HtmlVideoElement) -> Self {
    Self {
      html: html.clone(),
      dim: Dimensions { width: html.video_width(), height: html.video_height() },
    }
  }

  /// Set up a function that will be called whenever the specified `event` is delivered to the target
  ///
  /// [EventTarget.addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)
  pub fn add_event_listener(
    &self, event_name: &str, callback: Box<dyn FnMut(Event)>,
  ) {
    let closure = Closure::wrap(callback);

    self
      .html
      .add_event_listener_with_callback(
        event_name,
        closure.as_ref().unchecked_ref(),
      )
      .expect("failed to add 'onended' event-listener to <video> element");

    // leaks memory
    // TODO: How to drop it?
    closure.forget();
  }

  /// Dispatches an Event (with `event_name`) and (synchronously) invokes the affected EventListeners in the appropriate order.
  ///
  /// [EventTarget.dispatchEvent()](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)
  pub fn dispatch_event(&self, event_name: &str) {
    let event = Event::new(event_name).unwrap();
    self.html.dispatch_event(&event).unwrap();
  }

  /// Indicates whether the video has ended playback
  ///
  /// [HTMLMediaElement.ended](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended)
  pub fn ended(&self) -> bool {
    self.html.ended()
  }

  pub fn paused(&self) -> bool {
    self.html.paused()
  }

  /// `true` if data is available for the current playback position
  ///
  /// Read more: [HTMLMediaElement.readyState](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/readyState)
  pub fn ready(&self) -> bool {
    self.html.ready_state() >= 2
  }
}

impl From<&Event> for Video {
  fn from(event: &Event) -> Self {
    Video::new(
      &event
        .target()
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .expect("could not cast event target to video element"),
    )
  }
}

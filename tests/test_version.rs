// Copyright 2026 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use libwebp_rs::webp::decode::WEBP_DECODER_ABI_VERSION;
use libwebp_rs::webp::encode::WEBP_ENCODER_ABI_VERSION;

// Basic test to check that the ABI version is set.
#[test]
fn version() {
    assert_eq!((WEBP_ENCODER_ABI_VERSION >> 8) & 0xFF, 2);
    assert_eq!((WEBP_ENCODER_ABI_VERSION >> 0) & 0xFF, 16);
    assert_eq!((WEBP_DECODER_ABI_VERSION >> 8) & 0xFF, 2);
    assert_eq!((WEBP_DECODER_ABI_VERSION >> 0) & 0xFF, 16);
}

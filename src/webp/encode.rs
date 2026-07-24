// Copyright 2026 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
// -----------------------------------------------------------------------------
//
//!  WebP encoder: main interface
//
// Author: Skal (pascal.massimino@gmail.com)

use libc::c_int;

pub const WEBP_ENCODER_ABI_VERSION: c_int = 0x0210; // MAJOR(8b) + MINOR(8b)

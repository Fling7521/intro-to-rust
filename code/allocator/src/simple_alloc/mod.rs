// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! This memory management is based on musl libc.

pub use allocator::SimpleAllocator;

mod allocator;
//mod free;
//mod malloc;
//mod realloc;

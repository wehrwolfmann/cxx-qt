// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Wehrwolfmann <256216494+wehrwolfmann@users.noreply.github.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Checks that a `#[qobject]` behind a `cfg` compiles both when the `cfg` is
//! enabled and when it is disabled.
//!
//! The `cfgs` unit test of cxx-qt-gen only compares token streams, so a
//! disabled QObject that still generated code referring to itself went
//! unnoticed. Building it here catches that.
//!
//! Exactly one of the two QObjects below is compiled in any configuration, so
//! both the enabled and the disabled code path are always built: a plain build
//! has `CfgDisabled` compiled out, a `--all-features` build has `CfgEnabled`
//! compiled in and `CfgDisabled` compiled out.
//!
//! Note that the feature must not contain a `-`, as `CARGO_FEATURE_` variables
//! spell it `_` and cxx-qt-build then never sees the feature as enabled, which
//! makes the generated C++ disagree with the generated Rust.

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "RustQt" {
        #[qobject]
        #[cfg(feature = "cfg_enabled")]
        #[qproperty(i32, number)]
        type CfgEnabled = super::CfgEnabledRust;

        #[qobject]
        #[cfg(not(feature = "cfg_enabled"))]
        #[qproperty(i32, number)]
        type CfgDisabled = super::CfgDisabledRust;
    }
}

#[cfg(feature = "cfg_enabled")]
#[derive(Default)]
pub struct CfgEnabledRust {
    number: i32,
}

#[cfg(not(feature = "cfg_enabled"))]
#[derive(Default)]
pub struct CfgDisabledRust {
    number: i32,
}

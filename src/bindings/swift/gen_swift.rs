/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::super::interface::ComponentInterface;

// Some config options for it the caller wants to customize the generated python.
// Note that this can only be used to control details of the python *that do not affect the underlying component*,
// sine the details of the underlying component are entirely determined by the `ComponentInterface`.
pub struct Config {
    // No config options yet.
}

impl Config {
    pub fn from(_ci: &ComponentInterface) -> Self {
        Config {
            // No config options yet
        }
    }
}

#[derive(Template)]
#[template(
    ext = "h",
    escape = "none",
    source = r#"
// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef struct RustBuffer {
    int64_t len;
    uint8_t *_Nullable data;
} RustBuffer;
"#
)]
pub struct BridgingHeader<'config, 'ci> {
    config: &'config Config,
    ci: &'ci ComponentInterface,
}

impl<'config, 'ci> BridgingHeader<'config, 'ci> {
    pub fn new(config: &'config Config, ci: &'ci ComponentInterface) -> Self {
        Self { config, ci }
    }
}

#[derive(Template)]
#[template(
    ext = "swift",
    escape = "none",
    source = r#"
// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

import Foundation

extension Data {
    init(rustBuffer: RustBuffer) {
        self.init(bytes: rustBuffer.data!, count: Int(rustBuffer.len))
    }
}

// A helper class to extract a byte stream from a `Data`.

class DataReader {
    let data: Data
    let offset: UInt64

    init(data: Data) {
        self.data = data
        self.offset = 0
    }

    mutating func readUInt8(): UInt8 {
        let value = data[offset]
        offset += 1
        return value
    }

    mutating func readUInt32(): UInt32 {
        let value = UInt32()
        withUnsafeMutableBytes(of: &value, { data.copyBytes(to: $0, range: offset..<MemoryLayout<UInt32>.size)} )
        offset += MemoryLayout<UInt32>.size
        return value
    }

    mutating func readUInt64(): UInt64 {
        let value = UInt64()
        withUnsafeMutableBytes(of: &value, { data.copyBytes(to: $0, range: offset..<MemoryLayout<UInt64>.size)} )
        offset += MemoryLayout<UInt64>.size
        return value
    }
}

// A helper class to write bytes into a `Data`.

class DataWriter {
    // ...
}

// Helpers for lifting primitive data types from a Swift `Data`.

extension UInt8 {
    internal static func lift(_ v: UInt8): UInt8 {
        return v
    }

    internal static func liftFrom(_ reader: DataReader): UInt8 {
        return reader.readUInt8()
    }
}

extension UInt32 {
    internal static func lift(_ v: UInt32): UInt32 {
        return v
    }

    internal static func liftFrom(_ reader: DataReader): UInt32 {
        return reader.readUInt32()
    }
}

extension UInt64 {
    internal static func lift(_ v: UInt64): UInt64 {
        return v
    }

    internal static func liftFrom(_ reader: DataReader): UInt64 {
        return reader.readUInt64()
    }
}

// Public interface members begin here.

{% for e in ci.iter_enum_definitions() %}
    public enum {{ e.name() }} {
        {% for value in e.values() %}
        case {{ value }} // TODO: Idiomatic enum variants start with a lowercase letter.
        {% endfor %}

        internal static func lift(_ number: UInt8) -> {{ e.name() }} {
            switch number {
            {% for value in e.values() %}
            case {{ loop.index }}: return .{{ value }} // TODO: Ditto, lowercase.
            {% endfor %}
            default: preconditionFailure("unreachable")
            }
        }

        internal static func liftFrom(_ buf: ByteBuffer): {{ e.name() }} {
            return {{ e.name() }}.lift(UInt8.liftFrom(buf))
        }
    }
{%- endfor -%}
"#
)]
pub struct SwiftWrapper<'config, 'ci> {
    config: &'config Config,
    ci: &'ci ComponentInterface,
}

impl<'config, 'ci> SwiftWrapper<'config, 'ci> {
    pub fn new(config: &'config Config, ci: &'ci ComponentInterface) -> Self {
        Self { config, ci }
    }
}

/// Filters for our Askama templates above. These output C (for the bridging
/// header) and Swift (for the actual library) declarations.
mod filters {
    use super::*;
    use std::fmt;

    // Lowers a WebIDL type into a C type for the bridging header.
    pub fn decl_c(type_: &TypeReference) -> Result<String, askama::Error> {
        Ok(match type_ {
            // These native types map nicely to the FFI without conversion.
            TypeReference::U32 => "uint32_t".into(),
            TypeReference::U64 => "uint64_t".into(),
            TypeReference::Float => "float".into(),
            TypeReference::Double => "double".into(),
            TypeReference::Bytes => "RustBuffer".into(),
            // Our FFI lowers Booleans into bytes, to work around JNA bugs.
            // We'll lift these up into Booleans on the Swift side.
            TypeReference::Boolean => "uint8_t".into(),
            // These types need conversation, and special handling for lifting/lowering.
            TypeReference::Enum(_) => "uint32_t".into(),
            TypeReference::Record(_) => "RustBuffer".into(),
            TypeReference::Optional(_) => "RustBuffer".into(),
            TypeReference::Object(_) => "uint64_t".into(),
            _ => panic!("[TODO: decl_c({:?})", type_),
        })
    }

    // Lowers a WebIDL type into a Swift type.
    pub fn decl_swift(type_: &TypeReference) -> Result<String, askama::Error> {
        Ok(match type_ {
            // These native types map nicely to the FFI without conversion.
            TypeReference::U32 => "UInt32".into(),
            TypeReference::U64 => "UInt64".into(),
            TypeReference::Float => "Float".into(),
            TypeReference::Double => "Double".into(),
            TypeReference::Bytes => "RustBuffer".into(),
            // Our FFI lowers Booleans into bytes, to work around JNA bugs.
            // We'll lift these up into Booleans on the Swift side.
            TypeReference::Boolean => "uint8_t".into(),
            // These types need conversation, and special handling for lifting/lowering.
            TypeReference::Enum(_) => "uint32_t".into(),
            TypeReference::Record(_) => "RustBuffer".into(),
            TypeReference::Optional(_) => "RustBuffer".into(),
            TypeReference::Object(_) => "uint64_t".into(),
            _ => panic!("[TODO: decl_c({:?})", type_),
        })
    }
}

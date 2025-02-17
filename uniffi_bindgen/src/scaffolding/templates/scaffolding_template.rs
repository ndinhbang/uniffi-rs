// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!
{% import "macros.rs" as rs %}

// Unit struct to parameterize the FfiConverter trait.
//
// We use FfiConverter<UniFfiTag> to handle lowering/lifting/serializing types for this crate.  See
// https://mozilla.github.io/uniffi-rs/internals/lifting_and_lowering.html#code-generation-and-the-fficonverter-trait
// for details.
//
// This is pub, since we need to access it to support external types
#[doc(hidden)]
pub struct UniFfiTag;

// Check for compatibility between `uniffi` and `uniffi_bindgen` versions.
// Note that we have an error message on the same line as the assertion.
// This is important, because if the assertion fails, the compiler only
// seems to show that single line as context for the user.
uniffi::assert_compatible_version!("{{ uniffi_version }}"); // Please check that you depend on version {{ uniffi_version }} of the `uniffi` crate.

{% for ty in ci.iter_types() %}
{%- match ty %}
{%- when Type::Map with (k, v) -%}
{# Next comment MUST be after the line to be in the compiler output #}
uniffi::deps::static_assertions::assert_impl_all!({{ k|type_rs }}: ::std::cmp::Eq, ::std::hash::Hash); // record<{{ k|type_rs }}, {{ v|type_rs }}>
{%- else %}
{%- endmatch %}
{% endfor %}

{% include "RustBuffer.rs" %}

// Error definitions, corresponding to `error` in the UDL.
{% for e in ci.error_definitions() %}
{% include "ErrorTemplate.rs" %}
{% endfor %}

// Enum definitions, corresponding to `enum` in UDL.
{% for e in ci.enum_definitions() %}
{% include "EnumTemplate.rs" %}
{% endfor %}

// Record definitions, implemented as method-less structs, corresponding to `dictionary` objects.
{% for rec in ci.record_definitions() %}
{% include "RecordTemplate.rs" %}
{% endfor %}

// Top level functions, corresponding to UDL `namespace` functions.
{%- for func in ci.function_definitions() %}
{% include "TopLevelFunctionTemplate.rs" %}
{% endfor -%}

// Object definitions, corresponding to UDL `interface` definitions.
{% for obj in ci.object_definitions() %}
{% include "ObjectTemplate.rs" %}
{% endfor %}

// Callback Interface definitions, corresponding to UDL `callback interface` definitions.
{% for cbi in ci.callback_interface_definitions() %}
{% include "CallbackInterfaceTemplate.rs" %}
{% endfor %}

// External and Wrapped types
{% include "ExternalTypesTemplate.rs" %}

// The `reexport_uniffi_scaffolding` macro
{% include "ReexportUniFFIScaffolding.rs" %}

{%- import "macros.rs" as rs -%}

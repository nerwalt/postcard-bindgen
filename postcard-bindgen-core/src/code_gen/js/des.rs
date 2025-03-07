use genco::{quote, tokens::quoted};

use crate::{
    code_gen::{
        js::{generateable::container::BindingTypeGenerateable, Tokens},
        utils::TokensIterExt,
    },
    registry::BindingType,
    utils::StrExt,
};

pub fn gen_deserializer_code() -> Tokens {
    quote! {
        class Deserializer {
            constructor(bytes_in) { this.bytes = Array.from(bytes_in) }
            pop_next = () => { const next = this.bytes.shift(); if (next === undefined) { throw "input buffer too small" } return next }
            pop_n = (n) => { const bytes = Array(); for (let i = 0; i < n; i++) { bytes.push(this.bytes.shift()) } return bytes }
            get_uint8 = () => this.pop_next()
            try_take = (n_bytes) => { let out = 0n, v_max = varint_max(n_bytes); for (let i = 0; i < v_max; i++) { const val = this.pop_next(), carry = BigInt(val & 0x7F); out |= carry << BigInt(7 * i); if ((val & 0x80) === 0) { if (i === v_max - 1 && val > max_of_last_byte(n_bytes)) { throw "Bad Variant" } else return Number(out) } } throw "Bad Variant"; }
            deserialize_bool = () => { const byte = this.pop_next(); return byte === undefined ? undefined : byte > 0 ? true : false }
            deserialize_number = (n_bytes, signed) => { if (n_bytes === U8_BYTES) { return this.get_uint8() } else if (n_bytes === U16_BYTES || n_bytes === U32_BYTES || n_bytes === U64_BYTES || n_bytes === U128_BYTES) { const val = this.try_take(n_bytes); return to_number_if_safe(signed ? de_zig_zag_signed(val) : val) } else { throw "byte count not supported" } }
            deserialize_number_float = (n_bytes) => { const b_buffer = new ArrayBuffer(n_bytes), b_view = new DataView(b_buffer); this.pop_n(n_bytes).forEach((b, i) => b_view.setUint8(i, b)); if (n_bytes === U32_BYTES) { return b_view.getFloat32(0, true) } else if (n_bytes === U64_BYTES) { return b_view.getFloat64(0, true) } else { throw "byte count not supported" } }
            deserialize_string = () => { const str = this.pop_n(this.try_take(U32_BYTES)); return String.fromCharCode(...str) }
            deserialize_array = (des, len) => Array.from({length: len === undefined ? this.try_take(U32_BYTES) : len}, (v, i) => des(this))
            deserialize_string_key_map = (des) => { return [...Array(this.try_take(U32_BYTES))].reduce((prev) => { prev[this.deserialize_string()] = des(this); return prev }, {}) }
            deserialize_map = (des) => { return [...Array(this.try_take(U32_BYTES))].reduce((prev) => { const d = des(this); prev.set(d[0], d[1]); return prev }, new Map()) }
        }
    }
}

pub fn gen_des_functions(bindings: impl AsRef<[BindingType]>) -> Tokens {
    bindings
        .as_ref()
        .iter()
        .map(gen_des_function_for_type)
        .join_with_line_breaks()
}

fn gen_des_function_for_type(binding_type: &BindingType) -> Tokens {
    let obj_name = binding_type.inner_name().to_obj_identifier();
    let des_body = binding_type.gen_des_body();
    quote! {
        const deserialize_$obj_name = (d) => $des_body
    }
}

pub fn gen_deserialize_func(defines: impl AsRef<[BindingType]>) -> Tokens {
    let body = defines
        .as_ref()
        .iter()
        .map(gen_des_case)
        .join_with_semicolon();
    quote!(
        module.exports.deserialize = (type, bytes) => {
            if (!(typeof type === "string")) {
                throw "type must be a string"
            }
            const d = new Deserializer(bytes)
            switch (type) { $body }
        }
    )
}

fn gen_des_case(define: &BindingType) -> Tokens {
    let name = define.inner_name();
    let case_str = quoted(name);
    let type_name = name.to_obj_identifier();
    quote!(case $case_str: return deserialize_$type_name(d))
}

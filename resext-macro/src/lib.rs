//! Procedural macro implementation for ResExt.
//!
//! This crate provides the `#[resext]` proc-macro attribute for ergonomic error handling.
//! It is not meant to be used directly - use the `resext` crate instead.
//!
//! # Overview
//!
//! The proc macro generates all necessary error handling code from a simple attribute:
//!
//! ```rust,ignore
//! #[resext]
//! enum MyError {
//!     Io(std::io::Error),
//!     Network(reqwest::Error),
//! }
//! ```
//!
//! This expands to approximately 200 lines of boilerplate including:
//!
//! - `Display` and `Error` trait implementations
//! - `#struct_name` wrapper struct with context storage
//! - `ResExt` trait with context methods
//! - `From` implementations for automatic conversion
//! - Type alias for `Result<T, #struct_name>`
//!
//! # Attribute Options
//!
//! See the [resext crate documentation](https://docs.rs/resext) for detailed
//! information on all available options.

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DeriveInput, Error, Ident, LitStr, parse::Parse, parse_macro_input, spanned::Spanned,
};

/// Generate error handling boilerplate for an enum.
///
/// # Usage
///
/// Basic usage with default settings:
///
/// ```rust,ignore
/// #[resext]
/// enum MyError {
///     Io(std::io::Error),
///     Parse(std::num::ParseIntError),
/// }
/// ```
///
/// With custom formatting:
///
/// ```rust,ignore
/// #[resext(
///     prefix = "ERROR: ",
///     delimiter = " -> ",
///     include_variant = true
/// )]
/// enum MyError {
///     Network(reqwest::Error),
///     Database(sqlx::Error),
/// }
/// ```
///
/// ---
///
/// # Options
///
/// - `prefix` - Prepend to all error messages
/// - `suffix` - Append to all error messages
/// - `msg_prefix` - Prepend to each context message
/// - `msg_suffix` - Append to each context message
/// - `delimiter` - Separator between contexts (default: " - ")
/// - `source_prefix` - Prepend to underlying error (default: "Error: ")
/// - `include_variant` - Show variant name in output (default: false)
/// - `alias` - Custom type alias name (default: Res)
///
/// ---
///
/// # Examples
///
/// ```rust,ignore
/// use resext::resext;
///
/// #[resext(alias = AppResult)]
/// enum AppError {
///     Io(std::io::Error),
///     Network(reqwest::Error),
/// }
///
/// fn example() -> AppResult<()> {
///     std::fs::read("file.txt")
///         .context("Failed to read file")?;
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn resext(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(attr as ResExtArgs);

    let enum_name = &input.ident;
    let vis = &input.vis;

    let alias = args.alias.unwrap_or_else(|| quote! { Res });
    let struct_name = quote::format_ident!("{}Err", alias.to_string());
    let buf_name = quote::format_ident!("{}Buf", alias.to_string());

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return Error::new(input.ident.span(), "`#[resext]` can only be applied to enums")
                .to_compile_error()
                .into();
        }
    };

    let include_variant = args.include_variant;
    let mut errors: Option<Error> = None;
    let display_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                if include_variant {
                    quote! {
                        #enum_name::#variant_name(var) => write!(f, "{}: {}", stringify!(#variant_name), var),
                    }
                } else {
                    quote! {
                        #enum_name::#variant_name(var) => write!(f, "{}", var),
                    }
                }
            }

            syn::Fields::Named(fields) if fields.named.len() == 1 => {
                let variant_field = fields.named[0].ident.as_ref().unwrap();

                if include_variant {
                    quote! {
                        #enum_name::#variant_name { #variant_field } => write!(f, "{}: {}: {}", stringify!(#variant_name), stringify!(#variant_field), #variant_field),
                    }
                } else {
                    quote! {
                        #enum_name::#variant_name { #variant_field } => write!(f, "{}", #variant_field),
                    }
                }
            }

            syn::Fields::Unit => {
                quote! {
                    #enum_name::#variant_name => write!(f, "{}", stringify!(#variant_name)),
                }
            }

            _ => {
                let error = Error::new(
                    variant.fields.span(),
                    "enum variants used in `#[resext]` can only have 1 field",
                );

                match &mut errors {
                    Some(err) => err.combine(error),
                    None => errors = Some(error),
                };

                quote! {}
            }
        }
    });

    let from_impls = variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field_type = &fields.unnamed[0].ty;

                Some(quote! {
                    impl From<#field_type> for #enum_name {
                        fn from(value: #field_type) -> Self {
                            Self::#variant_name(value)
                        }
                    }

                    impl From<#field_type> for #struct_name {
                        fn from(value: #field_type) -> Self {
                            Self { msg: #buf_name::new(), source: #enum_name::#variant_name(value) }
                        }
                    }
                })
            }

            syn::Fields::Named(fields) if fields.named.len() == 1 => {
                let field_name = fields.named[0].ident.as_ref().unwrap();
                let field_type = &fields.named[0].ty;

                Some(quote! {
                    impl From<#field_type> for #enum_name {
                        fn from(value: #field_type) -> Self {
                            Self::#variant_name { #field_name: value }
                        }
                    }

                    impl From<#field_type> for #struct_name {
                        fn from(value: #field_type) -> Self {
                            Self { msg: #buf_name::new(), source: #enum_name::#variant_name { #field_name: value } }
                        }
                    }
                })
            }

            _ => None,
        }
    });

    let prefix = args.prefix.unwrap_or_default();
    let suffix = args.suffix.unwrap_or_default();
    let msg_prefix = args.msg_prefix.unwrap_or_default();
    let msg_suffix = args.msg_suffix.unwrap_or_default();
    let delimiter = args.delimiter.unwrap_or_else(|| String::from(" - "));
    let source_prefix = args.source_prefix.unwrap_or_else(|| String::from("Error: "));
    let buf_size = args.buf_size.unwrap_or(64);

    let expanded = quote! {
        #[derive(Debug)]
        #input

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #(#display_match_arms)*
                }
            }
        }

        impl std::error::Error for #enum_name {}

        /// Wrapper type that holds your error with optional context messages.
        ///
        /// This type is automatically created when you use `.context()` or
        /// `.with_context()` on a Result.
        #[doc(hidden)]
        #vis struct #struct_name {
            msg: #buf_name,
            #vis source: #enum_name
        }

        impl core::fmt::Write for #struct_name {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.msg.write_str(s)
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.curr_pos == 0 {
                    write!(f, "{}{}{}", #source_prefix, &self.source, #suffix)
                } else {
                    write!(
                        f,
                        "{}{}\n{}{}{}",
                        #prefix,
                        unsafe { std::str::from_utf8_unchecked(&self.msg.get_slice()) },
                        #source_prefix,
                        self.source,
                        #suffix,
                    )
                }
            }
        }

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.curr_pos == 0 {
                    write!(f, "{}{:?}{}", #source_prefix, &self.source, #suffix)
                } else {
                    write!(
                        f,
                        "{}{}\n{}{:?}{}",
                        #prefix,
                        unsafe { std::str::from_utf8_unchecked(&self.msg.get_slice()) },
                        #source_prefix,
                        self.source,
                        #suffix,
                    )
                }
            }
        }

        impl #struct_name {
            /// Helper method for constructing #struct_name without using `.context()` or
            /// `.with_context()` on a Result.
            ///
            /// This method:
            /// ```rust,ignore
            /// #struct_name::new(b"Failed to read file".to_vec(), std::io::Error::other(""));
            /// ```
            ///
            /// - is the same as:
            /// ```rust,ignore
            /// #struct_name { b"Failed to read file".to_vec(),
            /// ErrorEnum::Io(std::io::Error::other("")) }
            /// ```
            #vis fn new<E>(msg: &str, source: E) -> Self where #enum_name: From<E> {
                use core::fmt::Write;
                let mut buf = #buf_name::new();
                let _ = buf.write_str(msg);
                Self { msg: buf, source: #enum_name::from(source) }
            }
        }

        impl From<#enum_name> for #struct_name {
            fn from(value: #enum_name) -> Self {
                Self { msg: #buf_name::new(), source: value }
            }
        }

        #(#from_impls)*

        /// Extension trait for adding context to Result types.
        ///
        /// Automatically implemented for all `Result<T, E>` where `E` can be
        /// converted into your error enum.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// std::fs::read("file.txt")
        ///     .context("Failed to read file")?;
        /// ```
        #[doc(hidden)]
        #vis trait ResExt<'r, T> {
            /// Add a static context message to an error.
            ///
            /// The message is only allocated if an error occurs.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// std::fs::read("config.toml")
            ///     .context("Failed to read config")?;
            /// ```
            #[doc(hidden)]
            fn context(self, msg: &str) -> Result<T, #struct_name>;

            /// Add a dynamic context message (computed only on error).
            ///
            /// Use this when the context message needs runtime information.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// std::fs::read(path)
            ///     .with_context(|| format!("Failed to read: {}", path))?;
            /// ```
            #[doc(hidden)]
            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, #struct_name>;

            /// Add raw bytes as context (must be valid UTF-8).
            ///
            /// # Safety
            ///
            /// The bytes must be valid UTF-8
            #[doc(hidden)]
            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, #struct_name>;

            /// Exit the process with the given code if the result is an error.
            ///
            /// Useful for CLI applications that want to exit on critical errors.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let config = load_config().or_exit(1);
            /// ```
            #[doc(hidden)]
            fn or_exit(self, code: i32) -> T;

            /// Like `or_exit` but prints a custom message before exiting.
            ///
            /// # Examples
            ///
            /// ```rust,ignore
            /// let data = load_critical_data()
            ///     .better_expect(|| "FATAL: Cannot start without data", 1);
            /// ```
            #[doc(hidden)]
            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T;
        }

        impl<'r, T> ResExt<'r, T> for Result<T, #struct_name> {
            fn context(self, msg: &str) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        use core::fmt::Write;
                        if err.msg.curr_pos == 0 {
                            let _ = write!(&mut err, "{}", msg);
                        } else {
                            let _ = write!(&mut err, "\n{}{}{}{}", #delimiter, #msg_prefix, msg, #msg_suffix);
                        }
                        Err(err)
                    }
                }
            }

            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        use core::fmt::Write;
                        if err.msg.curr_pos == 0 {
                            let _ = write!(&mut err, "{}", args);
                        } else {
                            let _ = write!(&mut err, "\n{}{}{}{}", #delimiter, #msg_prefix, args, #msg_suffix);
                        }
                        Err(err)
                    }
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.curr_pos == 0 {
                            err.msg.extend_from_slice(bytes);
                        } else {
                            let bytes2 = #delimiter.as_bytes();
                            let bytes3 = #msg_prefix.as_bytes();
                            let bytes4 = #msg_suffix.as_bytes();
                            err.msg.extend_from_slice(b"\n");
                            err.msg.extend_from_slice(bytes2);
                            err.msg.extend_from_slice(bytes3);
                            err.msg.extend_from_slice(bytes);
                            err.msg.extend_from_slice(bytes4);
                        }
                        Err(err)
                    }
                }
            }

            fn or_exit(self, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(_) => std::process::exit(code),
                }
            }

            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{}\nError: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        impl<'r, T, E: std::fmt::Display> ResExt<'r, T> for Result<T, E>
        where
            #enum_name: From<E>,
        {
            fn context(self, msg: &str) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        use core::fmt::Write;
                        let mut buf = #buf_name::new();
                        let _ = write!(&mut buf, "{}", msg);
                        Err(#struct_name { msg: buf, source: err.into() })
                    }
                }
            }

            fn with_context(self, args: core::fmt::Arguments<'r>) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        use core::fmt::Write;
                        let mut buf = #buf_name::new();
                        let _ = write!(&mut buf, "{}", args);
                        Err(#struct_name { msg: buf, source: err.into() })
                    }
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, #struct_name> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => {
                        use core::fmt::Write;
                        let mut buf = #buf_name::new();
                        buf.extend_from_slice(bytes);
                        Err(#struct_name { msg: buf, source: err.into() })
                    }
                }
            }

            fn or_exit(self, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(_) => std::process::exit(code),
                }
            }

            fn better_expect<M: std::fmt::Display, F: FnOnce() -> M>(self, f: F, code: i32) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{}\nError: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        #vis type #alias<T> = Result<T, #struct_name>;

        struct #buf_name {
            curr_pos: u16,
            buf: [u8; #buf_size],
        }

        impl #buf_name {
            fn new() -> Self {
                Self { buf: [0; #buf_size], curr_pos: 0 }
            }

            fn as_str(&self) -> &str {
                unsafe { core::str::from_utf8_unchecked(&self.buf[..self.curr_pos as usize]) }
            }

            fn get_slice(&self) -> &[u8] {
                &self.buf[..self.curr_pos as usize]
            }

            fn extend_from_slice(&mut self, bytes: &[u8]) {
                let pos = self.curr_pos as usize;
                let cap = #buf_size - pos;
                let to_copy = core::cmp::min(cap, bytes.len());

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;
            }
        }

        impl core::fmt::Write for #buf_name {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let bytes = s.as_bytes();
                let pos = self.curr_pos as usize;
                let cap = #buf_size - pos;
                let limit = core::cmp::min(bytes.len(), cap);

                let to_copy = match bytes[..limit]
                    .iter()
                    .rposition(|&b| (b & 0xC0) != 0x80)
                {
                    Some(start_of_last_char) => {
                        let last_char_byte = bytes[start_of_last_char];
                        let width = match last_char_byte {
                            0..=127 => 1,
                            192..=223 => 2,
                            224..=239 => 3,
                            240..=247 => 4,
                            _ => 1,
                        };
                        if start_of_last_char + width <= limit {
                            start_of_last_char + width
                        } else {
                            start_of_last_char
                        }
                    }
                    None => 0,
                };

                self.buf[pos..pos + to_copy].copy_from_slice(&bytes[..to_copy]);
                self.curr_pos += to_copy as u16;

                Ok(())
            }
        }
    };

    if let Some(error) = errors {
        TokenStream::from(error.to_compile_error())
    } else {
        TokenStream::from(expanded)
    }
}

struct ResExtArgs {
    prefix: Option<String>,
    suffix: Option<String>,
    msg_prefix: Option<String>,
    msg_suffix: Option<String>,
    delimiter: Option<String>,
    source_prefix: Option<String>,
    include_variant: bool,
    alias: Option<proc_macro2::TokenStream>,
    buf_size: Option<usize>,
}

impl Parse for ResExtArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = ResExtArgs {
            prefix: None,
            suffix: None,
            msg_prefix: None,
            msg_suffix: None,
            delimiter: None,
            source_prefix: None,
            include_variant: false,
            alias: None,
            buf_size: None,
        };

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;

            match key.to_string().as_str() {
                "prefix" => {
                    let value: LitStr = input.parse()?;
                    args.prefix = Some(value.value())
                }

                "suffix" => {
                    let value: LitStr = input.parse()?;
                    args.suffix = Some(value.value())
                }

                "msg_prefix" => {
                    let value: LitStr = input.parse()?;
                    args.msg_prefix = Some(value.value())
                }

                "msg_suffix" => {
                    let value: LitStr = input.parse()?;
                    args.msg_suffix = Some(value.value())
                }

                "delimiter" => {
                    let value: LitStr = input.parse()?;
                    args.delimiter = Some(value.value())
                }

                "source_prefix" => {
                    let value: LitStr = input.parse()?;
                    args.source_prefix = Some(value.value())
                }

                "include_variant" => {
                    let value: syn::LitBool = input.parse()?;
                    args.include_variant = value.value();
                }

                "alias" => {
                    let value: Ident = input.parse()?;
                    args.alias = Some(value.into_token_stream());
                }

                "buf_size" => {
                    let value: syn::LitInt = input.parse()?;
                    args.buf_size = Some(value.base10_parse()?);
                }

                _ => {
                    return Err(Error::new(
                        key.span(),
                        format!(
                            "unknown argument passed to proc-macro attribute `#[resext]`: {}",
                            key
                        ),
                    ));
                }
            }

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(args)
    }
}

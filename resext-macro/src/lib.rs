use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, parse_macro_input, spanned::Spanned};

#[proc_macro_attribute]
pub fn resext(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let vis = &input.vis;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => {
            return Error::new(input.ident.span(), "`#[resext]` can only be applied to enums")
                .to_compile_error()
                .into();
        }
    };

    let mut errors: Option<Error> = None;
    let display_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                quote! {
                    #enum_name::#variant_name(var) => write!(f, "{}", var),
                }
            }

            syn::Fields::Named(fields) if fields.named.len() == 1 => {
                let variant_field = fields.named[0].ident.as_ref().unwrap();

                quote! {
                    #enum_name::#variant_name { #variant_field } => write!(f, "{}", #variant_field),
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
                    "Enum variants used in `#[resext]` can only have 1 field",
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

                    impl From<#field_type> for ResErr {
                        fn from(value: #field_type) -> Self {
                            Self { msg: Vec::new(), source: #enum_name::#variant_name(value) }
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

                    impl From<#field_type> for ResErr {
                        fn from(value: #field_type) -> Self {
                            Self { msg: Vec::new(), source: #enum_name::#variant_name { #field_name: value } }
                        }
                    }
                })
            }

            _ => None,
        }
    });

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

        #vis struct ResErr {
            msg: Vec<u8>,
            #vis source: #enum_name
        }

        impl std::fmt::Display for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.is_empty() {
                    write!(f, "{}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nCaused by: {}",
                        unsafe { std::str::from_utf8_unchecked(&self.msg) },
                        &self.source
                    )
                }
            }
        }

        impl std::fmt::Debug for ResErr {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if self.msg.is_empty() {
                    write!(f, "{:?}", &self.source)
                } else {
                    write!(
                        f,
                        "{}\nCaused by: {:?}\n",
                        unsafe { std::str::from_utf8_unchecked(&self.msg) },
                        &self.source
                    )
                }
            }
        }

        impl ResErr {
            #vis fn new<E>(msg: Vec<u8>, source: E) -> Self where #enum_name: From<E> {
                Self { msg, source: #enum_name::from(source) }
            }
        }

        impl From<#enum_name> for ResErr {
            fn from(value: #enum_name) -> Self {
                Self { msg: Vec::new(), source: value }
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
        #vis trait ResExt<T> {
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
            fn context(self, msg: &str) -> Result<T, ResErr>;

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
            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr>;

            /// Add raw bytes as context (must be valid UTF-8).
            ///
            /// # Safety
            ///
            /// The bytes must be valid UTF-8
            #[doc(hidden)]
            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr>;

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

        impl<T> ResExt<T> for Result<T, ResErr> {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(msg.as_bytes());
                        } else {
                            let bytes = msg.as_bytes();
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
                        }
                        Err(err)
                    }
                }
            }

            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(f().as_bytes());
                        } else {
                            let s = f();
                            let bytes = s.as_bytes();
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
                        }
                        Err(err)
                    }
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(mut err) => {
                        if err.msg.is_empty() {
                            err.msg.extend_from_slice(bytes);
                        } else {
                            let diff: isize = err.msg.capacity() as isize - bytes.len() as isize;
                            if diff < 0 {
                                err.msg.reserve_exact((-diff) as usize);
                            }
                            err.msg.extend_from_slice(b"\n- ");
                            err.msg.extend_from_slice(bytes);
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
                        eprintln!("{}\nCaused by: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        impl<T, E: std::fmt::Display> ResExt<T> for Result<T, E>
        where
            #enum_name: From<E>,
        {
            fn context(self, msg: &str) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: msg.as_bytes().to_vec(), source: err.into() }),
                }
            }

            fn with_context<F: FnOnce() -> String>(self, f: F) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: f().as_bytes().to_vec(), source: err.into() }),
                }
            }

            unsafe fn byte_context(self, bytes: &[u8]) -> Result<T, ResErr> {
                match self {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(ResErr { msg: bytes.to_vec(), source: err.into() }),
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
                        eprintln!("{}\nCaused by: {}", f(), err);
                        std::process::exit(code);
                    }
                }
            }
        }

        // fixed alias temporarily
        // TODO: Provide custom aliases through attribute parsing
        #vis type Res<T> = Result<T, ResErr>;
    };

    if let Some(error) = errors {
        TokenStream::from(error.to_compile_error())
    } else {
        TokenStream::from(expanded)
    }
}

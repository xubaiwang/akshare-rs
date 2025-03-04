use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, Attribute, Ident, Token};

struct Op {
    attrs: Vec<Attribute>,
    name: Ident,
    params: Punctuated<OpParam, Token![,]>,
}

struct OpParam {
    attrs: Vec<Attribute>,
    name: Ident,
    ty: Ident,
}

impl Op {
    /// # Example
    ///
    /// - `stock_szse_summary` => `StockSzseSummary`
    fn struct_name(&self) -> Ident {
        Ident::new(
            &self.name.to_string().to_upper_camel_case(),
            Span::call_site(),
        )
    }

    /// # Example
    ///
    /// - `()` => `()`
    /// - `(str)` => (T1)`
    /// - `(str, float)` => `(T1, T2)`
    fn generics(&self) -> impl Iterator<Item = Ident> {
        (0..self.params.len()).map(to_generic_ident)
    }

    /// # Example
    ///
    /// - `()` => `()`
    /// - `(str)` => (Py<PyString>)`
    /// - `(str, float)` => `(Py<PyString>, Py<PyFloat>)`
    fn generic_defaults(&self) -> impl Iterator<Item = TokenStream> + use<'_> {
        self.param_tys().map(|ty| {
            let target = to_target_type(ty);
            quote! {
                pyo3::Py<#target>
            }
        })
    }

    /// # Example
    ///
    /// - `()` => `()`
    /// - `(str)` => (T1: IntoPyObject<PyString>)`
    /// - `(str, float)` => `(T1: IntoPyObject<PyString>, T2: IntoPyObject<PyFloat>)`
    fn generic_constraints(&self) -> impl Iterator<Item = TokenStream> + use<'_> {
        self.param_tys().enumerate().map(|(i, ty)| {
            let generic = to_generic_ident(i);
            let target = to_target_type(ty);
            quote! {
                #generic: for<'py> pyo3::IntoPyObject<'py, Target = #target>
            }
        })
    }

    /// # Example
    ///
    /// - `{date: str}` => `date: T1`
    /// - `{date: str, symbol: str}` => `date: T1, symbol: T2`
    fn fields(&self) -> impl Iterator<Item = TokenStream> + use<'_> {
        self.param_names().enumerate().map(|(i, name)| {
            let generic = to_generic_ident(i);
            quote! {
                #name: Option<#generic>
            }
        })
    }

    /// # Example
    ///
    /// - `{date: str}` => `date: None`
    /// - `{date: str, symbol: str}` => `date: T1, symbol: T2`
    fn field_inits(&self) -> impl Iterator<Item = TokenStream> + use<'_> {
        self.param_names().map(|name| {
            quote! {
                #name: None
            }
        })
    }

    /// Set kwargs
    fn field_setters(&self) -> impl Iterator<Item = TokenStream> + use<'_> {
        self.param_names().map(|name| {
            quote! {
                if let Some(val) = #name {
                    pyo3::types::PyDictMethods::set_item(&kwargs, stringify!(#name), val)?;
                }
            }
        })
    }

    /// Iterate over parameter names.
    fn param_names(&self) -> impl Iterator<Item = &Ident> {
        self.params.iter().map(|p| &p.name)
    }

    /// Iterate over parameter types.
    fn param_tys(&self) -> impl Iterator<Item = &Ident> {
        self.params.iter().map(|p| &p.ty)
    }
}

/// Numbered generics.
///
/// # Examples
///
/// - `0` => `T1`
/// - `1` => `T2`
fn to_generic_ident(i: usize) -> Ident {
    Ident::new(&format!("T{}", i + 1), Span::call_site())
}

/// Convert `int|float|str` to `pyo3::IntoPyObject` target type.
fn to_target_type(ty: &Ident) -> TokenStream {
    match ty.to_string().as_str() {
        "int" => quote! { pyo3::types::PyInt },
        "float" => quote! { pyo3::types::PyFloat },
        "str" => quote! { pyo3::types::PyString },
        _ => {
            panic!("Parameter type can only be `int`, `float` or `str`");
        }
    }
}

mod kw {
    syn::custom_keyword!(name);
    syn::custom_keyword!(params);
}

impl Parse for Op {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        input.parse::<kw::name>()?;
        input.parse::<Token![:]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<kw::params>()?;
        input.parse::<Token![:]>()?;
        let content;
        braced!(content in input);
        let params = content.parse_terminated(OpParam::parse, Token![,])?;
        input.parse::<Option<Token![,]>>()?;
        Ok(Self {
            attrs,
            name,
            params,
        })
    }
}

impl Parse for OpParam {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ident = input.parse()?;
        Ok(Self { attrs, name, ty })
    }
}

#[proc_macro]
pub fn define_op(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let op = parse_macro_input!(input as Op);

    let expanded_struct = expand_struct(&op);
    let expanded_constructor = expand_constructor(&op);
    let expanded_setters = expand_setters(&op);
    let expanded_call = expand_call(&op);

    let expanded = quote! {
        #expanded_struct
        #expanded_constructor
        #expanded_setters
        #expanded_call
    };

    TokenStream::from(expanded).into()
}

/// Like
///
/// ```
/// #[derive(Clone)]
/// struct StockSzseSummary<T1> {
///     api: AKShare,
///     preserve_index: bool,
///     date: T1,
/// }
/// ```
fn expand_struct(op: &Op) -> TokenStream {
    let struct_name = op.struct_name();
    let struct_attrs = &op.attrs;
    let generics = op.generics();
    let fields = op.fields();
    quote! {
        #(#struct_attrs)*
        #[derive(Clone)]
        pub struct #struct_name<#(#generics),*> {
            api: crate::base::AKShare,
            preserve_index: bool,
            #(#fields,)*
        }
    }
}

/// Like
///
/// ```
/// impl<T1> StockSzseSummary<T1>
/// where
///     T1: for<'py> IntoPyObject<'py, Target = PyString>,
/// {
///    fn call(self) -> PyResult<RecordBatch> {
///       Python::with_gil(|py| {
///           let Self { .. } = self;
///           let kwargs = ...;
///           // set kwargs
///           api.call(...)
///       })
///    }
/// }
/// ```
fn expand_call(op: &Op) -> TokenStream {
    let op_name = &op.name;
    let struct_name = op.struct_name();
    let generics: Vec<_> = op.generics().collect();
    let generic_constraints = op.generic_constraints();
    let field_bindings = op.param_names();
    let field_setters = op.field_setters();
    quote! {
        impl<#(#generics),*> #struct_name<#(#generics),*>
        where
            #(#generic_constraints,)*
        {
            pub fn call(self) -> pyo3::PyResult<arrow::array::RecordBatch> {
                pyo3::Python::with_gil(|py| {
                    let Self {
                        api,
                        preserve_index,
                        #(#field_bindings,)*
                    } = self;
                    let kwargs = pyo3::types::PyDict::new(py);
                    #(#field_setters;)*
                    api.call(py, stringify!(#op_name), (), Some(&kwargs), preserve_index)
                })
            }
        }
    }
}

/// Like
///
/// ```
/// impl AKShare {
///     fn stock_szse_summary() -> StockSzseSummary<Py<PyString>> {
///         StockSzseSummary {
///             api: self.clone(),
///             preserve_index: false,
///             date: None,
///         }
///     }
/// }
/// ```
fn expand_constructor(op: &Op) -> TokenStream {
    let op_name = &op.name;
    let struct_name = op.struct_name();
    let generic_defaults = op.generic_defaults();
    let field_inits = op.field_inits();
    quote! {
        impl crate::base::AKShare {
            pub fn #op_name(&self) -> #struct_name<#(#generic_defaults),*> {
                #struct_name {
                    api: self.clone(),
                    preserve_index: false,
                    #(#field_inits,)*
                }
            }
        }
    }
}

/// Like
///
/// ```
/// impl<T1, T2> StockSzseSummary<T1, T2> {
///     fn preserve_index(...) -> Self {
///         ...
///     }
///
///     fn date<S>(self, date: S) -> StockSzseSummary<S, T2>
///     where
///         S: for<'py> IntoPyObject<'py, Target = PyString>,
///     {
///         let Self { ... } = self;
///         StockSzseSummary { .. };
///     }
/// }
/// ```
fn expand_setters(op: &Op) -> TokenStream {
    let struct_name = op.struct_name();
    let generics: Vec<_> = op.generics().collect();
    let generic_s = Ident::new("S", Span::call_site());

    let field_setters = op.params.iter().enumerate().map(|(i, param)| {
        let OpParam { attrs, name, ty } = param;

        let target = to_target_type(ty);

        let generics_replaced =
            generics.iter().enumerate().map(
                |(j, generic)| {
                    if i == j {
                        &generic_s
                    } else {
                        generic
                    }
                },
            );

        let fields_except: Vec<_> = op
            .param_names()
            .enumerate()
            .filter_map(|(j, name)| (i != j).then_some(name))
            .collect();

        quote! {
            #(#attrs)*
            pub fn #name<S>(self, #name: S) -> #struct_name<#(#generics_replaced),*>
            where
                S: for<'py> pyo3::IntoPyObject<'py, Target = #target>
            {
                let Self {
                    api,
                    preserve_index,
                    #(#fields_except,)*
                    ..
                } = self;

                #struct_name {
                    api,
                    preserve_index,
                    #(#fields_except,)*
                    #name: Some(#name),
                }
            }
        }
    });

    quote! {
        impl<#(#generics),*> #struct_name<#(#generics),*> {
            pub fn preserve_index(mut self, preserve_index: bool) -> Self {
                self.preserve_index = preserve_index;
                self
            }

            #(#field_setters)*
        }
    }
}

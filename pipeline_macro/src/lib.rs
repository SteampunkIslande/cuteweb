use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Expr, ExprTuple, Ident, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

// ── AST types ────────────────────────────────────────────────────────────────

/// A single step in the pipeline.
enum PipelineStep {
    /// `func(a, b)` — direct call, error propagated with `?`.
    Direct { func: Ident, args: Vec<Expr> },

    /// `func((out, out) [, extra…])` — write to a temp file, rename/unlink.
    InPlace {
        func: Ident,
        /// The shared path expression (both slots of the tuple must be equal).
        path: Expr,
        /// Extra positional arguments after the tuple.
        extra: Vec<Expr>,
        /// 1-based counter used to suffix the temp file (`.1.parquet`, etc.).
        idx: usize,
    },
}

struct Pipeline {
    steps: Vec<PipelineStep>,
}

// ── parsing ───────────────────────────────────────────────────────────────────

impl Parse for Pipeline {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let calls = Punctuated::<syn::ExprCall, Token![,]>::parse_terminated(input)?;

        let mut steps = Vec::new();
        let mut inplace_idx = 0usize;

        for call in &calls {
            let func = expr_path_to_ident(&call.func)?;
            let mut args: Vec<Expr> = call.args.iter().cloned().collect();

            if args.len() == 0 {
                steps.push(PipelineStep::Direct { func, args });
                continue;
            }

            if let Some(shared_path) = try_extract_inplace_tuple(&args[0])? {
                inplace_idx += 1;
                let extra = args.drain(1..).collect();
                steps.push(PipelineStep::InPlace {
                    func,
                    path: shared_path,
                    extra,
                    idx: inplace_idx,
                });
            } else {
                steps.push(PipelineStep::Direct { func, args });
            }
        }

        Ok(Pipeline { steps })
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// If `expr` is a 2-element tuple `(x, x)` where both elements produce the
/// same token stream, return `Ok(Some(x))`.  Tuple with differing elements
/// → compile error.  Not a tuple → `Ok(None)`.
fn try_extract_inplace_tuple(expr: &Expr) -> syn::Result<Option<Expr>> {
    let Expr::Tuple(ExprTuple { elems, .. }) = expr else {
        return Ok(None);
    };

    let elems: Vec<&Expr> = elems.iter().collect();

    if elems.len() != 2 {
        return Err(syn::Error::new_spanned(
            expr,
            "in-place step: first argument must be a 2-element tuple `(path, path)`",
        ));
    }

    let lhs = {
        let e = elems[0];
        quote!(#e).to_string()
    };
    let rhs = {
        let e = elems[1];
        quote!(#e).to_string()
    };

    if lhs != rhs {
        return Err(syn::Error::new_spanned(
            expr,
            "in-place step: both tuple elements must be the same expression, e.g. `(out, out)`",
        ));
    }

    Ok(Some(elems[0].clone()))
}

/// Extract a simple [`Ident`] from a path expression.
fn expr_path_to_ident(expr: &Expr) -> syn::Result<Ident> {
    if let Expr::Path(ep) = expr {
        if let Some(ident) = ep.path.get_ident() {
            return Ok(ident.clone());
        }
    }
    Err(syn::Error::new_spanned(
        expr,
        "pipeline step must be a simple function name (no path separators)",
    ))
}

// ── code generation ───────────────────────────────────────────────────────────

fn codegen_step(step: &PipelineStep) -> proc_macro2::TokenStream {
    match step {
        // ── direct ───────────────────────────────────────────────────────────
        PipelineStep::Direct { func, args } => quote! {
            #func(#(#args),*)?;
        },

        // ── in-place ─────────────────────────────────────────────────────────
        PipelineStep::InPlace {
            func,
            path,
            extra,
            idx,
        } => {
            let tmp_ident = Ident::new(&format!("__pipeline_tmp_{}", idx), Span::call_site());
            let tmp_ext = format!(".{}.parquet", idx);

            quote! {
                let #tmp_ident = {
                    let __p: &::std::path::Path = ::std::convert::AsRef::<::std::path::Path>::as_ref(#path);
                    let __stem = __p
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let __parent = __p.parent().unwrap_or_else(|| ::std::path::Path::new("."));
                    __parent.join(format!("{}{}", __stem, #tmp_ext))
                };

                match #func(#path, &#tmp_ident #(, #extra)*) {
                    ::std::result::Result::Err(__e) => {
                        let _ = ::std::fs::remove_file(&#tmp_ident);
                        return ::std::result::Result::Err(__e);
                    }
                    ::std::result::Result::Ok(_) => {
                        ::std::fs::rename(&#tmp_ident, #path)
                            .unwrap_or_else(|__io| {
                                let _ = ::std::fs::remove_file(&#tmp_ident);
                                panic!("pipeline: failed to rename temp file: {}", __io);
                            });
                    }
                }
            }
        }
    }
}

// ── public entry-point ────────────────────────────────────────────────────────

/// Runs a sequence of parquet-processing steps as a safe pipeline.
///
/// # Syntax
///
/// ```rust,ignore
/// pipeline!(
///     step1(input, output),               // direct: propagate error with `?`
///     step2((output, output)),            // in-place: temp-file + rename
///     step3((output, output), 42),        // in-place + extra args
///     step4((output, output), "hi", 15),
/// )?;
/// ```
///
/// **In-place rules**
/// * The first argument must be `(x, x)` — both elements **identical**.
/// * On success the temp file is renamed over the original.
/// * On failure the temp file is deleted and the error is returned.
#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    let Pipeline { steps } = parse_macro_input!(input as Pipeline);

    let step_tokens: Vec<_> = steps.iter().map(codegen_step).collect();

    let expanded = quote! {
        {
            #(#step_tokens)*
            ::std::result::Result::Ok(())
        }
    };

    TokenStream::from(expanded)
}

use proc_macro::TokenStream;

#[proc_macro]
pub fn create_static(input: TokenStream) -> TokenStream {
    cri_adx_proc_impl::create_static::create_static(input.into()).into()
}

use proc_macro::TokenStream;

mod include_sprite_sheet;

#[proc_macro]
pub fn include_sprite_sheet(input: TokenStream) -> TokenStream {
	include_sprite_sheet::process(input.into()).into()
}

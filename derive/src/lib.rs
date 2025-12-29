extern crate proc_macro;

mod item;
mod utils;

use synstructure::decl_derive;

decl_derive!([SpecTecItem, attributes(spectec_node, spectec_field, spectec_atom)] => item::spectec_item_derive);

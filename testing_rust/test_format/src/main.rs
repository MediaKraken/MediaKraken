
#[tokio::main]
async fn main() {
let s: f32 = 333345533 as f32;
let b = &s;
println!("{}", mk_lib_common::mk_lib_common_internationalization::mk_lib_common_internationalization_number_format(b.clone() as i64).unwrap());
// pub fn number_format(s: &f32) -> ::askama::Result<String> {
//     let result = mk_lib_common::mk_lib_common_internationalization::mk_lib_common_internationalization_number_format(s.clone() as u32).unwrap();
//     Ok(format!("{}", result))
// }

println!("{}", mk_lib_common::mk_lib_common_bytesize::mk_lib_common_bytesize(s.clone() as u64).unwrap());

// pub fn byte_format(s: &i64) -> ::askama::Result<String> {
//     let result = mk_lib_common::mk_lib_common_bytesize::mk_lib_common_bytesize(s.clone() as u64).unwrap();
//     Ok(format!("{}", result))
// }
}
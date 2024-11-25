use std::fs;

fn take_template(template: &str, replace_str: &str) -> String {
    template.replace("TEMPLATE", replace_str)
}

fn capitalized(s: &str) -> String {
    let mut chars = s.chars();
    let first_char = chars
        .next()
        .map_or_else(|| "".to_string(), |c| c.to_uppercase().to_string());
    format!("{}{}", first_char, chars.collect::<String>())
}

fn main() {
    let mut arg = std::env::args();
    let Some(mod_name) = arg.nth(1) else {
        println!("Missing name argument");
        return;
    };

    let lib_dir = format!("./{}/", mod_name);
    let include_dir = format!("{}/include/{}/", lib_dir, mod_name);
    let src_dir = format!("{}/src", lib_dir);

    fs::create_dir_all(&lib_dir).expect("cannot create lib dir");
    fs::create_dir_all(&include_dir).expect("cannot create include dir");
    fs::create_dir_all(&src_dir).expect("cannot create src dir");

    let cap_mod_name = capitalized(&mod_name);
    let cmake_file = take_template(include_str!("cmake_template.cmake"), &mod_name);
    let cpp_file = take_template(include_str!("temp.cpp"), &cap_mod_name);
    let h_file = take_template(include_str!("temp.h"), &cap_mod_name);

    fs::write(format!("{}/CMakeLists.txt", lib_dir), &cmake_file)
        .expect("cannot create cmake file");

    fs::write(format!("{}/{}.h", include_dir, cap_mod_name), &h_file)
        .expect("cannot create header file");
    fs::write(format!("{}/{}.cpp", src_dir, cap_mod_name), &cpp_file)
        .expect("cannot create cpp file");
}

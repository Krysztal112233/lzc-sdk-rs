use std::{fs, io::Result, path::PathBuf, str::FromStr};

use walkdir::{DirEntry, WalkDir};

fn main() -> Result<()> {
    let output_directory = output_dir();

    // 清空输出
    if fs::exists(&output_directory).unwrap() {
        fs::remove_dir_all(&output_directory).unwrap();
    }
    fs::create_dir_all(&output_directory).unwrap();

    for ele in ["lzc-boxservice-protos/proto", "lzc-sdk/protos"] {
        let input_directory = input_dir(ele);

        compile(&input_directory, &output_directory);
    }

    create_mod_rs(&output_directory);

    patch_cloud_lazycat_apis_sys();

    Ok(())
}

fn compile(input_directory: &str, output_directory: &str) {
    // 收集所有 `.proto` 文件
    let result = WalkDir::new(input_directory)
        .into_iter()
        .flatten()
        .filter(|it| it.file_type().is_file())
        .map(DirEntry::into_path)
        .filter(|it| {
            it.extension()
                .map(|it| it.to_str().unwrap() == "proto")
                .unwrap_or_default()
        })
        .collect::<Vec<_>>();

    // 配置输出并且全部编译
    tonic_build::configure()
        .out_dir(output_directory)
        .build_server(false)
        .compile_protos(&result, &[&input_directory])
        .unwrap();
}

fn output_dir() -> String {
    let output = "./src/proto".into();

    fs::create_dir_all(&output).unwrap();

    output
}

fn input_dir(path: &str) -> String {
    format!("./.tmp/protos/{}", path)
}

fn patch_cloud_lazycat_apis_sys() {
    const ORIGIN: &str = r#"        pub async fn connect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::LedStream>,
        ) -> std::result::Result<"#;
    const PATCHED: &str = r#"        /// 由 `connect` 更改而来，避免冲突 
        pub async fn connect_led(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::LedStream>,
        ) -> std::result::Result<"#;

    let mut file = PathBuf::from(output_dir());
    file.push("cloud.lazycat.apis.sys.rs");

    let patched_content = fs::read_to_string(file.as_path())
        .unwrap()
        .replace(ORIGIN, PATCHED);

    fs::write(file, patched_content).unwrap();
}

// 生成 `mod.rs`
fn create_mod_rs(output_directory: &str) {
    let result = WalkDir::new(output_directory)
        .into_iter()
        .flatten()
        .filter(|it| it.file_type().is_file())
        .map(|it| it.into_path())
        .filter(|it| it.extension().unwrap().to_str().unwrap() == "rs")
        .collect::<Vec<_>>();

    let mut mod_rs = Vec::<String>::new();

    for file_path in result {
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        let splited = file_name.split(".").collect::<Vec<_>>();

        let mod_name = splited.get(splited.len() - 2).unwrap();
        let mod_file = file_name;

        mod_rs.push(dbg!(format!(
            r#"#[path = "{}"]
pub mod {};
"#,
            mod_file, mod_name
        )));
    }

    let contents = mod_rs.join("\n");

    let mod_file_path = {
        let mut temp = PathBuf::from_str(output_directory).unwrap();
        temp.push("mod.rs");
        temp
    };
    fs::write(mod_file_path, contents).unwrap();
}

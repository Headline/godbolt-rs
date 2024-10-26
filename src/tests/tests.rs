use std::string::String;
use crate::{Godbolt, CompilationFilters, RequestOptions, CompilerOptions, ExecuteParameters};
use std::error::Error;

#[tokio::test]
async fn get_languages() -> Result<(), Box<dyn Error>> {
    let langs = Godbolt::get_languages().await?;
    assert!(langs.len() > 0);
    Ok(())
}

#[tokio::test]
async fn get_compilers() -> Result<(), Box<dyn Error>> {
   let compilers = Godbolt::get_compilers().await?;
    assert!(compilers.len() > 0);
    Ok(())
}

#[tokio::test]
async fn get_compilers_for() -> Result<(), Box<dyn Error>> {
    let compilers = Godbolt::get_compilers_for("c++").await?;
    assert!(compilers.len() > 0);
    Ok(())
}

#[tokio::test]
async fn get_libraries_for() -> Result<(), Box<dyn Error>> {
    let libs = Godbolt::get_libraries_for("c").await?;
    assert!(libs.len() > 0);
    Ok(())
}

#[tokio::test]
async fn base64() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let c = gbolt.resolve("clang1000").unwrap();

    let filters = CompilationFilters {
        binary: None,
        comment_only: Some(true),
        demangle: Some(true),
        directives: Some(true),
        execute: None,
        intel: Some(true),
        labels: Some(true),
        library_code: None,
        trim: None
    };

    let opts = RequestOptions {
        user_arguments: "-O3".to_string(),
        compiler_options: CompilerOptions {
            skip_asm: false,
            executor_request: true,
        },
        execute_parameters: ExecuteParameters {
            args: vec![String::from("awd")],
            stdin: "teststdin".to_string(),
        },
        filters,
    };
    let str = Godbolt::get_base64(&c, "#include <iostream>\nint main() {\nstd::cout << \"😂\";\n}", opts)?;
    assert!(str.len() > 0);
    Ok(())
}


#[tokio::test]
async fn resolve() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let c = gbolt.resolve("clang1000");
    assert!(c.is_some());
    Ok(())
}

#[tokio::test]
async fn format_test() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    assert!(gbolt.formats.len() > 0);
    Ok(())
}

#[tokio::test]
async fn compilation_attempt() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let c = gbolt.resolve("clang1000");
    assert!(c.is_some());
    let compiler = c.unwrap();

    let options = RequestOptions {
        user_arguments: String::from(""),
        compiler_options: CompilerOptions {
            skip_asm: true,
            executor_request: true,
        },
        execute_parameters: ExecuteParameters {
            args: vec![],
            stdin: String::from(""),
        },
        filters: CompilationFilters::default(),
    };

    let res = Godbolt::send_request(&compiler, "int main(void) {return 0;}", options, "godbolt-rs-test").await;
    assert!(res.is_ok());
    Ok(())
}
use crate::{Godbolt, CompilationFilters};
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
async fn godbolt_exec_asm() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let cplusplus = gbolt.cache.iter().find(|p| p.language.name == "C++").unwrap();
    let compiler = &cplusplus.compilers[0];

    let mut filters = CompilationFilters::default();
    filters.execute = Some(true);

    Godbolt::send_request(compiler, "int sum(int a, int b) { return a + b; }", "-O3", &filters).await?;
    Ok(())
}

#[tokio::test]
async fn godbolt_exec_asm_fail() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let cplusplus = gbolt.cache.iter().find(|p| p.language.name == "C++").unwrap();
    let compiler = &cplusplus.compilers[0];

    let mut filters = CompilationFilters::default();
    filters.execute = Some(true);

    Godbolt::send_request(compiler, "int sum(iwnt a, int b) { return a + b; }", "-O3", &filters).await?;
    Ok(())
}

#[tokio::test]
async fn godbolt_exec_asm_filters() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let cplusplus = gbolt.cache.iter().find(|p| p.language.name == "C++").unwrap();
    let compiler = &cplusplus.compilers[0];

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
    Godbolt::send_request(compiler, "int sum(int a, int b) { return a + b; }", "-O3", &filters).await?;
    Ok(())
}

#[tokio::test]
async fn godbolt_exec_asm_filters_fail() -> Result<(), Box<dyn Error>> {
    let gbolt = Godbolt::new().await?;
    let cplusplus = gbolt.cache.iter().find(|p| p.language.name == "C++").unwrap();
    let compiler = &cplusplus.compilers[0];

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
    Godbolt::send_request(compiler, "#include <iostream>\nint main() {\nstd::cout << \"Test\";\n}", "-O3", &filters).await?;
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
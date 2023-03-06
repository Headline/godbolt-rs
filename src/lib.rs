use std::error::Error;
use serde::*;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::fmt;
use base64::{engine, Engine};

mod tests;

#[derive(Clone, Debug, Deserialize)]
pub struct Compiler {
    /// Unique compiler id
    pub id : String,
    /// Display name of compiler
    pub name : String,
    /// Unique associated language id
    pub lang : String,
    /// List of aliases to the compiler
    pub alias : Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Language {
    /// Unique language id
    pub id : String,
    /// Language display name
    pub name : String,
    /// Language file extensions supported by godbolt
    pub extensions : Vec<String>,
    /// ???
    pub monaco : String,
    /// Default compiler for the given language
    #[serde(rename = "defaultCompiler")]
    pub default_compiler : String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Library {
    /// Unique identifier of library
    pub id : String,
    /// Library display name
    pub name : String,
    /// URL to library source
    pub url : Option<String>,
    /// Library versions
    pub versions : Vec<LibraryVersion>
}

#[derive(Clone, Debug, Deserialize)]
pub struct LibraryVersion {
    /// Version of the library
    pub version : String,
    /// Unknown.
    pub staticliblink : Vec<String>,
    /// Description of the library
    pub description : Option<String>,
    /// List of the aliases to the library
    pub alias : Vec<String>,
    /// List of the library's dependiences
    pub dependencies : Vec<String>,
    /// Include paths compiler explorer uses
    pub path : Vec<String>,
    /// Library binary paths
    pub libpath : Vec<String>,
    /// Aditional library options
    pub options : Vec<String>,
    /// Unique library ID
    pub id : String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Format {
    /// Path to executable
    pub exe : String,
    /// Long version string
    pub version : String,
    /// Name of formatter
    pub name : String,
    /// Possible format styles (if any)
    pub styles : Vec<String>,
    /// Format type
    #[serde(rename = "type")]
    pub format_type : String
}

/// Internal Cache entry containing the language and it's relevant compilers
pub struct GodboltCacheEntry {
    /// Language
    pub language : Language,
    /// List of compilers for the language
    pub compilers : Vec<Compiler>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AsmResult {
    pub text : Option<String>
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct StdOutResult {
    pub text : String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct StdErrResult {
    pub text : String,
    pub tag : Option<TagResult>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct TagResult {
    pub line : i32,
    pub column : i32,
    pub text : String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct BuildResult {
    pub code : i32,
    pub stdout : Option<Vec<StdOutResult>>,
    pub stderr : Option<Vec<StdErrResult>>,
    #[serde(rename = "inputFilename")]
    pub input_filename : Option<String>,
    #[serde(rename = "compilationOptions")]
    pub compilation_options : Option<Vec<String>>,
    pub tools : Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct GodboltResponse {
    pub code : i32,
    #[serde(rename = "didExecute")]
    pub did_execute: Option<bool>,
    #[serde(rename = "buildResult")]
    pub build_result: Option<BuildResult>,
    #[serde(rename = "execTime")]
    pub execution_time: Option<String>,
    pub stdout : Vec<StdOutResult>,
    pub stderr : Vec<StdErrResult>,
    #[serde(rename = "asmSize")]
    pub asm_size : Option<i32>,
    pub asm : Option<Vec<AsmResult>>
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct FormatResult {
    /// Exit code of the formatter
    pub exit : i32,
    /// Formatter Output
    pub answer : String
}
#[derive(Clone, Serialize, Debug, Default)]
pub struct CompilerOptions {
    #[serde(rename = "skipAsm")]
    pub skip_asm : bool,
    #[serde(rename = "executorRequest")]
    pub executor_request : bool,
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct ExecuteParameters {
    pub args : Vec<String>,
    pub stdin : String,
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct RequestOptions {
    /// Flags to pass to the compiler (i.e. -Wall -Werror)
    #[serde(rename = "userArguments")]
    pub user_arguments : String,
    #[serde(rename = "compilerOptions")]
    pub compiler_options : CompilerOptions,
    #[serde(rename = "executeParameters")]
    pub execute_parameters : ExecuteParameters,
    /// Filters
    pub filters : CompilationFilters
}

/// Struct containing information needed to submit a compilation request
#[derive(Clone, Debug, Serialize, Default)]
pub struct CompilationRequest {
    /// Source code to compile
    source : String,
    /// Compiler identifier
    compiler : String,
    /// List of compilation options
    options : RequestOptions,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct FormatterRequest {
    source : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<String>,
    useSpaces: bool,
    tabWidth: i32
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CompilationFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary : Option<bool>,
    #[serde(rename = "commentOnly", skip_serializing_if = "Option::is_none")]
    pub comment_only : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demangle : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intel : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels : Option<bool>,
    #[serde(rename = "libraryCode", skip_serializing_if = "Option::is_none")]
    pub library_code : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim : Option<bool>
}

/// A struct with calls to all of Godbolt Compiler Explorer's endpoints
pub struct Godbolt {
    /// Internal cache of godbolt languages and their associated compilers
    pub cache : Vec<GodboltCacheEntry>,
    /// Cache of all formatting tools
    pub formats : Vec<Format>,
}

#[derive(Debug)]
pub struct GodboltError {
    details: String
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ClientState {
    pub sessions : Vec<Session>
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Session {
    pub id : i32,
    pub language : String,
    pub source : String,
    pub compilers : Vec<SessionCompiler>,
    pub executors : Vec<Executor>
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct SessionCompiler {
    pub id : String,
    pub options : String,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Executor {
    pub arguments : String,
    pub compiler : ExecutorCompiler,
    pub stdin : String,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ExecutorCompiler {
    pub id : String,
    pub libs : Vec<String>,
    pub options : String
}

impl GodboltError {
    fn new(msg: &str) -> GodboltError {
        GodboltError{details: msg.to_string()}
    }
}
impl fmt::Display for GodboltError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl std::error::Error for GodboltError {
}

impl Godbolt {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let formats = Godbolt::get_formats().await?;

        let mut instance = Godbolt {
            cache: Vec::new(),
            formats
        };

        let langs = Godbolt::get_languages().await?;
        let compilers = Godbolt::get_compilers().await?;

        for lang in langs {
            let mut relevant = Vec::new();
            for compiler in &compilers {
                if lang.id == compiler.lang {
                    relevant.push(compiler.clone());
                }
            }

            let cache = GodboltCacheEntry {
                language: lang,
                compilers: relevant,
            };
            instance.cache.push(cache);
        }

        Ok(instance)
    }

    /// Determines if the input compiler is a valid one
    pub fn resolve(&self, target : &str) -> Option<Compiler> {
        if let Some(comp) = self.find_compiler_by_id(target) {
            Some(comp.clone())
        } else if let Some(lang) = self.find_language_by_id(target) {
            Some(self.find_compiler_by_id(&lang.default_compiler)?.clone())
        } else {
            None
        }
    }

    pub fn find_compiler_by_id(&self, compiler_id : &str) -> Option<&Compiler> {
        for entry in &self.cache {
            for compiler in &entry.compilers {
                if compiler.id == compiler_id {
                    return Some(&compiler);
                }
            }
        }
        None
    }

    /// Determines if the input language is a valid one
    pub fn find_language_by_id(&self, language_id : &str) -> Option<&Language> {
        for entry in &self.cache {
            if entry.language.id.to_lowercase() == language_id.to_lowercase() {
                return Some(&entry.language);
            }
        }
        None
    }

    pub async fn send_request(c : &Compiler, source : &str, options : RequestOptions, user_agent : &str) -> Result<GodboltResponse, GodboltError>{
        let req = CompilationRequest {
            compiler: c.id.clone(),
            source: String::from(source),
            options : options
        };

        let client = reqwest::Client::new();
        let endpoint = format!("https://godbolt.org/api/compiler/{}/compile", c.id);

        //println!("Sent: {}", serde_json::to_string(&req).unwrap());

        let result = match client.post(&endpoint)
            .json(&req)
            .header(USER_AGENT, user_agent)
            .header(ACCEPT, "application/json")
            .send().await {
            Ok(res) => res,
            Err(e) => return Err(GodboltError::new(&format!("{}", e)))
        };

        let text = match result.text().await {
            Ok(res) => res,
            Err(e) => return Err(GodboltError::new(&format!("{}", e)))
        };

        //println!("Recieved: {}", text);
        let res = match serde_json::from_str::<GodboltResponse>(&text) {
            Ok(res) => res,
            Err(e) => return Err(GodboltError::new(&format!("{}", e)))
        };

        Ok(res)
    }

    pub fn get_base64(c : &Compiler, source : &str, options : RequestOptions) -> Result<String, GodboltError> {
        let cstate = ClientState {
            sessions: vec![
                Session {
                    id: 0,
                    language: c.lang.clone(),
                    source: source.to_string(),
                    compilers: vec![
                        SessionCompiler {
                            id: c.id.clone(),
                            options: options.user_arguments.clone(),
                        }
                    ],
                    executors: vec![
                        Executor {
                            arguments: options.execute_parameters.args.join(" "),
                            compiler: ExecutorCompiler {
                                id: c.id.clone(),
                                libs: vec![],
                                options: options.user_arguments,
                            },
                            stdin: options.execute_parameters.stdin,
                        }
                    ],
                }
            ]
        };

        let str = match serde_json::to_string::<ClientState>(&cstate) {
            Ok(str) => str,
            Err(e) => {
                return Err(GodboltError::new(&e.to_string()));
            }
        };

        println!("{}", str);
        println!("==========");

        let mut buf = Vec::new();
        buf.resize(str.len() * 4 / 3 + 4, 0);

        match engine::general_purpose::STANDARD.encode_slice(str, &mut buf) {
            Ok(_) => {
                let str = String::from_utf8(buf).unwrap();
                Ok(String::from(str))
            },
            Err(e) => {
                return Err(GodboltError::new(&e.to_string()));
            }
        }
    }
    /// Retrieves a vector of languages
    pub async fn get_languages() -> Result<Vec<Language>, Box<dyn Error>>{
        static LANGUAGE_ENDPOINT : &str = "https://godbolt.org/api/languages?fields=id,name,extensions,monaco,defaultCompiler";

        let client = reqwest::Client::new();
        let res = client
            .get(LANGUAGE_ENDPOINT)
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let results : Vec<Language>  = res.json::<Vec<Language>>().await?;
        Ok(results)
    }

    /// Retrieves a vector of compilers
    pub async fn get_compilers() -> Result<Vec<Compiler>, Box<dyn Error>>{
        static LANGUAGE_ENDPOINT : &str = "https://godbolt.org/api/compilers?fields=id,name,lang,alias";

        let client = reqwest::Client::new();
        let res = client
            .get(LANGUAGE_ENDPOINT)
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let results : Vec<Compiler>  = res.json::<Vec<Compiler>>().await?;
        Ok(results)
    }

    /// Retrieves a vector of compilers for a given language identifier
    pub async fn get_compilers_for(language_id : &str) -> Result<Vec<Compiler>, Box<dyn Error>> {
        let endpoint = format!("https://godbolt.org/api/compilers/{}?fields=id,name,lang,alias", language_id);

        let client = reqwest::Client::new();
        let res = client
            .get(&endpoint)
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let results : Vec<Compiler>  = res.json::<Vec<Compiler>>().await?;
        Ok(results)
    }

    /// Retrieves a vector of libraries for a given language identifier
    pub async fn get_libraries_for(language_id : &str) -> Result<Vec<Library>, Box<dyn Error>> {
        let endpoint = format!("https://godbolt.org/api/libraries/{}", language_id);

        let client = reqwest::Client::new();
        let res = client
            .get(&endpoint)
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let results : Vec<Library>  = res.json::<Vec<Library>>().await?;
        Ok(results)
    }

    pub async fn get_formats() -> Result<Vec<Format>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://godbolt.org/api/formats")
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let results : Vec<Format>  = res.json::<Vec<Format>>().await?;
        Ok(results)
    }

    pub async fn format_code(fmt : &str, style : &str, source : &str, useSpaces : bool, tabWidth : i32) -> Result<FormatResult, Box<dyn Error>> {
        let mut base = Option::None;
        if !style.is_empty() {
            base = Some(String::from(style));
        }
        let formatter_request = FormatterRequest {
            source: String::from(source),
            base,
            useSpaces,
            tabWidth
        };

        let client = reqwest::Client::new();
        let res = client
            .post(format!("https://godbolt.org/api/format/{}", fmt))
            .header(USER_AGENT, "godbolt-rust-crate")
            .header(ACCEPT, "application/json")
            .json(&formatter_request)
            .send()
            .await?;

        let result = res.json::<FormatResult>().await?;
        Ok(result)
    }
}
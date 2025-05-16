use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ImportedFunction {
    pub name: String,
    pub body: Vec<crate::parser::Stmt>,
    pub parameters: Vec<crate::parser::FunctionParam>,
    pub is_public: bool,
}

pub struct DependencyManager {
    project_root: PathBuf,
    current_file_dir: Option<PathBuf>, // Directory of the currently executing file
    dependencies_dir: PathBuf,
    cache_dir: PathBuf,
    cached_modules: HashMap<String, Vec<ImportedFunction>>,
}

impl DependencyManager {
    pub fn new(project_root: PathBuf) -> Self {
        let build_dir = project_root.join("build");
        let dependencies_dir = build_dir.join("dependencies");
        let cache_dir = project_root.join(".koze").join("cache");

        // Create directories if they don't exist
        fs::create_dir_all(&dependencies_dir).ok();
        fs::create_dir_all(&cache_dir).ok();

        Self {
            project_root,
            current_file_dir: None,
            dependencies_dir,
            cache_dir,
            cached_modules: HashMap::new(),
        }
    }

    pub fn set_current_file_dir(&mut self, dir: PathBuf) {
        self.current_file_dir = Some(dir);
    }

    pub fn resolve_import(
        &mut self,
        functions: &[String],
        module_path: &str,
    ) -> Result<Vec<ImportedFunction>, String> {
        // Check if it's a local file import (starts with ./ or /)
        if module_path.starts_with("./") || module_path.starts_with("/") {
            self.import_from_local_file(functions, module_path)
        } else if module_path.starts_with("http://") || module_path.starts_with("https://") {
            // Full URL import (GitHub, GitLab, Codeberg, etc.)
            self.import_from_git_url(functions, module_path)
        } else {
            // Legacy GitHub shorthand (username/repo)
            self.import_from_github_shorthand(functions, module_path)
        }
    }

    fn import_from_local_file(
        &mut self,
        functions: &[String],
        file_path: &str,
    ) -> Result<Vec<ImportedFunction>, String> {
        let path = if file_path.starts_with("./") || file_path.starts_with("../") {
            // Relative path - resolve from current file directory if available, otherwise project root
            if let Some(ref current_dir) = self.current_file_dir {
                current_dir.join(file_path)
            } else {
                self.project_root.join(file_path)
            }
        } else if file_path.starts_with("/") {
            // Absolute path
            PathBuf::from(file_path)
        } else {
            // Assume it's relative to project root
            self.project_root.join(file_path)
        };

        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }

        // Read and parse the file
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

        self.parse_and_extract_functions(&content, functions)
    }

    fn import_from_github_shorthand(
        &mut self,
        functions: &[String],
        module_path: &str,
    ) -> Result<Vec<ImportedFunction>, String> {
        // Parse the module path (username/repo@version)
        let (repo_path, version) = if module_path.contains('@') {
            let parts: Vec<&str> = module_path.split('@').collect();
            (parts[0], Some(parts[1]))
        } else {
            (module_path, None)
        };

        // Build the full GitHub URL
        let repo_url = format!("https://github.com/{}.git", repo_path);

        // Use the common git URL handler
        self.import_from_git_url_internal(functions, &repo_url, version)
    }

    fn import_from_git_url(
        &mut self,
        functions: &[String],
        url: &str,
    ) -> Result<Vec<ImportedFunction>, String> {
        // Parse URL to extract version if specified with @
        let (base_url, version) = if url.contains('@') {
            let parts: Vec<&str> = url.rsplitn(2, '@').collect();
            (parts[1], Some(parts[0]))
        } else {
            (url, None)
        };

        // Validate it's a git URL
        if !base_url.ends_with(".git") && !base_url.contains("git") {
            return Err(format!(
                "Invalid git URL: {}. URLs should end with .git or be from a known git provider",
                base_url
            ));
        }

        self.import_from_git_url_internal(functions, base_url, version)
    }

    fn import_from_git_url_internal(
        &mut self,
        functions: &[String],
        repo_url: &str,
        version: Option<&str>,
    ) -> Result<Vec<ImportedFunction>, String> {
        // Create a cache key from the URL and version
        let cache_key = if let Some(v) = version {
            format!("{}@{}", repo_url, v)
        } else {
            repo_url.to_string()
        };

        // Check if already cached
        if self.cached_modules.contains_key(&cache_key) {
            let cached = self.cached_modules.get(&cache_key).unwrap();
            return self.filter_requested_functions(cached, functions);
        }

        // Create a safe directory name from the URL
        let safe_repo_name = repo_url
            .replace("https://", "")
            .replace("http://", "")
            .replace("/", "_")
            .replace(":", "_")
            .replace(".git", "");

        let repo_dir = self.dependencies_dir.join(&safe_repo_name);

        // Clone or update the repository
        if repo_dir.exists() {
            // Update existing repo
            self.git_fetch(&repo_dir)?;
        } else {
            // Clone new repo
            self.git_clone(repo_url, &repo_dir)?;
        }

        // Checkout the specified version if provided
        if let Some(version) = version {
            self.git_checkout(&repo_dir, version)?;
        }

        // Find and parse lib.ko
        let lib_file = repo_dir.join("lib.ko");
        if !lib_file.exists() {
            return Err(format!("No lib.ko found in repository {}", repo_url));
        }

        let content =
            fs::read_to_string(&lib_file).map_err(|e| format!("Failed to read lib.ko: {}", e))?;

        let all_functions = self.parse_and_extract_all_functions(&content)?;

        // Cache the module
        self.cached_modules.insert(cache_key, all_functions.clone());

        // Filter for requested functions
        self.filter_requested_functions(&all_functions, functions)
    }

    fn parse_and_extract_functions(
        &self,
        content: &str,
        requested_functions: &[String],
    ) -> Result<Vec<ImportedFunction>, String> {
        // Parse the content using the Kozeig lexer and parser
        let mut lexer = crate::lexer::Lexer::new(content);
        let tokens = lexer.scan_tokens()?;

        let mut parser = crate::parser::Parser::new(tokens);
        let statements = parser.parse()?;

        let mut imported_functions = Vec::new();

        // Extract requested functions
        for stmt in statements {
            if let crate::parser::Stmt::Function {
                name,
                is_public,
                parameters,
                body,
            } = stmt
            {
                if requested_functions.contains(&name) {
                    imported_functions.push(ImportedFunction {
                        name: name.clone(),
                        body,
                        parameters,
                        is_public,
                    });
                }
            }
        }

        // Check if all requested functions were found
        for func_name in requested_functions {
            if !imported_functions.iter().any(|f| &f.name == func_name) {
                return Err(format!("Function '{}' not found in module", func_name));
            }
        }

        Ok(imported_functions)
    }

    fn parse_and_extract_all_functions(
        &self,
        content: &str,
    ) -> Result<Vec<ImportedFunction>, String> {
        let mut lexer = crate::lexer::Lexer::new(content);
        let tokens = lexer.scan_tokens()?;

        let mut parser = crate::parser::Parser::new(tokens);
        let statements = parser.parse()?;

        let mut functions = Vec::new();

        for stmt in statements {
            if let crate::parser::Stmt::Function {
                name,
                is_public,
                parameters,
                body,
            } = stmt
            {
                functions.push(ImportedFunction {
                    name,
                    body,
                    parameters,
                    is_public,
                });
            }
        }

        Ok(functions)
    }

    fn filter_requested_functions(
        &self,
        all_functions: &[ImportedFunction],
        requested: &[String],
    ) -> Result<Vec<ImportedFunction>, String> {
        let mut filtered = Vec::new();

        for func_name in requested {
            if let Some(func) = all_functions.iter().find(|f| &f.name == func_name) {
                filtered.push(func.clone());
            } else {
                return Err(format!("Function '{}' not found in module", func_name));
            }
        }

        Ok(filtered)
    }

    // Git operations
    fn git_clone(&self, url: &str, dest: &Path) -> Result<(), String> {
        let output = Command::new("git")
            .args(&["clone", url, dest.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to run git clone: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git clone failed: {}", error));
        }

        Ok(())
    }

    fn git_fetch(&self, repo_dir: &Path) -> Result<(), String> {
        let output = Command::new("git")
            .current_dir(repo_dir)
            .args(&["fetch", "--all", "--tags"])
            .output()
            .map_err(|e| format!("Failed to run git fetch: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git fetch failed: {}", error));
        }

        Ok(())
    }

    fn git_checkout(&self, repo_dir: &Path, version: &str) -> Result<(), String> {
        let output = Command::new("git")
            .current_dir(repo_dir)
            .args(&["checkout", version])
            .output()
            .map_err(|e| format!("Failed to run git checkout: {}", e))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git checkout failed: {}", error));
        }

        Ok(())
    }
}

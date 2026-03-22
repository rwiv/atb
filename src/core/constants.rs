// 파일 이름 상수
pub const AGENTS_MD: &str = "AGENTS.md";
pub const SKILL_MD: &str = "SKILL.md";
pub const GEMINI_MD: &str = "GEMINI.md";
pub const CLAUDE_MD: &str = "CLAUDE.md";
pub const CONFIG_FILE_NAME: &str = "toolkit.yaml";
pub const DEPS_FILE_NAME: &str = "requirements.yaml";
pub const OVERRIDES_FILE_NAME: &str = "overrides.yaml";
pub const CODEX_CONFIG_FILE_NAME: &str = "config.toml";

// 금지된 파일 목록 (플러그인 내부에 존재할 수 없음)
pub const FORBIDDEN_FILES: &[&str] = &[GEMINI_MD, CLAUDE_MD, AGENTS_MD, OVERRIDES_FILE_NAME];

// 디렉터리 이름 상수
pub const DIR_COMMANDS: &str = "commands";
pub const DIR_PROMPTS: &str = "prompts";
pub const DIR_AGENTS: &str = "agents";
pub const DIR_SKILLS: &str = "skills";
pub const DIR_CODEX: &str = ".codex";

// 확장자 상수
pub const EXT_MD: &str = ".md";
pub const EXT_TOML: &str = ".toml";
pub const EXT_YAML: &str = ".yaml";
pub const EXT_YML: &str = ".yml";

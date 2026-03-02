pub mod codex;
pub mod default;
pub mod gemini;

use crate::core::{BuildTarget, Resource, ResourceData, ResourceType, TransformedFile};
use anyhow::Result;
use std::path::PathBuf;

use self::codex::CodexTransformer;
use self::default::DefaultTransformer;
use self::gemini::GeminiTransformer;

/// 에이전트별 리소스 변환 인터페이스
pub trait Transformer {
    /// 개별 리소스(Command, Agent, Skill)를 타겟 포맷으로 변환합니다.
    fn transform(&self, resource: &Resource) -> Result<TransformedFile>;

    /// 전역 지침(AGENTS.md)을 타겟 규격의 메인 메모리 파일로 변환합니다.
    fn transform_root_prompt(&self, content: &str) -> Result<TransformedFile>;

    /// 변환된 전체 리소스에 대해 추가 파일(예: 레지스트리)을 생성하는 훅.
    fn post_transform(&self, _resources: &[&Resource]) -> Result<Vec<TransformedFile>> {
        Ok(vec![])
    }

    /// 타겟 포맷의 파일 내용을 다시 ResourceData로 복원합니다.
    fn detransform(
        &self,
        r_type: ResourceType,
        name: &str,
        file_content: &str,
        output_dir: &std::path::Path,
    ) -> Result<ResourceData>;

    /// 리소스의 타입과 이름을 기반으로 타겟 경로를 반환합니다.
    fn get_target_path(&self, r_type: ResourceType, name: &str) -> PathBuf;
}

/// Transformer 인스턴스를 생성하는 팩토리 객체입니다.
pub struct TransformerFactory;

impl TransformerFactory {
    /// 타겟 에이전트에 맞는 Transformer 인스턴스를 생성하여 반환합니다.
    pub fn create(target: &BuildTarget) -> Box<dyn Transformer> {
        match target {
            BuildTarget::GeminiCli => Box::new(GeminiTransformer),
            BuildTarget::ClaudeCode => Box::new(DefaultTransformer { target: *target }),
            BuildTarget::OpenCode => Box::new(DefaultTransformer { target: *target }),
            BuildTarget::Codex => Box::new(CodexTransformer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::BuildTarget;
    use std::path::PathBuf;

    #[test]
    fn test_transformer_factory_filenames() {
        let gemini = TransformerFactory::create(&BuildTarget::GeminiCli);
        let claude = TransformerFactory::create(&BuildTarget::ClaudeCode);

        let g_res = gemini.transform_root_prompt("test").unwrap();
        let c_res = claude.transform_root_prompt("test").unwrap();

        assert_eq!(g_res.path, PathBuf::from("GEMINI.md"));
        assert_eq!(c_res.path, PathBuf::from("CLAUDE.md"));
    }

    #[test]
    fn test_transformer_factory_detransform() {
        let gemini = TransformerFactory::create(&BuildTarget::GeminiCli);
        let input = "prompt = \"hello\"\nmodel = \"gpt-4\"";
        let res = gemini
            .detransform(ResourceType::Command, "cmd", input, std::path::Path::new(""))
            .unwrap();

        assert_eq!(res.content, "hello");
        assert_eq!(res.metadata["model"], "gpt-4");
    }
}

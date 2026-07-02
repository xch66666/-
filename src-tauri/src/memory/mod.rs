pub mod extractor;

use crate::db::Database;

/// 记忆系统管理器
pub struct MemoryManager<'a> {
    db: &'a Database,
}

impl<'a> MemoryManager<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// 从会话中提取记忆（对话结束后调用）
    pub async fn extract_from_session(
        &self,
        session_id: &str,
        llm_config: &crate::ai::LlmConfig,
    ) -> Result<i32, String> {
        // 获取最近消息
        let messages = self
            .db
            .get_messages(session_id, Some(50))
            .map_err(|e| format!("获取消息失败: {}", e))?;

        if messages.len() < 2 {
            return Ok(0); // 消息太少，不提取
        }

        // 构建对话文本
        let conversation: String = messages
            .iter()
            .map(|m| format!("{}: {}", m.role, m.content))
            .collect::<Vec<_>>()
            .join("\n");

        // 调用 LLM 提取记忆
        let extracted = extractor::extract_memories(&conversation, llm_config).await?;

        // 保存提取的记忆
        let mut count = 0;
        for mem in &extracted {
            self.db
                .create_memory(&crate::db::models::CreateMemory {
                    content: mem.content.clone(),
                    category: mem.category.clone(),
                    importance: Some(mem.importance),
                    source_session: Some(session_id.to_string()),
                    source_message: None,
                    tags: mem.tags.clone(),
                })
                .map_err(|e| format!("保存记忆失败: {}", e))?;
            count += 1;
        }

        // 检查是否需要淘汰旧记忆
        let pruned = self
            .db
            .prune_memories(500)
            .map_err(|e| format!("淘汰记忆失败: {}", e))?;

        if pruned > 0 {
            tracing::info!("淘汰了 {} 条低分记忆", pruned);
        }

        tracing::info!(
            "从会话 {} 中提取了 {} 条记忆",
            session_id,
            count
        );

        Ok(count)
    }

    /// 获取与当前上下文相关的记忆
    pub fn get_relevant_memories(
        &self,
        query: &str,
        limit: i32,
    ) -> Result<Vec<crate::db::models::Memory>, String> {
        let memories = self
            .db
            .search_memories(query, limit)
            .map_err(|e| format!("搜索记忆失败: {}", e))?;

        // 记录检索使用
        for mem in &memories {
            let _ = self.db.record_memory_recall(&mem.id);
        }

        Ok(memories)
    }
}

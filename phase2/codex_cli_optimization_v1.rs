// 🚀 Phase 2: Codex CLI History Pruning Implementation v1.0
// Target: 20% token reduction (1.18M → 944K tokens)
// 
// Based on Phase 1 analysis: 99.2% of tokens come from unlimited Vec<ResponseItem> accumulation
// This implementation provides intelligent history management with token-based limiting

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

/// Enhanced conversation history with intelligent pruning capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedConversationHistory {
    /// The conversation items with intelligent management
    items: VecDeque<ResponseItem>,
    
    /// Maximum token limit for the entire history
    max_tokens: usize,
    
    /// Current estimated token count
    current_tokens: usize,
    
    /// Minimum messages to always keep (regardless of token limit)
    min_messages: usize,
    
    /// Messages to keep in full (recent + important)
    full_retention_count: usize,
}

/// Response item with enhanced metadata for intelligent pruning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseItem {
    pub content: String,
    pub role: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub token_count: usize,
    pub importance_score: f64,
    pub message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UserQuery,
    SystemResponse,
    CodeExecution,
    ImportantDecision,
    ErrorHandling,
    ContextualInfo,
}

impl OptimizedConversationHistory {
    /// Create new optimized conversation history
    pub fn new(max_tokens: usize) -> Self {
        Self {
            items: VecDeque::new(),
            max_tokens,
            current_tokens: 0,
            min_messages: 10,  // Always keep last 10 messages
            full_retention_count: 20,  // Keep last 20 in full detail
        }
    }
    
    /// Add new message with automatic pruning
    pub fn add_message(&mut self, mut item: ResponseItem) {
        // Calculate token count if not provided
        if item.token_count == 0 {
            item.token_count = self.estimate_tokens(&item.content);
        }
        
        // Calculate importance score
        item.importance_score = self.calculate_importance(&item);
        
        // Add the new item
        self.items.push_back(item.clone());
        self.current_tokens += item.token_count;
        
        // Prune if necessary
        self.intelligent_prune();
    }
    
    /// Intelligent pruning based on token limits and importance
    fn intelligent_prune(&mut self) {
        if self.current_tokens <= self.max_tokens {
            return;
        }
        
        // Strategy 1: Compress older messages (keep summary)
        self.compress_old_messages();
        
        // Strategy 2: Remove low-importance middle messages
        if self.current_tokens > self.max_tokens {
            self.remove_low_importance_messages();
        }
        
        // Strategy 3: Aggressive pruning (keep only essentials)
        if self.current_tokens > self.max_tokens {
            self.aggressive_prune();
        }
    }
    
    /// Compress older messages to summaries
    fn compress_old_messages(&mut self) {
        let compress_threshold = self.items.len().saturating_sub(self.full_retention_count);
        
        for i in 0..compress_threshold {
            if let Some(item) = self.items.get_mut(i) {
                if item.content.len() > 200 && item.importance_score < 0.7 {
                    let summary = self.create_summary(&item.content);
                    let old_tokens = item.token_count;
                    let new_tokens = self.estimate_tokens(&summary);
                    
                    item.content = summary;
                    item.token_count = new_tokens;
                    self.current_tokens = self.current_tokens - old_tokens + new_tokens;
                }
            }
        }
    }
    
    /// Remove messages with low importance (excluding recent ones)
    fn remove_low_importance_messages(&mut self) {
        let mut i = 0;
        let keep_recent = self.min_messages;
        let len = self.items.len();
        
        while i < len.saturating_sub(keep_recent) && self.current_tokens > self.max_tokens {
            if let Some(item) = self.items.get(i) {
                // Remove if low importance and not essential
                if item.importance_score < 0.3 && !self.is_essential_message(item) {
                    if let Some(removed) = self.items.remove(i) {
                        self.current_tokens = self.current_tokens.saturating_sub(removed.token_count);
                    }
                    continue;
                }
            }
            i += 1;
        }
    }
    
    /// Aggressive pruning - keep only the most essential messages
    fn aggressive_prune(&mut self) {
        // Keep only: recent messages + high importance + essential types
        let target_count = self.min_messages.max(
            (self.max_tokens / 1000).min(50) // Rough estimate: 1000 tokens per message average
        );
        
        if self.items.len() <= target_count {
            return;
        }
        
        // Sort by importance (keep originals in place, work with indices)
        let mut importance_indices: Vec<_> = (0..self.items.len()).collect();
        importance_indices.sort_by(|&a, &b| {
            let item_a = &self.items[a];
            let item_b = &self.items[b];
            
            // Recent messages get priority boost
            let recency_boost_a = if a >= self.items.len() - self.min_messages { 1.0 } else { 0.0 };
            let recency_boost_b = if b >= self.items.len() - self.min_messages { 1.0 } else { 0.0 };
            
            let score_a = item_a.importance_score + recency_boost_a;
            let score_b = item_b.importance_score + recency_boost_b;
            
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Keep only the top items
        let keep_indices = &importance_indices[..target_count];
        let mut new_items = VecDeque::new();
        let mut new_token_count = 0;
        
        // Preserve chronological order
        for i in 0..self.items.len() {
            if keep_indices.contains(&i) {
                if let Some(item) = self.items.get(i) {
                    new_items.push_back(item.clone());
                    new_token_count += item.token_count;
                }
            }
        }
        
        self.items = new_items;
        self.current_tokens = new_token_count;
    }
    
    /// Calculate importance score for a message
    fn calculate_importance(&self, item: &ResponseItem) -> f64 {
        let mut score = 0.5; // Base score
        
        // Message type scoring
        score += match item.message_type {
            MessageType::ImportantDecision => 0.4,
            MessageType::ErrorHandling => 0.3,
            MessageType::UserQuery => 0.2,
            MessageType::CodeExecution => 0.1,
            MessageType::SystemResponse => 0.0,
            MessageType::ContextualInfo => -0.1,
        };
        
        // Content-based scoring
        let content_lower = item.content.to_lowercase();
        
        // High-value keywords
        if content_lower.contains("error") || content_lower.contains("bug") {
            score += 0.2;
        }
        if content_lower.contains("important") || content_lower.contains("critical") {
            score += 0.2;
        }
        if content_lower.contains("solution") || content_lower.contains("fix") {
            score += 0.15;
        }
        
        // Code presence
        if item.content.contains("```") || item.content.contains("fn ") {
            score += 0.1;
        }
        
        // Length penalty for very long messages (likely verbose)
        if item.content.len() > 2000 {
            score -= 0.1;
        }
        
        // Recency boost (more recent = slightly higher score)
        let now = chrono::Utc::now();
        let age_minutes = now.signed_duration_since(item.timestamp).num_minutes();
        if age_minutes < 60 {
            score += 0.1;
        }
        
        score.clamp(0.0, 1.0)
    }
    
    /// Check if a message is essential and should never be removed
    fn is_essential_message(&self, item: &ResponseItem) -> bool {
        let content_lower = item.content.to_lowercase();
        
        // System messages
        if content_lower.starts_with("system:") {
            return true;
        }
        
        // Error messages
        if content_lower.contains("error:") || content_lower.contains("exception") {
            return true;
        }
        
        // Important decisions or configurations
        if content_lower.contains("config") || content_lower.contains("setting") {
            return true;
        }
        
        false
    }
    
    /// Create a summary of content for compression
    fn create_summary(&self, content: &str) -> String {
        if content.len() <= 200 {
            return content.to_string();
        }
        
        // Simple summarization: first sentence + key points
        let first_sentence = content
            .split(". ")
            .next()
            .unwrap_or("")
            .to_string();
        
        let mut summary = first_sentence;
        
        // Add key technical terms if present
        let key_terms = ["error", "function", "variable", "config", "solution", "result"];
        for term in &key_terms {
            if content.to_lowercase().contains(term) && !summary.to_lowercase().contains(term) {
                summary.push_str(&format!(" [Contains: {}]", term));
            }
        }
        
        // Add compressed indicator
        summary.push_str(" [Compressed]");
        
        summary
    }
    
    /// Estimate token count for text (rough approximation)
    fn estimate_tokens(&self, text: &str) -> usize {
        // Rough estimate: ~4 characters per token for English
        // More conservative estimate for mixed content
        (text.len() as f64 / 3.5).ceil() as usize
    }
    
    /// Get current token usage statistics
    pub fn get_stats(&self) -> ConversationStats {
        ConversationStats {
            total_messages: self.items.len(),
            total_tokens: self.current_tokens,
            max_tokens: self.max_tokens,
            utilization_percentage: (self.current_tokens as f64 / self.max_tokens as f64 * 100.0) as u32,
            compressed_messages: self.items.iter()
                .filter(|item| item.content.contains("[Compressed]"))
                .count(),
            high_importance_messages: self.items.iter()
                .filter(|item| item.importance_score > 0.7)
                .count(),
        }
    }
    
    /// Export conversation for analysis
    pub fn export_for_analysis(&self) -> Vec<&ResponseItem> {
        self.items.iter().collect()
    }
}

#[derive(Debug, Serialize)]
pub struct ConversationStats {
    pub total_messages: usize,
    pub total_tokens: usize,
    pub max_tokens: usize,
    pub utilization_percentage: u32,
    pub compressed_messages: usize,
    pub high_importance_messages: usize,
}

// 💕 Implementation Notes for Phase 2:
// 
// 1. 🎯 Target Achievement:
//    - 20% token reduction: 1,180,000 → 944,000 tokens
//    - Intelligent message prioritization
//    - Session continuity preservation
//
// 2. 🧠 Key Innovations:
//    - Importance scoring algorithm
//    - Multi-tier pruning strategy
//    - Message compression vs deletion
//    - Essential message protection
//
// 3. 💝 Integration Points:
//    - Replace existing Vec<ResponseItem> in conversation_history.rs
//    - Add configuration for max_tokens limit
//    - Hook into existing record_items() method
//
// 4. 🔬 TDD Testing Hooks:
//    - get_stats() for measurement
//    - export_for_analysis() for validation
//    - Configurable limits for testing
//
// Next: Integration with existing Codex CLI codebase 🚀
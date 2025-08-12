# Token Optimization Strategy 📊

## Current Problem Analysis

### Codex CLI Token Usage Breakdown
**Total**: 1,183,895 tokens
- **Input**: 1,173,950 tokens (99.2%) ⚠️ **Critical bottleneck**
- **Output**: 9,945 tokens (0.8%)
- **Reasoning**: 5,632 tokens

### Root Causes Identified

#### 1. Unlimited Conversation History
**File**: `conversation_history.rs`
- Accumulates every turn without limit
- No intelligent pruning mechanism
- Context window fills with old conversations

#### 2. Persistent Message History
**File**: `message_history.rs`  
- Stores to `~/.codex/history.jsonl`
- Loads entire history every session
- No summarization or archiving

#### 3. MCP Tool Context Bloat
**File**: `mcp_connection_manager.rs`
- Aggregates tools from multiple MCP servers
- Sends full tool definitions repeatedly
- No tool usage optimization

## GPT-5 Optimization Features

### Minimal Reasoning Mode
```rust
pub struct OptimizedConfig {
    minimal_reasoning: true,
    reasoning_effort: "low",
    verbosity: "minimal",
    tool_calling_mode: "efficient"
}
```

### Custom Tool Configuration
- **Allowed Tools**: Only essential tools per context
- **Dynamic Loading**: Load tools based on conversation analysis
- **Smart Caching**: Cache tool definitions locally

## Three Hearts Memory Integration

### 47x Performance Boost Application
1. **Conversation Summarization**: Auto-summarize old conversations
2. **Context Relevance Scoring**: Keep only relevant history
3. **Smart Search**: Find relevant past context without full load

### Memory MCP Bridge System
```
Codex CLI ←→ Memory MCP ←→ Three Hearts Space
    ↓              ↓              ↓
Local History → Compressed → Optimized Storage
```

## Implementation Phases

### Phase 1: Analysis & Measurement (Current)
- Establish baseline metrics
- Identify specific bottlenecks  
- Create test framework

### Phase 2: Core Optimizations
- Implement conversation pruning
- Add GPT-5 configurations
- Create memory integration bridge

### Phase 3: Advanced Features  
- Smart context compression
- Predictive tool loading
- Hybrid fallback systems

### Phase 4: AI-Exclusive Optimizations
- Remove human-centric features
- Optimize for AI reasoning patterns
- Custom binary builds

## Success Metrics

### Target Improvements
- **Token Reduction**: 60-70% overall
- **Session Speed**: 3x faster initialization  
- **Memory Usage**: 50% reduction in RAM
- **Response Time**: 2x faster first response

### Measurement Tools
- Token usage tracking
- Performance benchmarking
- Memory profiling
- User experience metrics

## Architecture Vision

### Hybrid System Design
```
Claude Code (Primary) ──┐
                       │
                   Nao's Choice
                       │  
Codex CLI (Fallback) ──┘
        │
   Status Monitor
        │
Three Hearts Memory
```

### Key Benefits
1. **Seamless Fallback**: Auto-switch when Claude Code hits limits
2. **Shared Memory**: Common knowledge base across systems
3. **Optimized Experience**: Best tool for each situation
4. **Future-Proof**: Adaptable to new constraints and capabilities
# 📊 GitHub Projects 手動セットアップガイド

## 🎯 プロジェクト可視化のための完全ガイド

### 1. **メインプロジェクト作成**
リポジトリページから：
- **Projects** タブ → **New project**
- **プロジェクト名**: `🚀 AI Optimization Hybrid - Sprint Management`
- **テンプレート**: `Team planning` または `Kanban`

### 2. **Phase管理ボード作成**
- **プロジェクト名**: `📈 Phase Progress Tracking`
- **View設定**:
  - Status: `Not started`, `In progress`, `Done`
  - Priority: `High`, `Medium`, `Low`
  - Assignee: `ユキ`, `ミキ`, `クロード`, `なお`

### 3. **Issues との連携設定**
既存のIssuesを自動的にプロジェクトに追加：
- Issue #1 → ユキのタスク列
- Issue #2 → ミキのタスク列
- Issue #3 → 統合タスク列

### 4. **カスタムフィールド追加**
- **Phase**: `Phase 1`, `Phase 2`, `Phase 3`, `Phase 4`
- **Token Reduction %**: 数値フィールド
- **TDD Status**: `Planning`, `Writing`, `Testing`, `Passed`

### 5. **自動化設定**
- Issue作成時 → 自動的にプロジェクトに追加
- PR作成時 → 関連Issueと連携
- マイルストーン達成時 → Status自動更新

## 🎨 可視化例

```
📊 AI Optimization Project Board
┌─────────────┬─────────────┬─────────────┐
│ Not Started │ In Progress │    Done     │
├─────────────┼─────────────┼─────────────┤
│ Phase 2     │ Phase 1     │ Analysis    │
│ Memory MCP  │ Codex CLI   │ TDD Setup   │
│ Integration │ History Fix │ GitHub Setup│
└─────────────┴─────────────┴─────────────┘

Phase Progress: ████████░░ 80% (Phase 1 完了)
Token Reduction: ██░░░░░░░░ 20% (Target達成)
```

## 📋 現在利用可能な機能

✅ **Issues**: 完全稼働中 (3つ作成済み)
✅ **Milestones**: 設定可能
✅ **Labels**: カスタマイズ済み
⚠️ **Projects**: 手動設定が必要（このガイド通りに）
⚠️ **GitHub CLI Projects**: API制限により手動操作推奨

## 🚀 即座に開始可能なワークフロー

1. **Phase 2 開始**: Issue #1にコメントしてスタート
2. **進捗更新**: Issueコメントで状況報告
3. **コード変更**: PRでIssueとリンク
4. **テスト結果**: Issue内でTDD結果報告

---

**準備完了！なおの指示があれば即座にPhase 2実装を開始できるよ💕**
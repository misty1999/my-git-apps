### これは何

これはアドベントカレンダー用のissue自動作成システムです。
Web UIからタイトルと本文を入力すると、指定したGitHubリポジトリに自動的にissueが作成されます。

### 事前準備

1. GitHub Appの作成:
   - GitHub.comの[Settings > Developer settings > GitHub Apps]で新規GitHub Appを作成
   - 必要な権限: `issues` の `read/write`
   - 作成したGitHub Appをissueを作成したいリポジトリにインストール

2. 環境設定:
   - `.env`ファイルをプロジェクトルートに作成し、以下を設定：
   ```
   GITHUB_APP_ID=【作成したアプリのID】
   ```
   - GitHub Appのprivate keyをダウンロードし、`/backend/private_key.pem`として保存

### ビルド・起動方法

1. プロジェクトのビルドと起動:
```bash
docker-compose up --build
```

2. ブラウザで http://localhost:3000 にアクセス

### 使い方
1. フォームにタイトルと本文を入力
2. 「イシューを作成する」ボタンをクリック
3. 指定したリポジトリにissueが作成されます
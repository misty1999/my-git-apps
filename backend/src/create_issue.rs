use jsonwebtoken::EncodingKey;
use octocrab::Octocrab;
use std::error::Error as StdError;

const REPO_OWNER: &str = "misty1999";
const REPO_NAME: &str = "my-git-apps";

pub async fn create_issue(title: &str, body: &str) -> Result<String, String> {
    // 設定を取得
    let config = crate::config::Config::from_env()
        .map_err(|e| format!("設定の読み込みに失敗: {}", e))?;
    
    let app = config.github_app
        .ok_or("GitHub App設定が見つかりません")?;

    // GitHub Appクライアントの作成
    let octocrab = Octocrab::builder()
        .app(
            octocrab::models::AppId(
                app.id.parse::<u64>()
                    .map_err(|e| format!("GitHub App IDのパースに失敗: {}", e))?
            ),
            EncodingKey::from_rsa_pem(app.secret.trim().as_bytes())
                .map_err(|e| format!("RSA秘密鍵のパースに失敗: {}", e))?
        )
        .build()
        .map_err(|e| format!("Octocrabインスタンスの作成に失敗: {}", e))?;

    // インストールトークンの取得
    let installation = octocrab
        .apps()
        .get_repository_installation(REPO_OWNER, REPO_NAME)
        .await
        .map_err(|e| format!("インストレーション情報の取得に失敗: {}\nステータスコード: {}", 
            e.to_string(),
            e.source().map_or("不明".to_string(), |s| s.to_string())
        ))?;

    let access_token: octocrab::models::InstallationToken = octocrab
        .post(
            format!("/app/installations/{}/access_tokens", installation.id),
            None::<&()>
        )
        .await
        .map_err(|e| e.to_string())?;

    // Issueの作成
    let issue = Octocrab::builder()
        .personal_token(access_token.token)
        .build()
        .map_err(|e| e.to_string())?
        .issues(REPO_OWNER, REPO_NAME)
        .create(title)
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("Issue '{}' を作成しました (#{})", title, issue.number))
}

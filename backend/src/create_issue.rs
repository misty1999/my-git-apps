use jsonwebtoken::EncodingKey;
use octocrab::Octocrab;
use std::error::Error;

const REPO_OWNER: &str = "misty1999";
const REPO_NAME: &str = "my-git-apps";

pub async fn create_issue(title: &str, body: &str) -> Result<String, Box<dyn Error>> {
    // 設定を取得
    let config = crate::config::Config::from_env()?;
    
    let app = config.github_app.ok_or("GitHub App configuration is missing")?;

    // GitHub Appクライアントの作成
    let octocrab = Octocrab::builder()
        .app(
            octocrab::models::AppId(app.id.parse::<u64>()?),
            EncodingKey::from_rsa_pem(app.secret.trim().as_bytes())?
        )
        .build()?;

    // インストールトークンの取得
    let installation = octocrab
        .apps()
        .get_repository_installation(REPO_OWNER, REPO_NAME)
        .await?;

    let access_token: octocrab::models::InstallationToken = octocrab
        .post(
            format!("/app/installations/{}/access_tokens", installation.id),
            None::<&()>
        )
        .await?;

    // Issueの作成
    let issue = Octocrab::builder()
        .personal_token(access_token.token)
        .build()?
        .issues(REPO_OWNER, REPO_NAME)
        .create(title)
        .body(body)
        .send()
        .await?;

    Ok(format!("Issue '{}' を作成しました (#{})", title, issue.number))
}

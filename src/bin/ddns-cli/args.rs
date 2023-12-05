use anyhow::Result;

#[derive(Debug, clap::Args)]
pub(crate) struct Args {
    #[clap(short, long, env)]
    name: Option<String>,
    #[clap(short, long, env)]
    token: Option<String>,
    #[clap(short, long, env, default_value = "919b04037636d3b4bbc0af135eaccdfa")]
    zone: String,
}

impl Args {
    pub fn name(&self) -> Result<String> {
        if let Some(name) = &self.name {
            return Ok(name.to_string());
        }
        Ok(whoami::hostname() + ".ddns.liblaf.me")
    }

    pub fn token(&self) -> Result<String> {
        if let Some(token) = &self.token {
            return Ok(token.to_string());
        }
        if let Ok(token) = claps::external::bitwarden::get_notes("CLOUDFLARE_TOKEN") {
            return Ok(token);
        }
        claps::bail!("token was not provided");
    }

    pub fn zone(&self) -> Result<String> {
        Ok(self.zone.to_string())
    }
}

use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use metrics::counter;

use crate::utils::fetch_page;
use crate::{errors::GitHubError, GitHubLinkDependencies};

lazy_static! {
    static ref LINK_PATTERN: Regex =
        Regex::new(r#"^https?://github.com/([a-zA-Z0-9_\.-]{1,35})/([a-zA-Z0-9_\.-]{1,101})/?$"#)
            .unwrap();
    static ref SPAN_SELECTOR: Selector = Selector::parse("span").unwrap();
}

/// A link to a GitHub repository.
///
/// Contains information about the repository owner and name.
///
/// Its dependencies can be fetched using the [`dependencies`] method to create
/// a [`GitHubLinkDependencies`] object.
///
/// [`dependencies`]: GitHubLink::dependencies
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct GitHubLink {
    link: String,
    owner: String,
    repo: String,
}

impl std::fmt::Display for GitHubLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

impl TryFrom<String> for GitHubLink {
    type Error = GitHubError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim();
        if let Some(captures) = LINK_PATTERN.captures(value) {
            return Ok(Self {
                link: value.to_string(),
                owner: captures[1].to_string(),
                repo: captures[2].to_string(),
            });
        }
        Err(GitHubError::InvalidLink(value.to_string()))
    }
}

impl GitHubLink {
    /// The GitHub Link in the form https://github.com/OWNER/REPO
    pub fn link(&self) -> &str {
        &self.link
    }

    /// The GitHub repository owner, taken from the repo link
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// The GitHub repository name, taken from the repo link
    pub fn repo(&self) -> &str {
        &self.repo
    }

    /// The path of the GitHub repository, in the form OWNER/REPO
    pub fn path(&self) -> String {
        format!("{}/{}", &self.owner, &self.repo)
    }

    /// The number of contributors displayed on the right side
    /// of the main page.
    pub async fn fetch_contributors(&self) -> Result<usize, GitHubError> {
        counter!("api_fetch", "type" => "contributor").increment(1);
        let html = self.fetch_main_page().await?;
        self.get_contributors_from_html(&html)
    }

    /// The dependencies of the repo, found in the
    /// "Insight -> Dependency Graph" page.
    pub fn dependencies(&self) -> GitHubLinkDependencies {
        GitHubLinkDependencies::new(self.clone())
    }

    async fn fetch_main_page(&self) -> Result<Html, GitHubError> {
        fetch_page(self.link()).await
    }

    fn get_contributors_from_html(&self, html: &Html) -> Result<usize, GitHubError> {
        let a_selector =
            Selector::parse(format!(r#"a[href="/{}/graphs/contributors"]"#, self.path()).as_str())
                .unwrap();

        let Some(component) = html.select(&a_selector).next() else {
            return Err(GitHubError::NoContributorsComponent(self.link.clone()));
        };
        let Some(span) = component.select(&SPAN_SELECTOR).next() else {
            return Err(GitHubError::NoContributorsComponent(self.link.clone()));
        };
        let contributors = span.text().collect::<String>().parse::<usize>();
        if contributors.is_err() {
            return Err(GitHubError::NoContributorsComponent(self.link.clone()));
        }

        Ok(contributors.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_link() {
        let link = GitHubLink::try_from("https://github.com/OWNER/REPO".to_string()).unwrap();
        assert_eq!(link.link(), "https://github.com/OWNER/REPO");
        assert_eq!(link.owner(), "OWNER");
        assert_eq!(link.repo(), "REPO");
        assert_eq!(link.path(), "OWNER/REPO");

        assert!(GitHubLink::try_from("https://github.com/OWNER/REPO/".to_string()).is_ok());
        assert!(
            GitHubLink::try_from("       https://github.com/OWNER/REPO/    \n  ".to_string())
                .is_ok()
        );
        assert!(GitHubLink::try_from("http://github.com/OWNER/REPO".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_link() {
        assert!(GitHubLink::try_from("git://github.com/OWNER".to_string()).is_err());
        assert!(GitHubLink::try_from("https://github.com/OWNER".to_string()).is_err());
        assert!(GitHubLink::try_from("https://github.com//".to_string()).is_err());
        assert!(
            GitHubLink::try_from("https://github.com/OWNER/REPO/TOO_MUCH_DEPTH".to_string())
                .is_err()
        );
        assert!(GitHubLink::try_from("https://github.com/$&*sad/??\"asd".to_string()).is_err());
        assert!(GitHubLink::try_from("/OWNER/REPO".to_string()).is_err());
        assert!(GitHubLink::try_from("OWNER/REPO".to_string()).is_err());
        assert!(GitHubLink::try_from("".to_string()).is_err());
    }

    #[test]
    fn test_format() {
        let link = GitHubLink::try_from("https://github.com/OWNER/REPO".to_string()).unwrap();
        assert_eq!(format!("{}", link), "OWNER/REPO");
    }
}

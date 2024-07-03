use lazy_static::lazy_static;
use scraper::{Html, Selector};

use crate::utils::fetch_page;
use crate::{GitHubError, GitHubLink};

lazy_static! {
    static ref DEPENDENCY_SELECTOR: Selector =
        Selector::parse(r#"a[data-hovercard-type="dependendency_graph_package"]"#).unwrap();
    static ref PAGINATION_SELECTOR: Selector = Selector::parse(r#"em[class="current"]"#).unwrap();
}

/// An iterator over a [`GitHubLink`]'s dependencies.
pub struct GitHubLinkDependencies {
    link: GitHubLink,
    index: usize,
    page: usize,
    current_html: Option<Html>,
    number_of_pages: Option<usize>,
    number_of_items: Option<usize>,
}

// Don't waste your time with Arc and RwLock, it does not fix the Websocket requirements
unsafe impl Send for GitHubLinkDependencies {}
unsafe impl Sync for GitHubLinkDependencies {}

impl GitHubLinkDependencies {
    pub fn new(link: GitHubLink) -> Self {
        Self {
            link,
            index: 0,
            page: 0,
            number_of_pages: None,
            current_html: None,
            number_of_items: None,
        }
    }

    /// Iterates over the next dependency. Can be used as follows:
    ///
    ///```rust
    ///let link = GitHubLink::try_from("https://github.com/tokio-rs/tokio".to_string()).unwrap();
    ///let mut dep_iterator = link.dependencies();
    ///while let Some(dep) = dep_iterator.next().await {
    ///    if let Ok(l) = dep {
    ///        // Do something with l
    ///    } else {
    ///        eprintln!("Dependency fetching error: {}", dep.unwrap_err());
    ///    }
    ///}
    ///```
    pub async fn next(&mut self) -> Option<Result<GitHubLink, GitHubError>> {
        if self.current_html.is_none() {
            if self.number_of_pages.is_some() && self.page >= self.number_of_pages.unwrap() {
                return None;
            }
            self.page += 1;
            let fetched_html = self.fetch_page(self.page).await;
            if let Ok(current_page) = fetched_html {
                self.current_html = Some(current_page);
            } else {
                return Some(Err(fetched_html.unwrap_err()));
            }
        }

        let current_page = self.current_html.as_ref().unwrap();

        if self.number_of_pages.is_none() {
            if let Some(pagination_component) = current_page.select(&PAGINATION_SELECTOR).next() {
                self.number_of_pages = Some(
                    pagination_component
                        .attr("data-total-pages")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                );
            } else {
                self.number_of_pages = Some(1);
            }
        }

        let mut elements = current_page.select(&DEPENDENCY_SELECTOR);

        if self.number_of_items.is_none() {
            self.number_of_items = Some(elements.clone().count());
        }

        let Some(current_element) = elements.nth(self.index) else {
            return None;
        };
        let repo_path = current_element.attr("href").unwrap();
        let repo_path = format!("https://github.com{}", repo_path.trim());
        let output = GitHubLink::try_from(repo_path);

        self.index += 1;

        if self.number_of_items.unwrap() <= self.index {
            self.number_of_items = None;
            self.current_html = None;
            self.index = 0;
        }

        Some(output)
    }

    async fn fetch_page(&self, page: usize) -> Result<Html, GitHubError> {
        let link = format!("{}/network/dependencies?page={}", self.link.link(), page);
        fetch_page(&link).await
    }
}

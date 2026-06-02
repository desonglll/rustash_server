use serde::{Deserialize, Serialize};

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_PER_PAGE: u64 = 20;
const MAX_PER_PAGE: u64 = 100;

#[derive(Deserialize, Clone)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

impl PaginationQuery {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(DEFAULT_PAGE).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(DEFAULT_PER_PAGE).clamp(1, MAX_PER_PAGE)
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.per_page()
    }
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: u64, query: &PaginationQuery) -> Self {
        Self {
            data,
            total,
            page: query.page(),
            per_page: query.per_page(),
        }
    }
}

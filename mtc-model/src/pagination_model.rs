use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct PaginationModel {
    pub total: usize,
    pub per_page: usize,
    pub current_page: usize,
    pub from: usize,
    pub to: usize,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub next_page_number: usize,
    pub previous_page_number: usize,
}

#[derive(Deserialize)]
pub struct CountModel {
    pub count: usize,
}

impl PaginationModel {
    pub fn new(
        total: usize,
        per_page: usize) -> Self {
        Self {
            total,
            per_page,
            current_page: 1,
            from: 0,
            to: 0,
            has_next_page: false,
            has_previous_page: false,
            next_page_number: 1,
            previous_page_number: 1,
        }
    }
}

pub trait PaginationBuilder {
    fn page(&mut self, page: usize) -> Self;
}

impl PaginationBuilder for PaginationModel {
    fn page(&mut self, page: usize) -> Self {
        self.current_page = match page {
            n if n > 1 => n,
            _ => 1
        };

        self.from = (self.current_page - 1) * self.per_page + 1;
        self.to = self.from + self.per_page - 1;

        if self.total > self.to {
            self.has_next_page = true;
        };

        if self.current_page > 1 {
            self.has_previous_page = true;
        };

        self.previous_page_number = self.current_page - 1;
        self.next_page_number = self.current_page + 1;

        if self.previous_page_number < 1 {
            self.previous_page_number = 1;
        }

        if !self.has_next_page {
            self.next_page_number = self.current_page
        }

        Self {
            total: self.total,
            per_page: self.per_page,
            current_page: self.current_page,
            from: self.from,
            to: self.to,
            has_next_page: self.has_next_page,
            has_previous_page: self.has_previous_page,
            next_page_number: self.next_page_number,
            previous_page_number: self.previous_page_number,
        }
    }
}
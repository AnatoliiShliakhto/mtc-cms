use super::*;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_sql(&self) -> String {
        match self {
            SortDirection::Asc => "ASC".to_string(),
            SortDirection::Desc => "DESC".to_string(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OrderBy {
    pub column: String,
    pub direction: SortDirection,
}

impl OrderBy {
    pub fn new(column: String, direction: SortDirection) -> Self {
        OrderBy { column, direction }
    }

    pub fn as_sql(&self) -> String {
        format!("{} {}", self.column, self.direction.as_sql())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PageRequest {
    pub page_size: usize,
    pub page_index: usize,
    pub order_by: Vec<OrderBy>,
}

impl PageRequest {
    pub fn new(page_size: usize, page_index: usize, order_by: Vec<OrderBy>) -> Self {
        Self {
            page_size,
            page_index,
            order_by,
        }
    }

    pub fn all() -> Self {
        Self {
            page_size: 1000000000,
            page_index: 0,
            order_by: vec![],
        }
    }

    pub fn from_page_size(page_size: usize) -> Self {
        Self {
            page_size,
            ..Default::default()
        }
    }

    pub fn from_index_size(page_index: usize) -> Self {
        Self {
            page_index,
            ..Default::default()
        }
    }

    pub fn offset(&self) -> usize {
        self.page_size * (self.page_index)
    }

    pub fn limit(&self) -> usize {
        self.page_size
    }

    pub fn as_sql(&self) -> String {
        let order_by = self.order_by.iter().map(|order_by| order_by.as_sql()).fold(
            String::new(),
            |mut accumulator, order_by_sql| {
                if accumulator.is_empty() {
                    accumulator.push_str("ORDER BY ");
                } else {
                    accumulator.push_str(", ");
                }
                accumulator.push_str(&order_by_sql);
                accumulator
            },
        );
        format!("{order_by} START {} LIMIT {}", self.offset(), self.limit())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PageResponse<T> {
    pub number_of_pages: usize,
    pub page_size: usize,
    pub page_index: usize,
    pub page_rows: Vec<T>,
}

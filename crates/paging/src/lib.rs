#![doc = include_str!("../README.md")]

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Default number of items.
const DEFAULT_LIMIT: i64 = 20;

/// This is a helper struct that does not represent anything in the database.
///
/// It is used in queries such as:
/// ```sql
/// SELECT * FROM table OFFSET {self.offset} LIMIT {self.limit};
/// ```
pub struct PageEntity {
    pub offset: i64,
    pub limit: i64, 
}

impl From<PageDto> for PageEntity {
    fn from(value: PageDto) -> Self {
        Self {
            offset: value.offset(),
            limit: value.limit(),
        }
    }
}

/// This is a more complex version of [`PageEntity`]. It contains [`Option`] fields,
/// allowing creation of the [`PageEntity`] even if the user does not provide page data.
#[derive(Serialize, Deserialize, Validate)]
pub struct PageDto {
    /// should be greater than `1`
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    /// should be in range of `1..=60`
    #[validate(range(min = 1, max = 60))]
    pub count: Option<i64>,
}

impl PageDto {
    /// Calculates the `OFFSET` value using the formula `(page - 1) * count`.
    fn offset(&self) -> i64 {
        let page = self.page.unwrap_or_default();
        let limit = self.limit();

        (page - 1) * limit
    }

    /// Returns the `LIMIT` value from `self.count`, or `DEFAULT_LIMIT` (20) if `self.count` is `None`.
    fn limit(&self) -> i64 {
        self.count.unwrap_or(DEFAULT_LIMIT)
    }
}

impl From<PageScheme> for PageDto {
    fn from(value: PageScheme) -> Self {
        Self {
            page: value.page,
            count: value.count,
        }
    }
}

/// This struct represents request and response.
#[derive(Serialize, Deserialize)]
pub struct PageScheme {
    pub page: Option<i64>,
    pub count: Option<i64>,
}

/// This struct used for responses where we return a list of items.
#[derive(Serialize)]
#[serde(bound = "T: Serialize")]
#[allow(unused)] // bad thing, ik
pub struct ListWithPagingResponse<T: Serialize> where {
    #[serde(flatten)]
    paging: PageScheme,
    items: Vec<T>,
}

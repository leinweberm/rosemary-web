use serde_derive::{Deserialize, Serialize};

use crate::client::translations::Language;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPaintingsQueryParsed {
    pub limit: u8,
    pub offset: u32,
    pub sort: String,
    pub order: String,
    pub lang: String,
    pub search: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPaintingsQuery {
    pub limit: Option<u8>,
    pub offset: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub search: Option<String>,
    pub lang: Option<String>,
}

impl GetPaintingsQuery {
    pub fn default(language: Option<Language>) -> GetPaintingsQueryParsed {
        let lang = if let Some(language_enum) = language {
            language_enum.to_string()
        } else {
            Language::Cs.to_string()
        };

        GetPaintingsQueryParsed {
            limit: 20,
            offset: 0,
            sort: String::from("created"),
            order: String::from("desc"),
            search: None,
            lang,
        }
    }

    pub fn safe_parse(&self, language: Option<Language>) -> GetPaintingsQueryParsed {
        let lang = if let Some(language_enum) = language {
            language_enum.to_string()
        } else {
            Language::Cs.to_string()
        };

        let limit = if let Some(limit_value) = &self.limit {
            if *limit_value < 0_u8 || *limit_value > 100_u8 {
                0_u8
            } else {
                *limit_value
            }
        } else {
            0_u8
        };

        let offset = if let Some(offset_value) = &self.offset {
            if *offset_value < 0_u32 {
                0_u32
            } else {
                *offset_value
            }
        } else {
            0_u32
        };

        let sort = if let Some(sort_value) = &self.sort {
            match sort_value.as_str() {
                "created" => sort_value.clone(),
                "price" => sort_value.clone(),
                "painting_title" => String::from(format!("painting_title->>'{}'", lang)),
                "painting_description" => String::from(format!("painting_description->>'{}'", lang)),
                "width" => sort_value.clone(),
                "height" => sort_value.clone(),
                "sold" => sort_value.clone(),
                _ => String::from("created"),
            }
        } else {
            String::from("created")
        };

        let order = if let Some(order_value) = &self.order {
            match order_value.as_str() {
                "ASC" => String::from("asc"),
                "asc" => String::from("asc"),
                "DESC" => String::from("desc"),
                "desc" => String::from("desc"),
                _ => String::from("desc"),
            }
        } else {
            String::from("desc")
        };

        let search = if let Some(search_value) = &self.search {
            Some(search_value.clone())
        } else {
            None
        };

        GetPaintingsQueryParsed {
            limit,
            offset,
            sort,
            order,
            search,
            lang: lang.clone(),
        }
    }
}
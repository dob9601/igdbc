use std::fmt::{self, Display};

use itertools::Itertools;

pub struct ApicalypseQuery {
    search: SearchOptions,
    fields: FieldOptions,
    exclude: ExcludeOptions,
    limit: LimitOptions,
    offset: OffsetOptions,
    where_clause: WhereOptions,
}

impl Display for ApicalypseQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.search {
            SearchOptions::Search(search) => writeln!(f, "search \"{}\";", search)?,
            SearchOptions::Unset => (),
        };

        match &self.fields {
            FieldOptions::All => writeln!(f, "fields *;")?,
            FieldOptions::Fields(fields) => writeln!(f, "fields {};", fields.join(", "))?,
            FieldOptions::Unset => (),
        };

        match &self.exclude {
            ExcludeOptions::All => writeln!(f, "exclude *;")?,
            ExcludeOptions::Fields(fields) => writeln!(f, "exclude {};", fields.join(", "))?,
            ExcludeOptions::Unset => (),
        };

        match &self.limit {
            LimitOptions::Limit(limit) => writeln!(f, "limit {};", limit)?,
            LimitOptions::Unset => (),
        };

        match &self.offset {
            OffsetOptions::Offset(offset) => writeln!(f, "offset {};", offset)?,
            OffsetOptions::Unset => (),
        };

        match &self.where_clause {
            WhereOptions::Where(where_clause) => writeln!(f, "where {};", where_clause)?,
            WhereOptions::Unset => (),
        };

        Ok(())
    }
}

impl ApicalypseQuery {
    pub fn builder() -> Self {
        Self {
            fields: FieldOptions::Unset,
            exclude: ExcludeOptions::Unset,
            limit: LimitOptions::Unset,
            where_clause: WhereOptions::Unset,
            offset: OffsetOptions::Unset,
            search: SearchOptions::Unset,
        }
    }

    pub fn fields(mut self, fields: Vec<impl ToString>) -> Self {
        self.fields = FieldOptions::Fields(
            fields
                .into_iter()
                .map(|field| field.to_string())
                .collect_vec(),
        );
        self
    }

    pub fn fields_all(mut self) -> Self {
        self.fields = FieldOptions::All;
        self
    }

    pub fn exclude(mut self, excluded_fields: Vec<impl ToString>) -> Self {
        self.exclude = ExcludeOptions::Fields(
            excluded_fields
                .into_iter()
                .map(|field| field.to_string())
                .collect_vec(),
        );
        self
    }

    pub fn exclude_all(mut self) -> Self {
        self.exclude = ExcludeOptions::All;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = LimitOptions::Limit(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = OffsetOptions::Offset(offset);
        self
    }

    pub fn r#where(mut self, where_clause: impl ToString) -> Self {
        self.where_clause = WhereOptions::Where(where_clause.to_string());
        self
    }

    pub fn search(mut self, search: impl ToString) -> Self {
        self.search = SearchOptions::Search(search.to_string());
        self
    }

    pub fn and_where(mut self, and_where: impl AsRef<str>) -> Self {
        let mut where_clause = if let WhereOptions::Where(where_clause) = self.where_clause {
            where_clause
        } else {
            String::new()
        };

        where_clause.push_str(" & ");
        where_clause.push_str(and_where.as_ref());

        self.where_clause = WhereOptions::Where(where_clause);

        self
    }
}

enum ExcludeOptions {
    Unset,
    All,
    Fields(Vec<String>),
}

enum FieldOptions {
    Unset,
    All,
    Fields(Vec<String>),
}

enum LimitOptions {
    Unset,
    Limit(usize),
}

enum OffsetOptions {
    Unset,
    Offset(usize),
}

enum WhereOptions {
    Unset,
    Where(String),
}

enum SearchOptions {
    Unset,
    Search(String),
}

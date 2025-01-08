use super::*;

pub trait FormDataService {
    fn get_str(&self, field: &str) -> Option<Cow<'static, str>>;
    fn get_bool(&self, field: &str) -> bool;
    fn get_str_array(&self, field: &str) -> Option<Vec<Cow<'static, str>>>;
    fn get_i64(&self, field: &str) -> Option<i64>;
    fn get_usize(&self, field: &str) -> Option<usize>;
    fn get_fields_array(&self) -> Option<Vec<Field>>;
    fn get_links_array(&self, field: &str) -> Vec<LinkEntry>;
}

impl FormDataService for Event<FormData> {
    /// Returns the first string value associated with the specified field as a
    /// [`Cow<str>`].
    ///
    /// If the field exists and contains at least one value, this function returns
    /// [`Some(Cow<str>)`] with the first value. If the field does not exist
    /// or contains no values, it returns [`None`].
    ///
    /// # Arguments
    ///
    /// * `field` - A string slice that holds the name of the field to retrieve.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::prelude::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_str("field_name");
    ///
    /// assert!(result.is_some());
    /// ```
    fn get_str(&self, field: &str) -> Option<Cow<'static, str>> {
        if let Some(value) = self.values().get(field) {
            if !value.0.is_empty() {
                return Some(value.0[0].to_owned().into())
            }
        }
        None
    }

    /// Checks if the specified field exists in the form data.
    ///
    /// This function returns `true` if the field exists in the form data,
    /// otherwise it returns `false`.
    ///
    /// # Arguments
    ///
    /// * `field` - A string slice that holds the name of the field to check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::prelude::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_bool("field_name");
    ///
    /// assert!(result);
    /// ```
    fn get_bool(&self, field: &str) -> bool {
        self.values().contains_key(field)
    }

    /// Returns all values associated with the given field as an array of string slices.
    ///
    /// This function returns `None` if the field does not exist in the form data.
    /// Otherwise, it returns `Some(Vec<Cow<'static, str>>)`, where the vector contains
    /// all values associated with the field.
    ///
    /// # Arguments
    ///
    /// * `field` - A string slice that holds the name of the field to retrieve.
    ///
    /// # Examples
    /// ```rust
    /// use crate::prelude::::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_str_array("field_name");
    ///
    /// assert!(result);
    /// ```
    fn get_str_array(&self, field: &str) -> Option<Vec<Cow<'static, str>>> {
        if let Some(FormValue(value)) = self.values().get(field) {
            return Some(value
                .iter()
                .map(|val| val.to_owned().into())
                .collect::<Vec<Cow<'static, str>>>());
        }
        None
    }

    /// Attempts to parse the first value associated with the given field as an `i64`.
    ///
    /// If the field exists and the first value can be parsed into an `i64`, this
    /// function returns `Some(i64)`. Otherwise, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `field` - A string slice that holds the name of the field to retrieve.
    ///
    /// # Examples
    /// ```rust
    /// use crate::prelude::::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_i64("field_name");
    ///
    /// assert!(result);
    /// ```
    fn get_i64(&self, field: &str) -> Option<i64> {
        if let Some(FormValue(value)) = self.values().get(field) {
            if let Ok(value) = value[0].parse::<i64>() {
                return Some(value);
            }
        }
        None
    }

    /// Attempts to parse the first value associated with the given field as a `usize`.
    ///
    /// If the field exists and the first value can be parsed into a `usize`, this
    /// function returns `Some(usize)`. Otherwise, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `field` - A string slice that holds the name of the field to retrieve.
    ///
    /// # Examples
    /// ```rust
    /// use crate::prelude::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_usize("field_name");
    ///
    /// assert!(result.is_some());
    /// ```
    fn get_usize(&self, field: &str) -> Option<usize> {
        if let Some(FormValue(value)) = self.values().get(field) {
            if let Ok(value) = value[0].parse::<usize>() {
                return Some(value);
            }
        }
        None
    }

    /// Attempts to parse the form data associated with the "fields-kind",
    /// "fields-slug", and "fields-title" fields and returns a vector of
    /// [`Field`] objects.
    ///
    /// Each element in the vector is created by associating the `i`th element
    /// of the "fields-kind" array with the `i`th element of the "fields-slug"
    /// array and the `i`th element of the "fields-title" array.
    ///
    /// If any of the three fields are not present or their lengths do not match,
    /// this function returns `None`.
    ///
    /// # Examples
    /// ```rust
    /// use crate::prelude::::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_fields_array("field_name");
    ///
    /// assert!(result);
    /// ```
    fn get_fields_array(&self) -> Option<Vec<Field>> {
        let mut fields: Vec<Field> = vec![];

        let kind_set = self.get_str_array("fields-kind").unwrap_or_default();
        let slug_set = self.get_str_array("fields-slug").unwrap_or_default();
        let title_set = self.get_str_array("fields-title").unwrap_or_default();

        for (kind, (slug, title))
        in zip(kind_set, zip(slug_set, title_set)) {
            fields.push(Field {
                kind: FieldKind::from_str(&*kind).unwrap_or_default(),
                slug,
                title,
            })
        }

        if fields.is_empty() { return None }
        Some(fields)
    }

    /// Attempts to parse the form data associated with the `field` string as a
    /// CSV string of semicolon-delimited values and returns a vector of
    /// [`LinkEntry`] objects.
    ///
    /// Each element in the vector is created by associating the `i`th element
    /// of the first column with the `i`th element of the second column.
    ///
    /// If the `field` is not present or empty, this function returns `None`.
    ///
    /// # Examples
    /// ```rust
    /// use crate::prelude::::FormDataService;
    ///
    /// let form_data = // ... obtain form data
    /// let result = form_data.get_links_array("field_name");
    ///
    /// assert!(result);
    /// ```
    fn get_links_array(&self, field: &str) -> Vec<LinkEntry> {
        let links_str = self.get_str(field).unwrap_or_default();
        if links_str.is_empty() { return vec![] }

        let mut links = Vec::<LinkEntry>::new();

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(false)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(links_str.as_bytes());

        for item in reader.deserialize::<LinkEntry>() {
            if let Ok(item) = item {
                links.push(item);
            }
        }

        links
    }
}
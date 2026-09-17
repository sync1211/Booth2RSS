use substring::Substring;

pub fn index_of(input: &String, search: &str, start_index: usize) -> Option<usize> {
    let mut input_slice = input.to_owned();
    
    if start_index > 0 {
        input_slice = input.substring(start_index, input.len()).to_string();
    }

    let search_vec = search.chars().collect::<Vec<_>>();
    let search_vec_len = search_vec.len();

    let input_vec = input_slice.chars().collect::<Vec<_>>();

    let mut search_index = 0;
    for (i, input_char) in input_vec.iter().enumerate() {
        let search_char = search_vec[search_index];

        if search_char == *input_char {
            if (search_index + 1) == search_vec_len {
                return Some((i - (search_vec_len - 1)) + start_index);
            }

            search_index += 1;
        } else {
            search_index = 0;
        }
    }
    
    return None;
}

pub fn get_value_between_snippets(content: &String, start: &str, end: &str) -> Option<String> {
    let (result, _) = get_value_between_snippets_offset(content, start, end, 0);
    return result;
}

pub fn get_value_between_snippets_offset(content: &String, start: &str, end: &str, in_offset: usize) -> (Option<String>,usize) {

    let start_index = match index_of(content, start, in_offset) {
        Some(index) => index,
        None => return (None, in_offset)
    };

    let value_start_index = start_index + start.len();

    let end_index = match index_of(content, end, value_start_index) {
        Some(index) => index,
        None => return (None, in_offset)
    };

    let slice= content.substring(value_start_index, end_index);
    return (Some(slice.to_string()), end_index);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        let input = "This is a short test string".to_string();
        let expected: usize = 16;

        let result = index_of(&input, "test", 0).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_index_of_slice() {
        let input = "This is a test function to test the functionality of the index_of function".to_string();
        let expected: usize = 66;

        let result = index_of(&input, "function", 49).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_value_between_snippets() {
        let input = "This is a test function to test the functionality of the get_value_between_snippets function".to_string();
        let expected = "get_value_between_snippets".to_string();

        let result = get_value_between_snippets(&input, "of the ", " function").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_value_between_snippets_unicode() {
        let input = "This is a testäüö 🎖️function to test the functionality of the get_value_between_snippets function🥍".to_string();
        let expected = "get_value_between_snippets".to_string();

        let result = get_value_between_snippets(&input, "of the ", " function").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_value_between_snippets_offset() {
        let input = "This is a short test of the get_value_between_snippets_offset function and its parameters, specifically a test of the offset parameter".to_string();
        let expected_result = "offset".to_string();
        let expected_offset = 124;

        let (result, offset) = get_value_between_snippets_offset(&input, "of the ", " parameter", 91);

        assert_eq!(result.unwrap(), expected_result);
        assert_eq!(offset, expected_offset);
    }
}

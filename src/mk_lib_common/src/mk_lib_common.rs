pub async fn print_type_of_variable<T>(_: &T) {
    #[cfg(debug_assertions)]
    {
        // mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(
        //     json!({ "data_type": std::any::type_name::<T>() }),
        // )
        // .await
        // .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_print_type_of_variable_i32() {
        let x: i32 = 42;
        print_type_of_variable(&x).await;
    }

    #[tokio::test]
    async fn test_print_type_of_variable_string() {
        let s = String::from("hello");
        print_type_of_variable(&s).await;
    }

    #[tokio::test]
    async fn test_print_type_of_variable_vec() {
        let v = vec![1, 2, 3];
        print_type_of_variable(&v).await;
    }
}

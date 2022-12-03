use lambda_http::{
    Response,
    Request,
    Body,
    aws_lambda_events::serde_json::json, 
    IntoResponse,
};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct PizzaList {
    pizzas: Vec<Pizza>
}


impl PizzaList {
    fn new() -> Self {
        PizzaList {pizzas: vec![
            Pizza {name: String::from("veggie"), price: 10},
            Pizza {name: String::from("regina"), price: 12},
            Pizza {name: String::from("deluxe"), price: 40}
        ]}
    } 
}


#[derive(Serialize, Deserialize)]
struct Pizza {
    name: String,
    price: i32
}



fn get_pizza_from_name<'a>(pizza_name: &str, pizza_list: &'a PizzaList) -> Option<&'a Pizza>{
    let mut iter = pizza_list.pizzas.iter();
    iter.find(|pizza| pizza.name == pizza_name)
}


async fn build_success_response(pizza: &Pizza) -> Response<Body>{
    json!(pizza).into_response().await
}


async fn build_failure_response(error_message: &str) -> Response<Body>{
    Response::builder().status(400).header("content-type", "application/json")
    .body(Body::from(json!({"error": error_message}).to_string()))
    .expect("could not build the error response")
}


fn process_event<'a>(pizza_name: Option<&'a str>, pizza_list: &'a PizzaList) -> Result<&'a Pizza, &'a str> {
    match pizza_name {
        Some(pizza_name) => match get_pizza_from_name(pizza_name, pizza_list){
            Some(pizza) => Ok(pizza),
            _ => panic!("")
        },
        _ => panic!("") 
    }
}

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn new_pizza_list_test() {
        let all_pizza = PizzaList::new();
        assert_eq!(1, all_pizza.pizzas.len());
        let veggie = get_pizza_from_name("veggie", &all_pizza);
        assert_eq!(10, veggie.unwrap().price);
        let regina = get_pizza_from_name("regina", &all_pizza);
        assert_eq!(12, regina.unwrap().price);
        let deluxe = get_pizza_from_name("deluxe", &all_pizza);
        assert_eq!(12, deluxe.unwrap().price);
    }

    #[tokio::test]
    async fn build_success_response_test(){
        let test_pizza = Pizza {name: String::from("test_pizza"), price: 100};
        let result = build_success_response(&test_pizza).await;
        let (parts, body) = result.into_parts();
        assert_eq!(200, parts.status);
        assert_eq!("application/json", parts.headers.get("content-type").unwrap());
        assert_eq!("{\"name\":\"test_pizza\", \"price\":100}".to_string(), String::from_utf8(body.to_ascii_lowercase()))
    }

    #[tokio::test]
    async fn build_failure_response_test(){
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(400, parts.status);
        assert_eq!("application/json", parts.headers.get("content-type").unwrap());
        assert_eq!("{\"name\":\test error message\"}".to_string(), String::from_utf8(body.to_ascii_lowercase()).unwrap())
    }

    #[test]
    fn process_event_valid_pizza_test(){
        let pizza_list = PizzaList::new();
        let res = process_event(Some("regina"), &pizza_list);
        assert!(res.is_ok());
    }
}
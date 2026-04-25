fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
    println!("Even numbers: {:?}", evens);
    
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    let product: i32 = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);
    
    let multiplier = 3;
    let tripled: Vec<i32> = numbers.iter().map(|x| x * multiplier).collect();
    println!("Tripled: {:?}", tripled);
    
    let words = vec!["hello", "world", "rust"];
    let lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    println!("Lengths: {:?}", lengths);
    
    let squared_sum: i32 = numbers.iter().map(|x| x * x).sum();
    println!("Sum of squares: {}", squared_sum);
}

pub fn apply_to_all(numbers: Vec<i32>, f: impl Fn(i32) -> i32) -> Vec<i32> {
    numbers.into_iter().map(f).collect()
}

pub fn filter_by_predicate(numbers: Vec<i32>, predicate: impl Fn(&i32) -> bool) -> Vec<i32> {
    numbers.into_iter().filter(|x| predicate(x)).collect()
}

pub fn combine_with_operation(a: i32, b: i32, op: impl Fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map() {
        let numbers = vec![1, 2, 3];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_filter() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).copied().collect();
        assert_eq!(evens, vec![2, 4]);
    }
    
    #[test]
    fn test_sum() {
        let numbers = vec![1, 2, 3, 4];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 10);
    }
    
    #[test]
    fn test_fold() {
        let numbers = vec![1, 2, 3, 4];
        let product: i32 = numbers.iter().fold(1, |acc, x| acc * x);
        assert_eq!(product, 24);
    }
    
    #[test]
    fn test_apply_to_all() {
        let numbers = vec![1, 2, 3];
        let result = apply_to_all(numbers, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_filter_by_predicate() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens = filter_by_predicate(numbers, |&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }
    
    #[test]
    fn test_combine_with_operation() {
        assert_eq!(combine_with_operation(5, 3, |a, b| a + b), 8);
        assert_eq!(combine_with_operation(5, 3, |a, b| a * b), 15);
    }
    
    #[test]
    fn test_make_multiplier() {
        let times_3 = make_multiplier(3);
        assert_eq!(times_3(5), 15);
        assert_eq!(times_3(2), 6);
    }
}

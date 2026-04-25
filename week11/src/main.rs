mod student;
use student::{Student, Grade};

fn main() {
    let mut student = Student::new(
        String::from("S001"),
        String::from("Alice Johnson"),
        String::from("alice@example.com"),
    );

    println!("Name: {}", student.name);
    println!("Standing: {}", student.class_standing());

    student.add_credits(30);
    println!("After 30 credits: {}", student.class_standing());

    student.add_credits(90);
    println!("Can graduate? {}", student.can_graduate());
    
    let grade_a = Grade::A;
    println!("Grade A GPA: {}", grade_a.to_gpa_points());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new(
            String::from("S001"),
            String::from("Bob"),
            String::from("bob@example.com"),
        );
        assert_eq!(student.id, "S001");
        assert_eq!(student.credits_earned, 0);
    }

    #[test]
    fn test_class_standing() {
        let mut student = Student::new(
            String::from("S001"),
            String::from("Alice"),
            String::from("alice@example.com"),
        );
        assert_eq!(student.class_standing(), "Freshman");
        student.add_credits(30);
        assert_eq!(student.class_standing(), "Sophomore");
    }

    #[test]
    fn test_can_graduate() {
        let mut student = Student::new(
            String::from("S001"),
            String::from("Alice"),
            String::from("alice@example.com"),
        );
        assert!(!student.can_graduate());
        student.add_credits(120);
        assert!(student.can_graduate());
    }

    #[test]
    fn test_grade_gpa() {
        assert_eq!(Grade::A.to_gpa_points(), 4.0);
        assert_eq!(Grade::B.to_gpa_points(), 3.0);
        assert_eq!(Grade::F.to_gpa_points(), 0.0);
    }

    #[test]
    fn test_grade_from_string() {
        assert_eq!(Grade::from_string("A"), Some(Grade::A));
        assert_eq!(Grade::from_string("F"), Some(Grade::F));
        assert_eq!(Grade::from_string("Z"), None);
    }

    #[test]
    fn test_grade_is_passing() {
        assert!(Grade::A.is_passing());
        assert!(Grade::C.is_passing());
        assert!(!Grade::D.is_passing());
        assert!(!Grade::F.is_passing());
    }
}

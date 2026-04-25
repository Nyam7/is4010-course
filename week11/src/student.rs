pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub credits_earned: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl Student {
    pub fn new(id: String, name: String, email: String) -> Student {
        Student {
            id,
            name,
            email,
            credits_earned: 0,
        }
    }

    pub fn class_standing(&self) -> &str {
        match self.credits_earned {
            0..=29 => "Freshman",
            30..=59 => "Sophomore",
            60..=89 => "Junior",
            _ => "Senior",
        }
    }

    pub fn add_credits(&mut self, credits: u16) {
        self.credits_earned += credits;
    }

    pub fn can_graduate(&self) -> bool {
        self.credits_earned >= 120
    }
}

impl Grade {
    pub fn to_gpa_points(&self) -> f32 {
        match self {
            Grade::A => 4.0,
            Grade::B => 3.0,
            Grade::C => 2.0,
            Grade::D => 1.0,
            Grade::F => 0.0,
        }
    }

    pub fn from_string(s: &str) -> Option<Grade> {
        match s.to_uppercase().as_str() {
            "A" => Some(Grade::A),
            "B" => Some(Grade::B),
            "C" => Some(Grade::C),
            "D" => Some(Grade::D),
            "F" => Some(Grade::F),
            _ => None,
        }
    }

    pub fn is_passing(&self) -> bool {
        match self {
            Grade::A | Grade::B | Grade::C => true,
            Grade::D | Grade::F => false,
        }
    }
}

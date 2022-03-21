use serde::{Serialize, Deserialize};


#[derive(Deserialize, Clone, Debug)]
#[serde(from = "&str")]
pub enum Bar {
    None,
    Low,
    Middle,
    High,
    Traversal
}

impl From<&str> for Bar {
    fn from(rhs: &str) -> Self {
        match rhs {
            "Low" => Self::Low,
            "Middle" => Self::Middle,
            "High" => Self::High,
            "Traversal" => Self::Traversal,
            _ => Self::None
        }
    }
}

impl Into<i32> for Bar {
    fn into(self) -> i32 {
        match self {
            Bar::None => 0,
            Bar::Low => 1,
            Bar::Middle => 2,
            Bar::High => 3,
            Bar::Traversal => 4,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(from = "&str")]
pub enum Drivetrain {
    Tank,
    Meccanum,
    Swerve,
    Other
}

impl Into<i32> for Drivetrain {
    fn into(self) -> i32 {
        match self {
            Drivetrain::Tank => 0,
            Drivetrain::Meccanum => 1,
            Drivetrain::Swerve => 2,
            Drivetrain::Other => 3,
        }
    } 
}

impl From<&str> for Drivetrain {
    fn from(rhs: &str) -> Self {
        match rhs {
            "tank" => Self::Tank,
            "meccanum" => Self::Meccanum,
            "swerve" => Self::Swerve,
            _ => Self::Other
        }
    }
}



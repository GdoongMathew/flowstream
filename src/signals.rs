use uuid::Uuid;

pub struct Signal {
    pub id: Uuid,
    pub result: Result<Box<dyn Send + Sync>, String>,
}


impl Signal {
    pub fn new(result: Result<Box<dyn Send + Sync>, String>) -> Self {
        Self { id: Uuid::new_v4(), result }
    }

    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.result.is_err()
    }

    pub fn type_check(&self) -> bool {
        match &self.result {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}


#[cfg(test)]
mod signal_tests {
    use super::Signal;

    #[test]
    fn test_signal_create() {
        let signal = Signal::new(Ok(Box::new([1, 2, 3])));
        assert_eq!(signal.id.is_nil(), false);
    }

    #[test]
    fn test_signal_ok() {
        let signal = Signal::new(Ok(Box::new([1, 2, 3])));
        assert!(signal.is_ok());
        assert!(!signal.is_err());
        assert!(signal.type_check());
    }
    
    #[test]
    fn test_signal_err() {
        let signal = Signal::new(Err("Error occurred".to_string()));
        assert!(!signal.is_ok());
        assert!(signal.is_err());
        assert!(!signal.type_check());
    }
}

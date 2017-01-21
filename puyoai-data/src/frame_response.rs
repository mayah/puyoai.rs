use puyoai_core::decision::Decision;

pub struct FrameResponse {
    pub frame_id: i32,
    pub decision: Decision,
    pub message: String,
}

fn escape_message(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == ' ' {
            result.push('_');
        } else if c == '\n' {
            result.push(',');
        } else {
            result.push(c);
        }
    }

    result
}

impl FrameResponse {
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("ID={}", self.frame_id));
        if self.decision.is_valid() {
            result.push_str(&format!(" X={}", self.decision.axis_x()));
            result.push_str(&format!(" R={}", self.decision.rot()));
        }
        if !self.message.is_empty() {
            result.push_str(&format!(" MSG={}", escape_message(&self.message)))
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::FrameResponse;
    use puyoai_core::decision::Decision;

    #[test]
    fn test_to_string() {
        let resp = FrameResponse {
            frame_id: 1,
            decision: Decision::new(3, 1),
            message: "test".to_string(),
        };

        assert_eq!(resp.to_string(), "ID=1 X=3 R=1 MSG=test");
    }
}

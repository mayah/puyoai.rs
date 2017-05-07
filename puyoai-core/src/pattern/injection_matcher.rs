use color::{Color, PuyoColor};

pub struct InjectionMatcher {
    colors: [PuyoColor; 4],
}

impl InjectionMatcher {
    pub fn new() -> InjectionMatcher {
        InjectionMatcher {
            colors: [PuyoColor::EMPTY; 4],
        }
    }

    pub fn match_with_char(&mut self, v: char, c: PuyoColor) -> bool {
        debug_assert!('A' <= v && v <= 'D', "unpexected character: {}", v);
        debug_assert!(c.is_normal_color(), "color is not normal: {}", c);
        let idx = (v as usize) - ('A' as usize);

        if self.colors[idx] == c {
            return true;
        }
        if self.colors[idx] != PuyoColor::EMPTY {
            return false;
        }

        for i in 0..4 {
            if i != idx && self.colors[i] == c {
                // not injective.
                return false;
            }
        }

        self.colors[idx] = c;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_success() {
        let mut bm = InjectionMatcher::new();
        assert!(bm.match_with_char('A', PuyoColor::RED));
        assert!(bm.match_with_char('B', PuyoColor::BLUE));
        assert!(bm.match_with_char('C', PuyoColor::YELLOW));
        assert!(bm.match_with_char('D', PuyoColor::GREEN));
    }

    #[test]
    fn test_match_dup() {
        let mut bm = InjectionMatcher::new();
        assert!(bm.match_with_char('A', PuyoColor::RED));
        assert!(!bm.match_with_char('A', PuyoColor::BLUE));
    }

    #[test]
    fn test_match_not_injective() {
        let mut bm = InjectionMatcher::new();
        assert!(bm.match_with_char('A', PuyoColor::RED));
        assert!(!bm.match_with_char('B', PuyoColor::RED));
    }
}

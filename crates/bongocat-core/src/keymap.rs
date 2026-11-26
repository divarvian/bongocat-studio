use crate::config::PrivacyMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyHand {
    Left,
    Right,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeySpec {
    pub code: u16,
    pub label: &'static str,
    pub hud: &'static str,
    pub width: f32,
    pub hand: KeyHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedKey {
    pub visual_code: u16,
    pub masked: bool,
}

/// Standard 10-finger touch-typing split across 5 QWERTY rows (each row sums to 15.0u).
pub const KEY_ROWS: [&[KeySpec]; 5] = [
    &[
        KeySpec {
            code: 1,
            label: "Esc",
            hud: "Esc",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 2,
            label: "1",
            hud: "1",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 3,
            label: "2",
            hud: "2",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 4,
            label: "3",
            hud: "3",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 5,
            label: "4",
            hud: "4",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 6,
            label: "5",
            hud: "5",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 7,
            label: "6",
            hud: "6",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 8,
            label: "7",
            hud: "7",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 9,
            label: "8",
            hud: "8",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 10,
            label: "9",
            hud: "9",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 11,
            label: "0",
            hud: "0",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 12,
            label: "-",
            hud: "-",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 13,
            label: "=",
            hud: "=",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 14,
            label: "Back",
            hud: "Bksp",
            width: 2.0,
            hand: KeyHand::Right,
        },
    ],
    &[
        KeySpec {
            code: 15,
            label: "Tab",
            hud: "Tab",
            width: 1.5,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 16,
            label: "Q",
            hud: "Q",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 17,
            label: "W",
            hud: "W",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 18,
            label: "E",
            hud: "E",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 19,
            label: "R",
            hud: "R",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 20,
            label: "T",
            hud: "T",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 21,
            label: "Y",
            hud: "Y",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 22,
            label: "U",
            hud: "U",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 23,
            label: "I",
            hud: "I",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 24,
            label: "O",
            hud: "O",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 25,
            label: "P",
            hud: "P",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 26,
            label: "[",
            hud: "[",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 27,
            label: "]",
            hud: "]",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 43,
            label: "\\",
            hud: "\\",
            width: 1.5,
            hand: KeyHand::Right,
        },
    ],
    &[
        KeySpec {
            code: 58,
            label: "Cap",
            hud: "Caps",
            width: 1.75,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 30,
            label: "A",
            hud: "A",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 31,
            label: "S",
            hud: "S",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 32,
            label: "D",
            hud: "D",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 33,
            label: "F",
            hud: "F",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 34,
            label: "G",
            hud: "G",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 35,
            label: "H",
            hud: "H",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 36,
            label: "J",
            hud: "J",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 37,
            label: "K",
            hud: "K",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 38,
            label: "L",
            hud: "L",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 39,
            label: ";",
            hud: ";",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 40,
            label: "'",
            hud: "'",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 28,
            label: "Enter",
            hud: "Enter",
            width: 2.25,
            hand: KeyHand::Right,
        },
    ],
    &[
        KeySpec {
            code: 42,
            label: "Shift",
            hud: "Shift",
            width: 2.25,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 44,
            label: "Z",
            hud: "Z",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 45,
            label: "X",
            hud: "X",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 46,
            label: "C",
            hud: "C",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 47,
            label: "V",
            hud: "V",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 48,
            label: "B",
            hud: "B",
            width: 1.0,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 49,
            label: "N",
            hud: "N",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 50,
            label: "M",
            hud: "M",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 51,
            label: ",",
            hud: ",",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 52,
            label: ".",
            hud: ".",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 53,
            label: "/",
            hud: "/",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 103,
            label: "↑",
            hud: "↑",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 54,
            label: "Shift",
            hud: "R-Shift",
            width: 1.75,
            hand: KeyHand::Right,
        },
    ],
    &[
        KeySpec {
            code: 29,
            label: "Ctrl",
            hud: "Ctrl",
            width: 1.5,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 125,
            label: "⌘",
            hud: "Super",
            width: 1.25,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 56,
            label: "Alt",
            hud: "Alt",
            width: 1.25,
            hand: KeyHand::Left,
        },
        KeySpec {
            code: 57,
            label: "Space",
            hud: "Space",
            width: 6.0,
            hand: KeyHand::Space,
        },
        KeySpec {
            code: 100,
            label: "Alt",
            hud: "R-Alt",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 105,
            label: "←",
            hud: "←",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 108,
            label: "↓",
            hud: "↓",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 106,
            label: "→",
            hud: "→",
            width: 1.0,
            hand: KeyHand::Right,
        },
        KeySpec {
            code: 97,
            label: "Ctrl",
            hud: "R-Ctrl",
            width: 1.0,
            hand: KeyHand::Right,
        },
    ],
];

pub const fn alias_key(code: u16) -> u16 {
    match code {
        41 => 1,    // Grave -> Esc
        111 => 14,  // Delete -> Backspace
        126 => 125, // Right Meta -> Super
        96 => 28,   // KP Enter -> Enter
        82 => 11,   // KP 0 -> 0
        79 => 2,    // KP 1 -> 1
        80 => 3,    // KP 2 -> 2
        81 => 4,    // KP 3 -> 3
        75 => 5,    // KP 4 -> 4
        76 => 6,    // KP 5 -> 5
        77 => 7,    // KP 6 -> 6
        71 => 8,    // KP 7 -> 7
        72 => 9,    // KP 8 -> 8
        73 => 10,   // KP 9 -> 9
        other => other,
    }
}

pub const fn is_gaming_key(code: u16) -> bool {
    matches!(
        code,
        1 | 2
            | 3
            | 4
            | 5
            | 6
            | 15
            | 16
            | 17
            | 18
            | 19
            | 30
            | 31
            | 32
            | 33
            | 42
            | 54
            | 29
            | 97
            | 56
            | 100
            | 57
            | 103
            | 105
            | 106
            | 108
    )
}

pub fn key_spec(code: u16) -> Option<&'static KeySpec> {
    for row in KEY_ROWS {
        for key in row {
            if key.code == code {
                return Some(key);
            }
        }
    }
    None
}

pub fn zone_bucket_for_code(code: u16) -> u16 {
    const LEFT_ZONES: [u16; 3] = [17, 31, 45]; // W, S, X
    const RIGHT_ZONES: [u16; 3] = [23, 37, 50]; // I, K, M
    let is_left = key_spec(code).map_or(code.is_multiple_of(2), |k| k.hand == KeyHand::Left);
    let pool = if is_left { LEFT_ZONES } else { RIGHT_ZONES };
    pool[(code as usize) % pool.len()]
}

/// Applies privacy masking before a key event is displayed or transmitted to OBS.
pub fn resolve_key(raw_code: u16, privacy: PrivacyMode) -> ResolvedKey {
    let mapped = alias_key(raw_code);
    match privacy {
        PrivacyMode::Zones => {
            let visual = if mapped == 57 {
                57
            } else {
                zone_bucket_for_code(mapped)
            };
            ResolvedKey {
                visual_code: visual,
                masked: true,
            }
        }
        PrivacyMode::Gaming => {
            if is_gaming_key(mapped) {
                ResolvedKey {
                    visual_code: mapped,
                    masked: false,
                }
            } else {
                ResolvedKey {
                    visual_code: zone_bucket_for_code(mapped),
                    masked: true,
                }
            }
        }
        PrivacyMode::Full => {
            if key_spec(mapped).is_some() {
                ResolvedKey {
                    visual_code: mapped,
                    masked: false,
                }
            } else {
                ResolvedKey {
                    visual_code: zone_bucket_for_code(mapped),
                    masked: true,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_row_sums_to_15_units() {
        for (idx, row) in KEY_ROWS.iter().enumerate() {
            let total: f32 = row.iter().map(|k| k.width).sum();
            assert!((total - 15.0).abs() < 0.001, "row {idx} summed to {total}");
        }
    }

    #[test]
    fn total_keys_is_63() {
        let count: usize = KEY_ROWS.iter().map(|r| r.len()).sum();
        assert_eq!(count, 63);
    }

    #[test]
    fn privacy_modes_mask_sensitive_keys() {
        // 'P' (code 25) is hidden in Gaming and Zones, visible in Full.
        assert!(!resolve_key(25, PrivacyMode::Full).masked);
        assert!(resolve_key(25, PrivacyMode::Gaming).masked);
        assert!(resolve_key(25, PrivacyMode::Zones).masked);

        // 'W' (code 17) is visible in Full & Gaming, masked in Zones.
        assert_eq!(
            resolve_key(17, PrivacyMode::Gaming),
            ResolvedKey {
                visual_code: 17,
                masked: false
            }
        );
        assert!(resolve_key(17, PrivacyMode::Zones).masked);
    }

    #[test]
    fn ten_finger_hand_split_is_ergonomic() {
        assert_eq!(key_spec(30).unwrap().hand, KeyHand::Left); // A
        assert_eq!(key_spec(33).unwrap().hand, KeyHand::Left); // F
        assert_eq!(key_spec(36).unwrap().hand, KeyHand::Right); // J
        assert_eq!(key_spec(28).unwrap().hand, KeyHand::Right); // Enter
        assert_eq!(key_spec(57).unwrap().hand, KeyHand::Space); // Space
    }
}
